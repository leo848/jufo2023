import numpy as np
import gc
import sys

import tensorflow as tf
from tensorflow.keras import models

# filename = sys.argv[1]

# model = models.load_model(filename)
# model.summary()

# test_x, test_y = np.load("npy_files/20M_neural_input/15.npy"), np.load("npy_files/20M_neural_output/15.npy")

# BATCH_SIZE = 20000

# def test_generator():
#     for i in range(0, len(test_y), BATCH_SIZE):
#         yield test_x[i:i+BATCH_SIZE], tf.keras.utils.to_categorical(test_y[i:i+BATCH_SIZE], num_classes=4096)

# loss, accuracy = model.evaluate(test_generator(), batch_size=BATCH_SIZE, verbose=True, steps=len(test_y)//BATCH_SIZE)

# print(f"Accuracy: {accuracy * 100.0:.2f}%")

def data_generator(stage, count):
    path = {
        "openings": "15M_openings_neural",
        "middlegame": "15M_middlegame_neural",
        "endgame": "15M_endgame_neural",
        "default": "20M_neural",
    }[stage]
    test_x, test_y = np.load(f"npy_files/{path}_input/{count}.npy", mmap_mode="r"), np.load(f"npy_files/{path}_output/{count}.npy", mmap_mode="r")
    BATCH = 1024
    for i in range(0, len(test_x), BATCH):
        yield test_x[i:i+BATCH], tf.keras.utils.to_categorical(test_y[i:i+BATCH], num_classes=4096)
        if i % (BATCH * 16) == 0:
            gc.collect()
    gc.collect()


def load_model(stage):
    path = {
        "openings": "15Mopenings-512neurons-4layers",
        "middlegame": "14Mmiddlegame-512neurons-4layers",
        "endgame": "15Mendgame-512neurons-4layers",
        "default": "mates-20Mtrain-512-4layers-2",
    }[stage]
    model = models.load_model(f"models/{path}.h5")
    return model


accuracies = {}

for count in range(2, 16, 4):
    print(f"Testing on {count} data", file=sys.stderr)
    for data in ["openings", "middlegame", "endgame", "default"]:
        for model in ["openings", "middlegame", "endgame", "default"]:
            print(f"Testing {model} on {data} data", file=sys.stderr)
            # Calculate the accuracy of the model on the test data
            loss, accuracy = load_model(model).evaluate(data_generator(data, count), verbose=True)
            accuracies[count, model, data] = accuracy

            print(f"\tAccuracy: {accuracy * 100.0:.2f}%", file=sys.stderr)

            gc.collect()

print("\n\n=== Results ===", file=sys.stderr)

for count, model, data in accuracies:
    print(f"Count {count}: Model {model} on {data} data:\n\t{accuracies[model, data] * 100.0:.3f}%")
