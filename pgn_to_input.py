import numpy as np
import chess.pgn
import random
import time

from itertools import count, islice
from more_itertools import flatten, unique_everseen

from board_code import board_to_input, move_to_output, move_to_complete_output

from filenames import PGN_DATABASE, NUMPY_FILE


def is_valid_game(game) -> bool:
    try:
        elo_enough = (
            int(game.headers["WhiteElo"]) > 1700
            and int(game.headers["BlackElo"]) > 1700
        )
        time_control = int(game.headers["TimeControl"].split("+")[0]) > 500
        return elo_enough and time_control
    except ValueError:
        return False


def game_to_boards(game):
    boards = []
    board = chess.Board()
    for move in game.mainline_moves():
        boards.append((board.copy(), move))
        board.push(move)
    return random.sample(boards, min(10, len(boards)))


def inspect(iterator):
    def print_ret(v):
        print(v)
        return v

    return map(print_ret, iterator)


def load(n: int) -> np.ndarray:
    start_time = time.time()

    def output_if_significant(iterator):
        for (index, value) in enumerate(iterator, 1):
            if index % 100 == 0:
                current_time = time.time()
                time_per_step = (current_time - start_time) / index
                eta = time_per_step * (n - index)
                formatted_time = time.strftime("%H:%M:%S", time.gmtime(eta))
                formatted_current_time = time.strftime(
                    "%H:%M:%S", time.gmtime(current_time - start_time)
                )
                print(
                    f"Processed {index} / {n} boards in {formatted_current_time}, ETA: {formatted_time} ({time_per_step*1000:.0f} ms per step)",
                    end="\r",
                )
            yield value

    file = open(PGN_DATABASE)
    games = map(lambda _: chess.pgn.read_game(file), count())
    games = filter(is_valid_game, games)
    boards = flatten(map(game_to_boards, games))
    boards = unique_everseen(boards, key=lambda x: x[0].fen())
    boards = islice(boards, n)
    boards = output_if_significant(boards)
    boards, moves = zip(*boards)
    boards = np.array(list(map(board_to_input, boards)))
    # moves = np.array(list(map(move_to_output, moves)))
    moves = np.array(list(map(move_to_complete_output, moves)))

    file.close()

    return boards, moves


if __name__ == "__main__":
    loaded = load(200_000)
    print("\nLoaded, saving...")
    with open(NUMPY_FILE, "wb") as f:
        np.savez_compressed(f, *loaded)
        print(f"Saved to {NUMPY_FILE} (size: {f.tell() / 1024 / 1024:.2f} MB)")
