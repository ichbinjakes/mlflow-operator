use std::sync::Arc;
use std::env;

use kube::{
    api::ListParams, client::Client, runtime::controller::Action, runtime::Controller, Api,
};

use k8s_openapi::api::apps::v1 as apps_v1;
use tokio::time::Duration;
use thiserror::Error;
use futures::StreamExt;

use mlflow_operator::objects::model_deployment;

#[tokio::main]
async fn main() {
    let k8s_client: Client = Client::try_default()
        .await
        .expect("Expected a valid KUBECONFIG environment variable.");

    let context: Arc<ContextData> = Arc::new(ContextData::new(k8s_client.clone()));

    let namespace: String = match env::var("MY_POD_NAMESPACE") {
        Ok(env_var) => env_var,
        Err(_) => panic!("MY_POD_NAMESPACE environment variable was not set.")
    };

    let model_deployments = Api::<model_deployment::ModelDeployment>::namespaced(k8s_client.clone(), &namespace);
    let k8s_deployments = Api::<apps_v1::Deployment>::namespaced(k8s_client.clone(), &namespace);

    Controller::new(model_deployments, ListParams::default())
        .owns(k8s_deployments, ListParams::default())
        .run(reconcile, on_error, context)
        .for_each(|result| async move {
            match result {
                Ok(o) => println!("reconciled {:?}", o),
                Err(e) => println!("reconcile failed {:?}", e),
            }
        })
        .await;
}

struct ContextData {
    client: Client,
    // You would inject in memory state here e.g. locks to prevent something ?
}

impl ContextData {
    fn new(client: Client) -> Self {
        ContextData { client }
    }
}

async fn reconcile(
    deployment: Arc<model_deployment::ModelDeployment>,
    context: Arc<ContextData>,
) -> Result<Action, Error> {
    println!("Reconciling: {:?}", deployment);
    let action_type: CRDAction = determine_action(&deployment);
    match action_type {
        CRDAction::Create => {
            println!("Creating Model Deployment...");
            match model_deployment::apply_model_deployment(
                context.client.clone(), (*deployment).clone()
            ).await {
                Ok(_) => println!("SUCCESS"),
                Err(e) => println!("{:?}", e)
            };
        },
        CRDAction::Delete => println!("DELETE"),
        CRDAction::Update => println!("UPDATE"),
        CRDAction::NoOp => println!("NO-OP"),
    }
    Ok(Action::requeue(Duration::from_secs(60)))
}

fn determine_action(deployment: &model_deployment::ModelDeployment) -> CRDAction {
    match deployment.metadata.deletion_timestamp.clone() {
        Some(_) => CRDAction::Create,
        None => {
            match &deployment.metadata.labels {
                Some(val) => {
                    if !val.contains_key("mlflow-operator-uid") {
                        return CRDAction::Create
                    }
                },
                None => {
                    return CRDAction::Create
                }
            }
            CRDAction::NoOp
        }
    }
}

fn on_error(
    crd_object: Arc<model_deployment::ModelDeployment>,
    error: &Error,
    context: Arc<ContextData>,
) -> Action {
    eprintln!("Reconciliation error:\n{:?}", crd_object);
    Action::requeue(Duration::from_secs(10))
}

#[derive(Debug, Error)]
enum Error {}

pub enum CRDAction {
    Create,
    Update,
    Delete,
    NoOp,
}
