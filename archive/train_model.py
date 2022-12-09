import tensorflow as tf
import matplotlib.pyplot as plt

from tensorflow.keras import models, layers

import numpy as np

import os
import sys
import gc

from filenames import MODEL_NAME, MODEL_INPUT, MODEL_OUTPUT

print("TensorFlow version: ", tf.__version__)

if os.path.isfile(MODEL_NAME):
    print("Model already exists. Exiting.")
    exit(1)

# Load the data
x, y = np.load(MODEL_INPUT, mmap_mode='r'), np.load(MODEL_OUTPUT, mmap_mode='r')
# y encodes the move as a number from 0 to 4095. We need to convert it to a numpy array of 4096 floats.

x_train, x_test = x[0:90_000], x[90_000:100_000]
y_train, y_test = y[0:90_000], y[90_000:100_000]


print("Converting y to one-hot encoding...", end="", flush=True)
y_train = tf.keras.utils.to_categorical(y_train, num_classes=4096)
y_test = tf.keras.utils.to_categorical(y_test, num_classes=4096)
print(" Done.")

del x, y

# Execute garbage collection to free up memory
gc.collect()


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
history = model.fit(x_train, y_train, epochs=5, batch_size=64, validation_data=(x_test, y_test))

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
