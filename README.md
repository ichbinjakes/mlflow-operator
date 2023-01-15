# mlflow-operator
This is a rust based operator for doing mlflow things on kubernetes.

## TODO:
- Implement create for model deployment
- Implement delete for model deployment
- tilt.dev file
- pre commit hooks

## Roadmap:
1. initial release
2. Implement update/patch for model deployment
3. Implement adding resources, env vars to model deployment
4. Implement adding tolerations, security context etc. to model deployment
5. Add support for deploying to seldon and/or kserve
6. Add logging to deployments (probably custom rust proxy)
7. Add monitoring to deployments
8. Migrate to k3d/kind/something better than microk8s?

#### Further ahead:
1. Deploy tracking servers
2. Build models into images