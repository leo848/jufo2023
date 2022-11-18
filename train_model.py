import tensorflow as tf
import matplotlib.pyplot as plt

from tensorflow.keras import models, layers

import numpy as np

import os.path

import training

from filenames import MODEL_NAME

if os.path.isfile(MODEL_NAME):
    exit(1)

# Load the data
npzfile = np.load("training_data.npz")
x, y = npzfile["arr_0"], npzfile["arr_1"]
x_train, x_test = x[:80_000], x[80_000:]
y_train, y_test = y[:80_000], y[80_000:]

model = models.Sequential()
model.add(layers.Dense(64, activation='relu', input_shape=(1 + (1+2*6) * 64,)))
model.add(layers.Dense(64, activation='relu'))
model.add(layers.Dense(64, activation='relu'))
model.add(layers.Dense(128, activation='sigmoid'))

model.compile(optimizer='adam', loss='binary_crossentropy', metrics=['accuracy'])

# Train the model and display the training results
history = model.fit(x_train, y_train, epochs=10, batch_size=32)
loss = history.history['loss']
epochs = range(1, len(loss) + 1)
plt.plot(epochs, loss, 'bo', label='Training loss')
plt.title('Training loss')
plt.xlabel('Epochs')
plt.ylabel('Loss')
plt.legend()
plt.show()

# Save the model
model.save(MODEL_NAME)
