build:
		docker build -t mlflow-operator/mlflow-operator:latest -f Dockerfile .

build-dev:
		docker build -t localhost:32000/mlflow-operator/mlflow-operator:dev -f dev/Dockerfile .

push-dev:
		docker push localhost:32000/mlflow-operator/mlflow-operator:dev

build-mlflow:
		docker build -t localhost:32000/mlflow:latest -f dev/kubernetes/mlflow/Dockerfile dev/kubernetes/mlflow
		docker push localhost:32000/mlflow:latest

conda-identity-model:
		conda env update -f dev/models/identity-model/conda-env.yaml
