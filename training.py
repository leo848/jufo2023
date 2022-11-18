import numpy as np
import chess.pgn

from itertools import count, islice
from more_itertools import flatten

from board_code import board_to_input, move_to_output

DB_FILE = "database.pgn"

def is_valid_game(game) -> bool:
    try:
        elo_enough = int(game.headers['WhiteElo']) > 1500 and int(game.headers['BlackElo']) > 1500
        time_control = int(game.headers['TimeControl'].split('+')[0]) > 600
        return elo_enough and time_control
    except ValueError:
        return False

def game_to_boards(game):
    board = chess.Board()
    for move in game.mainline_moves():
        yield (board.copy(), move)
        board.push(move)

def inspect(iterator):
    def print_ret(v):
        print(v)
        return v
    return map(print_ret, iterator)

def load(n: int) -> np.ndarray:
    file = open(DB_FILE)
    games = map(lambda _: chess.pgn.read_game(file), count())
    games = filter(is_valid_game, games)
    boards = flatten(map(game_to_boards, games))
    boards = islice(boards, n)
    boards, moves = zip(*boards)
    boards = np.array(list(map(board_to_input, boards)))
    moves = np.array(list(map(move_to_output, moves)))

    file.close()

    return boards, moves

