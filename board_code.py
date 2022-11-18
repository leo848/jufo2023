import chess.pgn


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


def move_to_output(move) -> list[float]:
    """
    Converts a chess.Move to a list of floats.
    The first 64 numbers encode the 'from' field, the last 64 the 'to' field.
    The first number is the 'a1' square, the last is the 'h8' square.
    """
    result = [0.0] * 128
    result[move.from_square] = 1.0
    result[move.to_square + 64] = 1.0
    return result


def output_to_move(noutput: list[float]) -> chess.Move:
    from_square = max(enumerate(noutput[:64]), key=lambda p: p[1])[0]
    to_square = max(enumerate(noutput[64:]), key=lambda p: p[1])[0]
    return chess.Move(from_square, to_square)


def neuron_to_unicode(value: float, thresh: float = 0.5) -> str:
    if value > 1.0 or value < 0.0:
        raise ValueError("value must be in range (0.0 ..= 1.0)")
    if value > thresh:
        return "⚪"
    else:
        return "⚫"


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
    iterator = iter(noutput)
    for _ in range(2):
        for _ in range(8):
            for _ in range(8):
                print(neuron_to_unicode(next(iterator)), end="")
            print()
        print()
    print()
