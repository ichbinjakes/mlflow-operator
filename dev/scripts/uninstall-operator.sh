#! /bin/bash

microk8s helm uninstall \
  --namespace mlflow \
  mlflow-operator

microk8s kubectl delete crd modeldeployments.mlflow.org