#! /bin/bash


helm repo add minio https://charts.min.io/
helm repo update


helm upgrade \
  --install \
  --namespace mlflow \
  --create-namespace \
  --values minio-standalone.yaml \
  mlflow-artifacts minio/minio


kubectl -n mlflow \
  create secret \
  generic \
  mlflow-artifacts-credentials \
  --from-literal=AWS_ACCESS_KEY_ID=minio \
  --from-literal=AWS_SECRET_ACCESS_KEY=minio123 \
  --from-literal=MLFLOW_S3_ENDPOINT_URL=http://mlflow-artifacts-minio:9000 \
  --from-literal=MLFLOW_S3_IGNORE_TLS=true


# helm -n mlflow uninstall mlflow-artifacts
