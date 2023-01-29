use std::collections::BTreeMap;
use std::env;

use k8s_openapi::api::apps::v1 as apps_v1;
use k8s_openapi::api::core::v1 as core_v1;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use kube::api::{Patch, PatchParams, PostParams};
use kube::{Api, Client, CustomResource, Error, Resource, ResourceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct MlflowConfig {
    pub tracking_server_url: String,
    pub tracking_server_storage_secret: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct ModelConfig {
    pub name: String,
    pub version: u16,
}

#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "mlflow.org",
    version = "v1alpha1",
    kind = "ModelDeployment",
    plural = "modeldeployments",
    singular = "modeldeployment",
    derive = "PartialEq",
    namespaced
)]
pub struct ModelDeploymentSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: Option<Vec<core_v1::LocalObjectReference>>,
    pub mlflow: MlflowConfig,
    pub model: ModelConfig,
}

pub async fn apply_model_deployment(
    client: Client,
    model_deployment: ModelDeployment,
) -> Result<apps_v1::Deployment, Error> {
    
    let namespace = match model_deployment.namespace() {
        Some(namespace) => namespace,
        None => panic!("No namespace on model deployment")
    };

    let name = model_deployment.name_any();

    let deployment_api: Api<apps_v1::Deployment> = Api::namespaced(client, &namespace);

    let deployment: apps_v1::Deployment = create_deployment_spec(model_deployment);
    // deployment = insert_uuid_for_deployment(deployment, Uuid::new_v4()); 
    
    // let new_status = Patch::Apply(json!({
    //     "apiVersion": "kube.rs/v1",
    //     "kind": "Document",
    //     "status": DocumentStatus {
    //         hidden: should_hide,
    //     }
    // }));
    

    // deployment_api
    //     .create(&PostParams::default(), &deployment)
    //     .await

    deployment_api.patch(
        &name,
        &PatchParams::apply("mlflow-operator"),
        &Patch::Apply(&deployment),
    ).await
}

fn insert_uuid_for_deployment(mut deployment: apps_v1::Deployment, uuid: Uuid) -> apps_v1::Deployment {
    let mut labels: BTreeMap<String, String> = deployment.metadata.labels.clone().unwrap();
    labels.insert("mlflow-operator-uuid".to_owned(), uuid.clone().to_string());

    deployment.metadata.labels = Some(labels);
    deployment
}

fn get_mlflow_image_name() -> String {
    match env::var("DEFAULT_MODEL_IMAGE") {
        Ok(env_var) => env_var,
        Err(_) => panic!("`DEFAULT_MODEL_IMAGE` environment variable was not set.")
    }
}

// fn insert_uuid_for_service(uuid: Uuid) {

// }

// fn insert_uuid_for_pod_template(mut pod_template_spec: core_v1::PodTemplateSpec, uuid: Uuid) -> core_v1::PodTemplateSpec {
//     let mut labels: BTreeMap<String, String> = pod_template_spec.metadata.clone().unwrap().labels.clone().unwrap();
//     labels.insert("mlflow-operator-uuid".to_owned(), uuid.clone().to_string());

//     let mut metadata = pod_template_spec.metadata.clone().unwrap();
//     metadata.labels = Some(labels);
//     pod_template_spec.metadata = Some(metadata);
//     pod_template_spec
// }

