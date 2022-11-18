import os.path
import numpy as np
import tensorflow as tf
from tensorflow.keras import models, layers

import chess

from abstraction import board_to_move

from board_code import board_to_input, move_to_output

if not os.path.isfile("model.h5"):
    exit(1)

# Load the model
model = models.load_model("model.h5")

print("\n"*50)
i = "input"
while i.strip() != "":
    i = input("Enter a board: ")
    if i == "exit":
        break
    try:
        board = chess.Board(i)
    except Exception:
        print("Invalid board")
        continue
    print(board.unicode().replace("⭘", " "))
    print(f"Computer plays: {board_to_move(model, board)}")
    print()
