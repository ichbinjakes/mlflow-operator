# Build Docker image
docker_build(
    'k3d-mlflow-registry.localhost:12000/mlflow-operator/mlflow-operator:dev',
    context='.',
    dockerfile='./dev/dev.Dockerfile',
)

# Generate helm yaml and apply
yaml = helm(
    'charts/mlflow-operator',
    name='mlflow-operator',
    namespace='mlflow',
    values=['./dev/kubernetes/operator.yaml'],
)
k8s_yaml(yaml)
