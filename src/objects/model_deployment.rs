use std::collections::BTreeMap;

use k8s_openapi::api::apps::v1 as apps_v1;
use k8s_openapi::api::core::v1 as core_v1;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use kube::api::PostParams;
use kube::{Api, Client, CustomResource, Error};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    namespace: &str,
    model_deployment: ModelDeployment,
) -> Result<apps_v1::Deployment, Error> {
    let deployment_api: Api<apps_v1::Deployment> = Api::namespaced(client, namespace);
    let deployment: apps_v1::Deployment = create_model_deployment_deployment(model_deployment);
    deployment_api
        .create(&PostParams::default(), &deployment)
        .await
}

fn create_model_deployment_deployment(model_deployment: ModelDeployment) -> apps_v1::Deployment {

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
                        image: Some(String::from("localhost:32000/mlflow:latest")),
                        args: Some(vec![
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
