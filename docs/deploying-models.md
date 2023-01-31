# Model Deployment

Once a tracking server has been setup and a RegisteredModel is in the model registry a model can be deployed using the operator.

## Example Manifest

```yaml
apiVersion: mlflow.org/v1alpha1
kind: ModelDeployment
metadata:
  name: identity-model-1
  namespace: mlflow
spec:
  mlflow:
    tracking_server_url: http://mlflow:5000
    tracking_server_storage_secret: mlflow-artifacts-credentials
  model:
    name: identity-model
    version: 1
```

### Details

The `ModelDeployment` `spec` contains two main parts: `mlflow` tracking server details and `model` details.
