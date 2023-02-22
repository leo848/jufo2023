import gc
import os

import numpy as np
import psutil

import argparse

import matplotlib.pyplot as plt
import tensorflow as tf
# from filenames import MODEL_INPUT, MODEL_NAME, MODEL_OUTPUT
from tensorflow.keras import layers, models

print("TensorFlow version: ", tf.__version__)

parser = argparse.ArgumentParser(
    description="Train a neural model using data created in Rust.",
)
parser.add_argument(
    "--output", type=str, help="The name of the model to be saved.", required=True
)
parser.add_argument(
    "--input", type=str, help="The name of the input directory to be used.", required=True
)
parser.add_argument(
    "--layers", type=int, help="The number of layers in the neural network.", default=4
)
parser.add_argument(
    "--neurons", type=int, help="The number of neurons in each layer.", default=512
)
parser.add_argument(
    "--batch", type=int, help="The batch size to be used."
)
parser.add_argument(
    "--tensorboard", action="store_true", help="Whether to use tensorboard."
)

args = parser.parse_args()

MODEL_NAME = f"models/{args.output}.h5"
GRAPH_NAME = f"figures/{args.output}.png"

if os.path.isfile(MODEL_NAME):
    print("Model already exists. Exiting.")
    exit(1)

MODEL_INPUT = "npy_files/" + args.input + "_input/{file}"
MODEL_OUTPUT = "npy_files/" + args.input + "_output/{file}"

TENSORBOARD = args.tensorboard

AMOUNT_OF_FILES = os.listdir(f"npy_files/{args.input}_input").__len__()
DATA_PER_FILE = len(np.load(MODEL_INPUT.format(file="0.npy")))
TOTAL_DATA_SIZE = AMOUNT_OF_FILES * DATA_PER_FILE

assert TOTAL_DATA_SIZE % AMOUNT_OF_FILES == 0

TRAINING_FILES = [ f"{i}.npy" for i in range(0, AMOUNT_OF_FILES) if (i + 7) % 10 != 0 ]
VALIDATION_FILES = [ f"{i}.npy" for i in range(0, AMOUNT_OF_FILES) if (i + 7) % 10 == 0 ]

assert len(TRAINING_FILES) + len(VALIDATION_FILES) == AMOUNT_OF_FILES
assert len(set(TRAINING_FILES) & set(VALIDATION_FILES)) == 0

BATCH_SIZE = args.batch
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

for i in range(args.layers - 1):
    model.add(layers.Dense(args.neurons, activation='relu'))

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
