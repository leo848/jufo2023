import os.path

import numpy as np

import chess
import tensorflow as tf
from abstraction import board_to_move
from board_code import board_to_input, move_to_output, print_neural_output
from filenames import MODEL_NAME
from tensorflow.keras import layers, models

if not os.path.isfile(MODEL_NAME):
    exit(1)

# Load the model
model = models.load_model(MODEL_NAME)

def visualize_output(model, board):
    ninput = board_to_input(board)
    noutput = model.predict(np.array([ninput]))
    print_neural_output(noutput[0])

def game_loop():
    print("\n"*50)
    board = chess.Board()
    while not board.is_game_over():
        print(board.unicode().replace("⭘", " "))
        print("\n"*3)
        move = input("Move: ")
        if not move.strip():
            visualize_output(model, board)
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


if __name__ == "__main__":
    game_loop()
