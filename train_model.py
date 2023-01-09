import gc
import os

import numpy as np
import psutil

import matplotlib.pyplot as plt
import tensorflow as tf
# from filenames import MODEL_INPUT, MODEL_NAME, MODEL_OUTPUT
from tensorflow.keras import layers, models

print("TensorFlow version: ", tf.__version__)

MODEL_NAME = "models/unique-15Mtrain-256-5layers.h5"

if os.path.isfile(MODEL_NAME):
    print("Model already exists. Exiting.")
    exit(1)

MODEL_INPUT = "npy_files/20M_neural_input/{n}.npy"
MODEL_OUTPUT = "npy_files/20M_neural_output/{n}.npy"


TOTAL_DATA_SIZE = 16_000_000
AMOUNT_OF_FILES = 16
DATA_PER_FILE = TOTAL_DATA_SIZE // AMOUNT_OF_FILES

TRAINING_DATA_SIZE = (0, 15)
VALIDATION_DATA_SIZE = (15, 16)

BATCH_SIZE = 100
EPOCHS = 75

TRAINING_STEPS = (TRAINING_DATA_SIZE[1] - TRAINING_DATA_SIZE[0]) * DATA_PER_FILE // BATCH_SIZE // EPOCHS - 1
VALIDATION_STEPS = (VALIDATION_DATA_SIZE[1] - VALIDATION_DATA_SIZE[0]) * DATA_PER_FILE // BATCH_SIZE // EPOCHS - 1


def generator_generator(start: int, end: int):
    def generator():
        for i in range(start, end):
            x = np.load(MODEL_INPUT.format(n=i))
            y = np.load(MODEL_OUTPUT.format(n=i))
            for i in range(0, x.shape[0], BATCH_SIZE):
                x_batch, y_batch = x[i:i+BATCH_SIZE], y[i:i+BATCH_SIZE]
                y_batch = tf.keras.utils.to_categorical(y_batch, num_classes=4096)

                yield x_batch, y_batch

            del x, y

    return generator

training_generator = generator_generator(*TRAINING_DATA_SIZE)
validation_generator = generator_generator(*VALIDATION_DATA_SIZE)

model = models.Sequential()
model.add(layers.Dense(256, activation='relu', input_shape=(1 + (1+2*6) * 64,)))
model.add(layers.Dense(256, activation='relu'))
model.add(layers.Dense(256, activation='relu'))
model.add(layers.Dense(256, activation='relu'))
model.add(layers.Dense(256, activation='relu'))
model.add(layers.Dense(4096, activation='softmax'))

model.compile(optimizer='adam', loss='categorical_crossentropy', metrics=['accuracy'])

model.summary()

input("Press enter to start training")

# Train the model and display the training results
history = model.fit(
    training_generator(),
    steps_per_epoch=TRAINING_STEPS,
    epochs=EPOCHS,
    validation_data=validation_generator(),
    validation_steps=VALIDATION_STEPS,
)

accuracy = history.history['accuracy']
val_accuracy = history.history['val_accuracy']
epochs = range(1, len(accuracy) + 1)
plt.plot(epochs, accuracy, 'bo', label='Training accuracy')
plt.plot(epochs, val_accuracy, 'b', label='Validation accuracy')
plt.title('Training and validation accuracy')
plt.xlabel('Epochs')
plt.ylabel('Accuracy')
plt.legend()
plt.show()

# Save the model
if input("Save model? [y/N] ").lower() == "y":
    model.save(MODEL_NAME)
    print("Model saved.")
