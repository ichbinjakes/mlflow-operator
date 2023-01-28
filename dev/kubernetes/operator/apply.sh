#! /bin/bash


helm upgrade \
  --install \
  --namespace mlflow-operator \
  --create-namespace \
  --values operator.yaml \
  mlflow-operator ../../../charts/mlflow-operator


# helm -n mlflow-operator uninstall mlflow-operator
