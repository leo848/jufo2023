import argparse
from os.path import expanduser
import chess
import chess.engine
import time
import numpy as np
import tensorflow as tf
from tensorflow.keras import models
import io

parser = argparse.ArgumentParser(description='Test a model against Stockfish.')

parser.add_argument('--stockfish', type=str, default=expanduser('~/.local/bin/stockfish'))
parser.add_argument('--model', type=str, required=True)

args = parser.parse_args()

model = models.load_model(args.model)
# engine = chess.engine.SimpleEngine.popen_uci(args.stockfish)

# Communicate with Stockfish
# Tell it to play at ELO 800
# engine.configure({"UCI_LimitStrength": True, "UCI_Elo": 1350})
# engine.configure({"Skill Level": 0})

def model_play(boards: list[chess.Board]) -> chess.Move:
    model_input = np.array([board_to_input(board) for board in boards])
    model_output = model.predict(model_input, verbose=0)
    return outputs_to_moves(model_output, boards)

def stockfish_play(board: chess.Board, engine: chess.engine.SimpleEngine) -> chess.Move:
    result = engine.play(board, chess.engine.Limit(time=0.0001))
    return result.move

def random_play(board: chess.Board) -> chess.Move:
    return np.random.choice(list(board.legal_moves))

def board_to_input(board) -> list[float]:
    result = [float(board.turn)]
    for square in chess.SQUARES:
        piece = board.piece_at(square)
        if piece is None:
            result.extend([1.0] + [0.0] * 12)
        else:
            result.append(0.0)
            result.extend(
                [
                    1.0
                    if piece.color == color and piece.piece_type == piece_type
                    else 0.0
                    for color in [chess.WHITE, chess.BLACK]
                    for piece_type in range(1, 7)
                ]
            )
    assert len(result) == (6 * 2 + 1) * 64 + 1
    return result

def outputs_to_moves(noutputs: list[list[float]], boards: list[chess.Board]) -> list[chess.Move]:
    return [ output_to_move(noutput, board) for (noutput, board) in zip(noutputs, boards) ]

def output_to_move(noutput: list[float], board: chess.Board) -> chess.Move:
    # return max(
    #     board.legal_moves,
    #     key=lambda move: noutput[move.from_square * 64 + move.to_square],
    # )

    # Return a random legal move with probability proportional to the output.
    POW = 3.

    legal_moves = list(board.legal_moves)
    probabilities = np.array([ noutput[move.from_square * 64 + move.to_square] ** POW for move in board.legal_moves ])
    one_hot_probs = np.zeros_like(probabilities)
    one_hot_probs[np.argmax(probabilities)] = 1.
    probabilities /= np.sum(probabilities)
    probabilities = 0.8 * one_hot_probs + 0.2 * probabilities
    return np.random.choice(legal_moves, p=probabilities)


    # sorted_output = sorted(enumerate(noutput), key=lambda x: x[1], reverse=True)
    # move = None
    # for (integer, value) in sorted_output:
    #     from_square = chess.Square(integer // 64)
    #     to_square = chess.Square(integer % 64)
    #     move = chess.Move(from_square, to_square)
    #     if move in board.legal_moves:
    #         return move
    # exception_string = f"Position: {board.fen()}, no legal move found (of {len(list(board.legal_moves))})."
    # raise Exception(exception_string)

AMOUNT = 400

def print_results(boards, move_no=None):
    """
    Print the results of the games in a 10x10 grid.
    Each square represents a game.
    It is colored in red if the game is lost, green if the game is won, and gray if the game is drawn.
    It is white when it is still in progress.
    """
    f = io.StringIO()
    print("\n" * 50, end="", file=f)

    done = sum([ 1 for board in boards if board.is_game_over() ])
    print(f"{done}/{AMOUNT}", end="", file=f)
    if move_no is not None:
        print(f" ({move_no})", file=f)
    else:
        print(file=f)

    for i in range(0, 400, 20):
        for j in range(20):
            board = boards[i + j]
            if board.is_game_over():
                if board.result() == "1-0":
                    print("\033[92m", end="", file=f)
                elif board.result() == "0-1":
                    print("\033[91m", end="", file=f)
                else:
                    print("\033[90m", end="", file=f)
            else:
                print("\033[97m", end="", file=f)
            print("██", end="", file=f)
        print("\033[0m", file=f)

    print(f.getvalue(), end="")

def main():
    results = {
        "1-0": 0,
        "0-1": 0,
        "1/2-1/2": 0,
    }

    engine = chess.engine.SimpleEngine.popen_uci(args.stockfish)
    engine.configure({"Skill Level": 0})

    boards = [ chess.Board() for _ in range(AMOUNT) ]
    filtered = boards

    white_to_move = True
    while len(filtered) > 0:
        if white_to_move:
            moves = model_play(filtered)
        else:
            # moves = [ random_play(board) for board in filtered ]
            moves = [ stockfish_play(board, engine) for board in filtered ]
        for (board, move) in zip(filtered, moves):
            board.push(move)
        filtered = [ board for board in filtered if not board.is_game_over() ]

        white_to_move = not white_to_move

        try:
            print_results(boards, move_no=filtered[0].fullmove_number)
        except IndexError:
            print_results(boards)

    for board in boards:
        result = board.result()
        results[result] += 1

    for (result, count) in results.items():
        print(f"{result}: {count}")

    percentage = results["1-0"] / AMOUNT + results["1/2-1/2"] / AMOUNT / 2
    print(f"Percentage: {percentage}")

    engine.quit()

if __name__ == '__main__':
    main()
