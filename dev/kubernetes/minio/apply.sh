#! /bin/bash

microk8s helm3 repo add minio https://operator.min.io/
microk8s helm3 repo update

microk8s helm3 upgrade \
  --install \
  --namespace minio-operator \
  --create-namespace \
  --values minio-operator.yaml \
  minio-operator minio/operator

microk8s helm3 upgrade \
  --install \
  --namespace mlflow \
  --create-namespace \
  --values minio-tenant.yaml \
  mlflow-artifacts minio/tenant

microk8s kubectl -n mlflow \
  create secret \
  generic \
  mlflow-artifacts-credentials \
  --from-literal=AWS_ACCESS_KEY_ID=minio \
  --from-literal=AWS_SECRET_ACCESS_KEY=minio123 \
  --from-literal=MLFLOW_S3_ENDPOINT_URL=https://minio \
  --from-literal=MLFLOW_S3_IGNORE_TLS=true

