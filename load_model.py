import os.path
import numpy as np
import tensorflow as tf
from tensorflow.keras import models, layers

import chess

from abstraction import board_to_move

from board_code import board_to_input, move_to_output

from filenames import MODEL_NAME

if not os.path.isfile(MODEL_NAME):
    exit(1)

# Load the model
model = models.load_model(MODEL_NAME)

print("\n"*50)
board = chess.Board()
while not board.is_game_over():
    print(board.unicode().replace("⭘", " "))
    print("\n"*3)
    move = input("Move: ")
    if not move.strip():
        move = board_to_move(model, board)
        if board.is_legal(move):
            print("Computer move:", move)
            board.push(move)
        else:
            print("Computer move:", move, "is illegal")
    else:
        try:
            board.push_uci(move)
        except ValueError:
            print("Illegal move:", move)

