import gc
import os
import sys

import numpy as np
import psutil

import matplotlib.pyplot as plt
import tensorflow as tf
from filenames import MODEL_INPUT, MODEL_NAME, MODEL_OUTPUT
from tensorflow.keras import layers, models

print("TensorFlow version: ", tf.__version__)

if os.path.isfile(MODEL_NAME):
    print("Model already exists. Exiting.")
    exit(1)

# # Load the data
# x, y = np.load(MODEL_INPUT, mmap_mode='r'), np.load(MODEL_OUTPUT, mmap_mode='r')
# # y encodes the move as a number from 0 to 4095. We need to convert it to a numpy array of 4096 floats.

# x_train, x_test = x[0:90_000], x[90_000:100_000]
# y_train, y_test = y[0:90_000], y[90_000:100_000]


# print("Converting y to one-hot encoding...", end="", flush=True)
# y_train = tf.keras.utils.to_categorical(y_train, num_classes=4096)
# y_test = tf.keras.utils.to_categorical(y_test, num_classes=4096)
# print(" Done.")

# del x, y

# # Execute garbage collection to free up memory
# gc.collect()

TOTAL_DATA_SIZE = 3_000_000
TRAINING_DATA_SIZE = (0, 2_900_000) # _000)
VALIDATION_DATA_SIZE = (2_900_000, 3_000_000) #_000, 3_000_000)

BATCH_SIZE = 128
EPOCHS = 10

TRAINING_STEPS = (TRAINING_DATA_SIZE[1] - TRAINING_DATA_SIZE[0]) // BATCH_SIZE // EPOCHS
VALIDATION_STEPS = (VALIDATION_DATA_SIZE[1] - VALIDATION_DATA_SIZE[0]) // BATCH_SIZE // EPOCHS

def generator_generator(start: int, end: int):
    def generator():
        x, y = np.load(MODEL_INPUT, mmap_mode='r'), np.load(MODEL_OUTPUT, mmap_mode='r')
        for i in range(start, end, BATCH_SIZE):
            x_batch, y_batch = x[i:i+BATCH_SIZE], y[i:i+BATCH_SIZE]
            y_batch = tf.keras.utils.to_categorical(y_batch, num_classes=4096)
            yield x_batch, y_batch
            if i % (BATCH_SIZE * 100) and psutil.virtual_memory().percent > 80:
                del x, y
                print(f"Cleaned up {gc.collect()} objects.")
                x, y = np.load(MODEL_INPUT, mmap_mode='r'), np.load(MODEL_OUTPUT, mmap_mode='r')
    return generator

training_generator = generator_generator(*TRAINING_DATA_SIZE)
validation_generator = generator_generator(*VALIDATION_DATA_SIZE)

model = models.Sequential()
model.add(layers.Dense(256, activation='relu', input_shape=(1 + (1+2*6) * 64,)))
model.add(layers.Dense(256, activation='relu'))
model.add(layers.Dense(256, activation='relu'))
model.add(layers.Dense(256, activation='relu'))
model.add(layers.Dense(4096, activation='softmax'))

model.compile(optimizer='adam', loss='categorical_crossentropy', metrics=['accuracy'])

model.summary()

input("Press enter to start training")

# Train the model and display the training results
history = model.fit_generator(
    training_generator(),
    steps_per_epoch=TRAINING_STEPS,
    epochs=EPOCHS,
    validation_data=validation_generator(),
    validation_steps=VALIDATION_STEPS,
)

print(history.history)
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
