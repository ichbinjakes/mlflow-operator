import argparse
import os

import mlflow

import identity_model


def log_model(
        tracking_uri: str,
        s3_endpoint: str,
):
    experiment_name = "identity-model"
    os.environ["MLFLOW_S3_ENDPOINT_URL"] = s3_endpoint
    os.environ["MLFLOW_S3_IGNORE_TLS"] = "true"
    mlflow.set_tracking_uri(uri=tracking_uri)
    mlflow.set_experiment(experiment_name=experiment_name)

    model = identity_model.IdentityModel()

    # log model to mlflow
    with mlflow.start_run() as run:
        run_id = run.info.run_id
        mlflow.pyfunc.log_model(
            artifact_path="model",
            python_model=model,
            signature=model.signature,
            code_path=[os.path.join(os.path.dirname(__file__), "identity_model.py")],
            pip_requirements=[]
        )

    # register model in model registry
    if not run_id:
        raise Exception("Run ID not found")

    client = mlflow.client.MlflowClient(tracking_uri=tracking_uri)
    client.create_registered_model("identity-model", description="A model for testing")
    mlflow.register_model(model_uri=f"runs:/{run_id}", name="identity-model")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--tracking-uri")
    parser.add_argument("--s3-endpoint")

    os.environ["AWS_SHARED_CREDENTIALS_FILE"] = os.path.join(os.path.dirname(__file__), "credentials")

    parsed_args = parser.parse_args()
    log_model(**vars(parsed_args))
