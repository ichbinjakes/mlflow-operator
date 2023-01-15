# Development

## Directory layout

- /charts
  - Helm charts to install the operator and CRDs
- /dev
  - Scripts and files to aid development of the operator
- /doc
  - Documentation about the operator
- /src
  - Source code for the operator and CRDs

### Development cluster

When developing the mlflow-operator I set up a test cluster using microk8s. The following plugins need to be enabled:
- dns
- helm3
- hostpath-storage
- registry

The development cluster will also need:
- minio-operator
- minio-tenant
- mlflow

Setting up minio can be done using `dev/kubernetes/minio/apply.sh`. Access the console with `kubectl -n minio-operator port-forward svc/console 9090:9090`

To setup mlflow:
1. Build the image with `make build-mlflow`, this will also push to microk8s' registry
2. Deploy the server with `kubectl apply -f dev/kubernetes/mlflow/deployment.yaml`


### Test model

One role of mlflow-operator is to deploy models, for that we need a test model. There is a test model in `dev/models/identity-model`. This model outputs the input.

To log the model to mlflow:
1. Create a python env: `make conda-identity-model`
2. Activate the environment: `conda activate identity-model`
3. Log the model:
```
python \
    dev/models/identity-model/log_register.py \
    --tracking-uri=http://localhost:32001 \
    --s3-endpoint=http://localhost:32002
```


### Developing

.... tilt.dev ... blah
