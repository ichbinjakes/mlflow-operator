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

## Development cluster setup

The `dev` directory contains bash scripts and manifests for deploying a local dev cluster using (k3d)[https://k3d.io]. There are also scripts for deploying a `mlflow` tracking server, a standalone `minio` cluster for model storage, and the `mlflow-operator`.

### Prequisites

1. [docker](https://www.docker.com/)
2. [k3d](https://k3d.io)
3. [helm](https://helm.sh/)
4. [kubectl](https://kubernetes.io/docs/tasks/tools/)
5. [tilt](https://tilt.dev/)

### Steps

Depending on your system config, you may need to run these commands with sudo

1. Deploy a k3d cluster. This step will override an existing `~/.kube/config` file.
   - `bash dev/kubernetes/setup.sh`
2. Build and push the mlflow image. This will push to the registry created in step 1.
   - `make build-mlflow`
3. Install minio.
   - `bash dev/kubernetes/minio/apply.sh`
4. Deploy mlflow tracking server.
   - `bash dev/kubernetes/mlflow/apply.sh`

You should now have a local cluster for development purposes. If `kubectl get pods -A` can't connect to the server try running `HOME=$HOME sudo k3d kubeconfig get mlflow > $HOME/.kube/config`

## Development workflow

Development testing requires the operator to be install and a RegisteredModel in the mlflow tracking server.

### Steps:

1. Build the operator image and deploy it run.
   - `tilt up`
2. Log a model and register it.
   - `bash dev/models/log_register.py`
3. Develop
