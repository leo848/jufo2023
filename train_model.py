import gc
import os

import numpy as np
import psutil

import matplotlib.pyplot as plt
import tensorflow as tf
# from filenames import MODEL_INPUT, MODEL_NAME, MODEL_OUTPUT
from tensorflow.keras import layers, models

print("TensorFlow version: ", tf.__version__)

MODEL_NAME = "models/15Mtrain-1024-4layers-mae.h5"

GRAPH_NAME = f"figures/{MODEL_NAME.split('/')[-1].split('.')[0]}-graph.png"

if os.path.isfile(MODEL_NAME):
    print("Model already exists. Exiting.")
    exit(1)

MODEL_INPUT = "npy_files/20M_neural_input/{file}"
MODEL_OUTPUT = "npy_files/20M_neural_output/{file}"

TENSORBOARD = True

TOTAL_DATA_SIZE = 20_000_000
AMOUNT_OF_FILES = 40

DATA_PER_FILE = TOTAL_DATA_SIZE // AMOUNT_OF_FILES

assert TOTAL_DATA_SIZE % AMOUNT_OF_FILES == 0

TRAINING_FILES = [ f"{i}.npy" for i in range(0, AMOUNT_OF_FILES) if (i + 7) % 10 != 0 ]
VALIDATION_FILES = [ f"{i}.npy" for i in range(0, AMOUNT_OF_FILES) if (i + 7) % 10 == 0 ]

assert len(TRAINING_FILES) + len(VALIDATION_FILES) == AMOUNT_OF_FILES
assert len(set(TRAINING_FILES) & set(VALIDATION_FILES)) == 0

BATCH_SIZE = 2048
EPOCHS = 128

TRAINING_STEPS = len(TRAINING_FILES) * DATA_PER_FILE // BATCH_SIZE // EPOCHS
VALIDATION_STEPS = len(VALIDATION_FILES) * DATA_PER_FILE // BATCH_SIZE // EPOCHS

print(TRAINING_STEPS, VALIDATION_STEPS)

callbacks = [
    tf.keras.callbacks.ModelCheckpoint(
        filepath=MODEL_NAME,
        save_best_only=False,
        monitor="val_accuracy",
        verbose=1,
    ),
]

if TENSORBOARD:
    callbacks.append(
        tf.keras.callbacks.TensorBoard(
            log_dir="tensorboard_logs",
            histogram_freq=1,
            embeddings_freq=1,
        )
    )

def generator_generator(files: list[str]):
    def generator():
        # for i in range(start, end):
        for file in files:
            x = np.load(MODEL_INPUT.format(file=file))
            y = np.load(MODEL_OUTPUT.format(file=file))
            for i in range(0, x.shape[0], BATCH_SIZE):
                x_batch, y_batch = x[i:i+BATCH_SIZE], y[i:i+BATCH_SIZE]
                y_batch = tf.keras.utils.to_categorical(y_batch, num_classes=4096)

                yield x_batch, y_batch

            del x, y
            gc.collect()

    return generator

training_generator = generator_generator(TRAINING_FILES)
validation_generator = generator_generator(VALIDATION_FILES)

model = models.Sequential()
model.add(layers.Dense(512, activation='relu', input_shape=(1 + (1+2*6) * 64,)))
model.add(layers.Dense(512, activation='relu'))
model.add(layers.Dense(512, activation='relu'))
model.add(layers.Dense(512, activation='relu'))
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
    callbacks=callbacks,
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
# Save the plot
plt.savefig(GRAPH_NAME)

print("Done")
