FROM python:3.10-slim-bullseye
WORKDIR /mlflow
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
ENTRYPOINT ["mlflow", "server"]
