import tensorflow as tf
import numpy as np

from board_code import board_to_input, output_to_move

def board_to_move(model, board):
    """
    Converts a chess.Board to a chess.Move using a neural network model.
    """
    ninput = board_to_input(board)
    noutput = model.predict(np.array([ninput]))[0]
    return output_to_move(noutput)
