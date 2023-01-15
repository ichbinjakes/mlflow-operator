#! /bin/bash

microk8s helm upgrade --install \
  --namespace mlflow \
  mlflow-operator \
  ./charts/mlflow-operator \
  --set image.repository=localhost:32000/mlflow-operator/mlflow-operator \
  --set image.tag=dev
