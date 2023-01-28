#! /bin/bash

# prerequsites: kubectl, helm and k3d

mkdir ~/.kube


sudo k3d registry \
    create mlflow-registry.localhost \
    --port 12000


sudo k3d cluster \
    create mlflow \
    --registry-use k3d-mlflow-registry.localhost:12000 \
    --agents 1 \
    -p "32000-32010:32000-32010@server:0"


HOME=$HOME sudo k3d kubeconfig get mlflow > $HOME/.kube/config
