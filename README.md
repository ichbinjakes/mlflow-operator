# mlflow-operator

mlflow-operator is a kubernetes operator to make it simple to deploy RegisteredModels from a mlflow tracking server to kubernetes.


## Implemented features:
- Deploy RegisteredModel from an instance of mlflow's tracking server using a ModelDeployment
- Delete a ModelDeployment


## MVP:
- Create model deployments
- Delete model deployments


## Roadmap:
1. initial release
   - Cleanup code
   - Documentation (installation, usage, development)
   - DockerHub
   - Wait for mlflow fix (https://github.com/mlflow/mlflow/issues/7645)
   - Logging
3. Implement adding resources, env vars to model deployment
4. Implement adding tolerations, security context etc. to model deployment
5. Python plugin for mlflow to deploy from mlflow cli
6. Add support for deploying to seldon and/or kserve
7. Add logging to deployments (probably custom rust proxy)
8. Add monitoring to deployments (customise the deployment? - specify sidecar container)
9. Configure for cluster or namespace scope in helm chart


#### Further ahead:
1. Deploy tracking servers
2. Build models into images
