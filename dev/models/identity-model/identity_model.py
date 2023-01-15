import numpy as np
from mlflow.models.signature import ModelSignature
from mlflow.pyfunc import PythonModel
from mlflow.types import Schema, TensorSpec


class IdentityModel(PythonModel):
    def load_context(self, context):
        pass

    def predict(self, context, model_input) -> np.array:
        print(f"Model Input: {model_input}")
        return model_input

    @property
    def signature(self):
        input_schema = Schema(
            [
                TensorSpec(type=np.dtype(int), shape=(-1,), name="feature_1"),
                TensorSpec(type=np.dtype(int), shape=(-1,), name="feature_2"),
            ]
        )
        output_schema = Schema(
            [
                TensorSpec(type=np.dtype(int), shape=(-1,), name="output_1"),
                TensorSpec(type=np.dtype(int), shape=(-1,), name="output_2"),
            ]
        )
        return ModelSignature(inputs=input_schema, outputs=output_schema)
