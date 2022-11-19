from itertools import count, islice
import time

import chess.pgn
import os.path

from board_code import *
from pgn_to_input import *

AMOUNT_OF_GAMES = 100

def db_test():
    file = open(DB_FILE)

    size = os.path.getsize(DB_FILE)

    print(f"File size: {size} bytes")

    games = map(lambda _: chess.pgn.read_game(file), count())
    games = filter(is_valid_game, games)
    games = islice(games, AMOUNT_OF_GAMES)

    for game in games:
        board = chess.Board()
        print(f"{game.headers['White']} vs {game.headers['Black']}")
        time.sleep(2.0)
        for (index, move) in enumerate(game.mainline_moves(), 2):
            time.sleep(0.1)
            print("\n"*50)
            board.push(move)
            print(board.unicode().replace("⭘", " "))
        print()
        print(f"{game.headers['Result']}")
        time.sleep(3.0)
    print()

    file.close()

def neural_conversion_test():
    print("Board: ")
    board = chess.Board()
    print(board)

    print("\nInput: ")
    ninput = board_to_input(board)
    print_neural_input(ninput)

    print("Output: ")
    noutput = move_to_output(chess.Move.from_uci("g1f3"))
    print_neural_output(noutput)
    print(output_to_move(noutput))

def load_games():
    boards, moves = load(1_000)
    print(boards.shape)
    print(moves.shape)
    print(boards[:10])
    print(moves[:10])

if __name__ == "__main__":
    load_games()

