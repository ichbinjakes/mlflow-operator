# mlflow-operator

> :warning: This project is in the early alpha stage

mlflow-operator is a kubernetes operator to make it simple to deploy RegisteredModels from a mlflow tracking server to kubernetes.

## Implemented features

- Deploy RegisteredModel from an instance of mlflow's tracking server using a ModelDeployment
- Delete a ModelDeployment

## MVP

- Create model deployments
- Delete model deployments

## Roadmap

1. initial release
   - Documentation (installation, usage, development)
   - Wait for mlflow [fix](https://github.com/mlflow/mlflow/issues/7645)
   - Wait for mlflow server images
   - Logging
2. Implement adding resources, env vars to model deployment
3. Implement adding tolerations, security context etc. to model deployment
4. Python plugin for mlflow to deploy from mlflow cli
5. Configure for cluster or namespace scope in helm chart

## Further ahead

1. Deploy tracking servers
2. Build models into images
