let model = null;

let board = createBoard();

const $bestMoves = $("#best-moves");

async function loadModel(name) {
  model = await tf.loadLayersModel(`models/${name}/model.json`);
  return model;
}

loadModel(settings.model).then(update);

function createBoard(options = {}) {
  return Chessboard("board", Object.assign(options, {
    draggable: true,
    dropOffBoard: "trash",
    position: "start",
    pieceTheme,
    onDrop,
    onSnapEnd,
  }));
}

let game = new Chess();

function pieceTheme(piece) {
  return `assets/img/chesspieces/${settings.theme}/${piece}.svg`;
}

function onDrop(source, target) {
  let move = game.move({
    from: source,
    to: target,
    promotion: "q",
  });

  if (move === null) return "snapback";

  update();
}

function getMove(input) {
  let move = game.move(input);
  if (move != null) game.undo();
  return move;
}

function onSnapEnd() {
  board.position(game.fen());
}

function update() {
  if (model == null) return;

  const input = models[settings.model].encodeInput(game.fen());
  const output = model.predict(input).dataSync();
  let amount = 10;
  let moves = [];
  while (moves.length < 8 && amount < 10001) {
    moves = models[settings.model].decodeOutput(output, { amount })
      .map((obj) => Object.assign(obj, { move: getMove(obj) }))
      .map((obj) => Object.assign(obj, { valid: obj.move != null }))
      .filter(({ valid }) => settings.showInvalidMoves || valid);
    amount *= 10;
  }

  if (
    (game.turn() === "w" && settings.playAuto.white) ||
    (game.turn() === "b" && settings.playAuto.black)
  ) {
    if (
      game.in_threefold_repetition() ||
      game.in_checkmate() ||
      game.in_stalemate()
    ) {
      settings.playAuto = { white: false, black: false };
      return;
    }
    game.move(moves[0]);
    board.position(game.fen());
    requestAnimationFrame(update);
  }

  updateBestMoves(moves);
}

function updateBestMoves(moves, amount = 8) {
  $bestMoves.empty();

  moves.slice(0, amount).forEach(({ act, from, to, move, valid }) => {
    $bestMoves.append(
      $("<button>")
        .addClass("move")
        .addClass(valid ? "valid" : "invalid")
        .css("background-color", valid ? actToColor(act) : "darkgray")
        .click(() => {
          game.move({ from, to });
          board.position(game.fen());
          update();
        })
        .append(
          $("<span>")
            .addClass("act")
            .text(act.toFixed(4).slice(1)),
          $("<span>")
            .addClass("move")
            .html(move ? moveToHtml(move) : `${from}-${to}`)
        ),
      $("<br>")
    );
  });
}

const interpolate = (t) => (x) =>
  1 + 1 / t - 1 / (t * x + 1) - 1 / (t * t * x + t);

function actToColor(act) {
  // Return a color based on the activation value.
  // The color is a gradient from red to green.
  // It is red when the activation is 0 and green when the activation is 1.
  // The gradient is logarithmic.
  let newAct = interpolate(25)(act);
  let red = Math.min(1.0, 2.0 - 2.0 * newAct) * 255;
  let green = Math.min(1.0, 2.0 * newAct) * 255;
  return `rgb(${red}, ${green}, 0)`;
}

function moveToHtml(move) {
  let san = move.san;
  if (move.piece !== "p" && !san.startsWith("O")) san = san.slice(1);
  const imgSrc = pieceTheme(move.color + move.piece.toUpperCase());
  return `<div class="move-desc"><img class="piece-letter" src="${imgSrc}"/> <span>${san}</span></div>`;
}
