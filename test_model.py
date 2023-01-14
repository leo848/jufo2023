import numpy as np
import sys

import tensorflow as tf
from tensorflow.keras import models

filename = sys.argv[1]

model = models.load_model(filename)
model.summary()

test_x, test_y = np.load("npy_files/20M_neural_input/15.npy"), np.load("npy_files/20M_neural_output/15.npy")

BATCH_SIZE = 20000

def test_generator():
    for i in range(0, len(test_y), BATCH_SIZE):
        yield test_x[i:i+BATCH_SIZE], tf.keras.utils.to_categorical(test_y[i:i+BATCH_SIZE], num_classes=4096)

loss, accuracy = model.evaluate(test_generator(), batch_size=BATCH_SIZE, verbose=True, steps=len(test_y)//BATCH_SIZE)

print(f"Accuracy: {accuracy * 100.0:.2f}%")
