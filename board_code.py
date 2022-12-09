from enum import Enum, auto
from typing import Tuple

import numpy as np

import chess.pgn


def board_to_input(board) -> np.ndarray:
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
    return np.array(result)


def move_to_output(move) -> np.ndarray:
    """
    Converts a chess.Move to a list of floats.
    The first 64 numbers encode the 'from' field, the last 64 the 'to' field.
    The first number is the 'a1' square, the last is the 'h8' square.
    """
    result = np.zeros(128)
    result[move.from_square] = 1.0
    result[move.to_square + 64] = 1.0
    return result

def move_to_complete_output(move) -> int:
    return move.from_square * 64 + move.to_square


def output_to_move(noutput: list[float]) -> chess.Move:
    from_square = max(enumerate(noutput[:64]), key=lambda p: p[1])[0]
    to_square = max(enumerate(noutput[64:]), key=lambda p: p[1])[0]
    return chess.Move(from_square, to_square)

def color(string: str, rgb: Tuple[int, int, int]) -> str:
    return f"\x1b[38;2;{rgb[0]};{rgb[1]};{rgb[2]}m{string}\x1b[0m"

def neuron_to_unicode(value: float, numbers: bool = False) -> str:
    if value > 1.0 or value < 0.0:
        raise ValueError("value must be in range (0.0 ..= 1.0)")
    if numbers:
        return f"{value:.2f}"
    return color("██", (int(value * 255), int(value * 255), int(value * 255)))

def print_neural_input(ninput: list[float]):
    iterator = iter(ninput)
    print(neuron_to_unicode(next(iterator)))
    for square in chess.SQUARES:
        for _ in range(13):
            print(neuron_to_unicode(next(iterator)), end="")
        print()
        if square % 8 == 7:
            print()


def print_neural_output(noutput: list[float]):
    move_from = noutput[:64]
    move_to = noutput[64:]

    strings = ["" for _ in range(8)]

    for row in range(8):
        strings[7 - row] += "abcdefgh"[row] + " "
        for col in range(8):
            strings[7 - row] += neuron_to_unicode(move_from[row * 8 + col])

    for row in range(8):
        strings[7 - row] += "\t"
        strings[7 - row] += "abcdefgh"[row] + " "
        for col in range(8):
            strings[7 - row] += neuron_to_unicode(move_to[row * 8 + col])

    strings.append("  1 2 3 4 5 6 7 8 \t  1 2 3 4 5 6 7 8")
    strings.append("")

    print("\n".join(strings))
