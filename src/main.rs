use std::sync::Arc;

use kube::{
    api::ListParams, client::Client, runtime::controller::Action, runtime::Controller, Api,
};
use tokio::time::Duration;
use thiserror::Error;
use futures::StreamExt;

use mlflow_operator::objects::model_deployment;

#[tokio::main]
async fn main() {
    let k8s_client: Client = Client::try_default()
        .await
        .expect("Expected a valid KUBECONFIG environment variable.");

    let crd_api: Api<model_deployment::ModelDeployment> = Api::all(k8s_client.clone());
    let context: Arc<ContextData> = Arc::new(ContextData::new(k8s_client.clone()));

    Controller::new(crd_api.clone(), ListParams::default())
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
    Ok(Action::requeue(Duration::from_secs(10)))
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
}
