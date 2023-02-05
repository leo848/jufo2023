from tensorflow.keras import layers, models
import tensorflow as tf
import numpy as np
import gc
import os.path
from matplotlib import pyplot as plt

MODEL_NAME = "models/eval-20Mtrain-1024neurons-4layers-mae.h5"
GRAPH_NAME = f"figures/{MODEL_NAME.split('/')[-1].split('.')[0]}-graph.png"

if os.path.isfile(MODEL_NAME):
    print("Model already exists. Exiting.")
    exit(1)

MODEL_INPUT = "npy_files/20M_evaluations_input/{file}"
MODEL_OUTPUT = "npy_files/20M_evaluations_output/{file}"

TENSORBOARD = True

TOTAL_DATA_SIZE = 20_000_000
DATA_PER_FILE = 500_000

AMOUNT_OF_FILES = TOTAL_DATA_SIZE // DATA_PER_FILE

assert TOTAL_DATA_SIZE % AMOUNT_OF_FILES == 0

TRAINING_FILES = [ f"{i}.npy" for i in range(0, AMOUNT_OF_FILES) if (i + 7) % 10 != 0 ]
VALIDATION_FILES = [ f"{i}.npy" for i in range(0, AMOUNT_OF_FILES) if (i + 7) % 10 == 0 ]

assert len(TRAINING_FILES) + len(VALIDATION_FILES) == AMOUNT_OF_FILES
assert len(set(TRAINING_FILES) & set(VALIDATION_FILES)) == 0

BATCH_SIZE = 1024
EPOCHS = 64

TRAINING_STEPS = len(TRAINING_FILES) * DATA_PER_FILE // BATCH_SIZE // EPOCHS
VALIDATION_STEPS = len(VALIDATION_FILES) * DATA_PER_FILE // BATCH_SIZE // EPOCHS

print(TRAINING_STEPS, VALIDATION_STEPS)

callbacks = [
    tf.keras.callbacks.ModelCheckpoint(
        filepath=MODEL_NAME,
        save_best_only=False,
        monitor="val_mae",
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
                yield x_batch, y_batch

            del x, y
            gc.collect()

    return generator

training_generator = generator_generator(TRAINING_FILES)
validation_generator = generator_generator(VALIDATION_FILES)

model = models.Sequential()
model.add(layers.Dense(1024, activation='relu', input_shape=(1 + (1+2*6) * 64,)))
model.add(layers.Dense(1024, activation='relu'))
model.add(layers.Dense(1024, activation='relu'))
model.add(layers.Dense(1024, activation='relu'))
model.add(layers.Dense(1, activation='sigmoid'))

model.compile(optimizer='adam', loss='mae', metrics=['mae'])

model.summary()

history = model.fit(
    training_generator(),
    steps_per_epoch=TRAINING_STEPS,
    epochs=EPOCHS,
    validation_data=validation_generator(),
    validation_steps=VALIDATION_STEPS,
    callbacks=callbacks,
)

mae = history.history['mae']
val_mae = history.history['val_mae']
epochs = range(1, len(mae) + 1)
if len(mae) != len(val_mae):
    print("Warning: len(mae) != len(val_mae)")
    epochs = epochs[:min(len(mae), len(val_mae))]
plt.plot(epochs, mae, 'bo', label='Durchschnittlicher absoluter Fehler / Training')
plt.plot(epochs, val_mae, 'b', label='Durchschnittlicher absoluter Fehler / Validierung')
plt.title('Durchschnittlicher absoluter Fehler')
plt.xlabel('Epochen')
plt.ylabel('MAE')
plt.legend()
# Save the plot
plt.savefig(GRAPH_NAME)

print("Done")
