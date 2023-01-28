build:
		docker build \
			-t k3d-mlflow-registry.localhost:12000/mlflow-operator/mlflow-operator:latest \
			-f Dockerfile \
			.
		docker push k3d-mlflow-registry.localhost:12000/mlflow-operator/mlflow-operator:latest


build-dev:
		docker build \
			-t k3d-mlflow-registry.localhost:12000/mlflow-operator/mlflow-operator:dev \
			-f dev/operator.dev.Dockerfile \
			.
		docker push k3d-mlflow-registry.localhost:12000/mlflow-operator/mlflow-operator:dev


push-dev:
		docker push mlflow-operator/mlflow-operator:dev


build-mlflow:
		# `grep mlflow dev/kubernetes/mlflow/requirements.txt | tr -d mlflow==`
		docker build \
			-t k3d-mlflow-registry.localhost:12000/mlflow-operator/mlflow:latest \
			-f dev/kubernetes/mlflow/Dockerfile \
			dev/kubernetes/mlflow
		docker push k3d-mlflow-registry.localhost:12000/mlflow-operator/mlflow:latest


create-model-env:
		conda env update -f dev/models/identity-model/conda-env.yaml
