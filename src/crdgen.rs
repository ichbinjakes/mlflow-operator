use kube::CustomResourceExt;
use mlflow_operator::objects::model_deployment;

fn main() {
    print!(
        "{}",
        serde_yaml::to_string(&model_deployment::ModelDeployment::crd()).unwrap()
    )
}