fn create_deployment_spec(model_deployment: ModelDeployment) -> apps_v1::Deployment {

    let oref = model_deployment.controller_owner_ref(&()).unwrap();

    let name: String = model_deployment.metadata.name.unwrap();

    let mut labels: BTreeMap<String, String> = BTreeMap::new();
    labels.insert("app".to_owned(), name.clone());
    
    let model_uri: String = format!(
        "models:/{}/{}",
        model_deployment.spec.model.name, model_deployment.spec.model.version
    );

    let deployment: apps_v1::Deployment = apps_v1::Deployment {
        metadata: ObjectMeta {
            name: Some(name),
            namespace: Some(model_deployment.metadata.namespace.unwrap()),
            labels: Some(labels.clone()),
            owner_references: Some(vec![oref]),
            ..ObjectMeta::default()
        },
        spec: Some(apps_v1::DeploymentSpec {
            replicas: Some(1),
            selector: LabelSelector {
                match_expressions: None,
                match_labels: Some(labels.clone()),
            },
            template: core_v1::PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels),
                    ..ObjectMeta::default()
                }),
                spec: Some(core_v1::PodSpec {
                    // TODO: service account, security, init_containers, volumes
                    image_pull_secrets: model_deployment.spec.image_pull_secrets,
                    containers: vec![core_v1::Container {
                        // TODO: volume_mounts, image_pull_policy, readiness_probe,
                        // TODO: startup_probe, security_context
                        name: String::from("model-server"),
                        image: Some(get_mlflow_image_name()),
                        command: Some(vec![
                            "mlflow".to_string(),
                            "models".to_string(),
                            "serve".to_string(),
                            "--enable-mlserver".to_string(),
                            "--host".to_string(),
                            "0.0.0.0".to_string(),
                            "--port".to_string(),
                            "8080".to_string(),
                            "--model-uri".to_string(),
                            String::from(&model_uri),
                        ]),
                        env: Some(vec![core_v1::EnvVar {
                            name: String::from("MLFLOW_TRACKING_URI"),
                            value: Some(model_deployment.spec.mlflow.tracking_server_url),
                            value_from: None,
                        }]),
                        env_from: Some(vec![core_v1::EnvFromSource {
                            secret_ref: Some(core_v1::SecretEnvSource {
                                name: Some(
                                    model_deployment.spec.mlflow.tracking_server_storage_secret,
                                ),
                                optional: Some(false),
                            }),
                            ..core_v1::EnvFromSource::default()
                        }]),
                        ports: Some(vec![core_v1::ContainerPort {
                            container_port: 8080,
                            ..core_v1::ContainerPort::default()
                        }]),
                        ..core_v1::Container::default()
                    }],
                    ..core_v1::PodSpec::default()
                }),
                ..core_v1::PodTemplateSpec::default()
            },
            ..apps_v1::DeploymentSpec::default()
        }),
        ..apps_v1::Deployment::default()
    };
    deployment
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_model_deployment() {
        let model_deployment: ModelDeployment = ModelDeployment {
            metadata: ObjectMeta { 
                name: Some(String::from("test-deployment")), 
                namespace: Some(String::from("test-namespace")),
                ..ObjectMeta::default()
            },
            spec: ModelDeploymentSpec { 
                image_pull_secrets: None, 
                mlflow: MlflowConfig { 
                    tracking_server_url: String::from("http://test-mlflow"), 
                    tracking_server_storage_secret: String::from("mlflow-storage-credentials") }, 
                model: ModelConfig { 
                    name: String::from("test-model"), version: 1 
                } 
            }
        };
        
        let document: &str = "
        apiVersion: apps/v1
        kind: Deployment
        metadata:
          name: test-deployment
          namespace: test-namespace
          labels:
            app: test-deployment
        spec:
          replicas: 1
          selector:
            matchLabels:
              app: test-deployment
          template:
            metadata:
              labels:
                app: test-deployment
            spec:
              containers:
              - name: model-server
                args:
                - mlflow
                - models
                - serve
                - --enable-mlserver
                - --host
                - 0.0.0.0
                - --port
                - 8080
                - --model-uri
                - models:/test-model/1
                env:
                  - name: MLFLOW_TRACKING_URI
                    value: http://test-mlflow
                envFrom:
                  - secretRef:
                      name: mlflow-storage-credentials
                      optional: false
                image: localhost:32000/mlflow:latest
                ports:
                - containerPort: 8080";

        let kubernetes_deployment: apps_v1::Deployment = create_deployment_spec(model_deployment);
        let check_deployment: apps_v1::Deployment = match serde_yaml::from_str(document) {
            Ok(result) => result,
            Err(_) => panic!()
        };

        println!("{:?\n\n}", kubernetes_deployment);
        println!("{:?\n\n}", check_deployment);
        
        assert_eq!(kubernetes_deployment, check_deployment);


    }
}
