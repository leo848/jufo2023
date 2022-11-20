const settings = {
  showInvalidMoves: false,
  playAuto: {
    white: false,
    black: false,
  },
  theme: "gioco",
  model: "original",
};

const $settings = $("#settings");

$settings.append($("<h2>").text("Settings"));

$settings.append(
  $("<label>")
    .text("Show invalid moves")
    .append(
      $("<input>")
        .attr("type", "checkbox")
        .change(function () {
          settings.showInvalidMoves = $(this).prop("checked");
          update();
        })
        .prop("checked", settings.showInvalidMoves)
    )
);

$settings.append($("<span>").text("Play automatically for: "));

for (const color of ["White", "Black"]) {
  $settings.append(
    $("<label>")
      .html(`&emsp;${color}`)
      .append(
        $("<input>")
          .attr("type", "checkbox")
          .change(function (elt) {
            settings.playAuto[color.toLowerCase()] = $(this).prop("checked");
            update();
          })
          .prop("checked", settings.playAuto[color.toLowerCase()])
      )
  );
}

const licenses = {
  ByNcSa: "CC-BY-NC-SA 4.0", // + '<img src=https://i.creativecommons.org/l/by-nc-sa/4.0/88x31.png"/>',
  BySa: "CC-BY-SA 4.0", // + <img src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png"/>',
  Mit: "MIT",
};
const themes = {
  horsey: { by: "cham and michael1241", license: licenses.ByNcSa },
  shapes: { by: "flugsio", license: licenses.BySa },
  maestro: { by: "sadsnake1", license: licenses.ByNcSa },
  fresca: { by: "sadsnake1", license: licenses.ByNcSa },
  cardinal: { by: "sadsnake1", license: licenses.ByNcSa },
  icpieces: { by: "sadsnake1", license: licenses.ByNcSa },
  gioco: { by: "sadsnake1", license: licenses.ByNcSa },
  tatiana: { by: "sadsnake1", license: licenses.ByNcSa },
  staunty: { by: "sadsnake1", license: licenses.ByNcSa },
  dubrovny: { by: "sadsnake1", license: licenses.ByNcSa },
  libra: { by: "sadsnake1", license: licenses.ByNcSa },
  anarcandy: { by: "caderek", license: licenses.ByNcSa },
  disguised: { by: "danegraphics", license: licenses.ByNcSa },
  chess7: { by: "Style-7", license: null },
  companion: { by: "David L. Brown", license: null },
  fantasy: { by: "Maurizio Monge", license: null },
  spatial: { by: "Maurizio Monge", license: null },
  leipzig: { by: "Armando Hernandez Marroquin", license: null },
};

$settings.append(
  // Selection of piece icons
  $("<label>")
    .text("Piece icons")
    .append(
      $("<select>")
        .val(settings.theme)
        .change(function () {
          settings.theme = $(this).val();
          const obj = themes[settings.theme];
          $credit.html(
            `by ${obj.by} ${obj.license ? "under " + obj.license : ""}`
          );
          board.destroy();
          board = createBoard();
          update();
          board.position(game.fen());
        })
        .append(
          (() => {
            let keys = Object.keys(themes);
            keys.sort();
            return keys;
          })().map((theme) => {
            return $("<option>").attr("value", theme).text(theme);
          })
        )
    )
);

const $credit = $("<div>").attr("id", "credit");
$settings.append($credit);

const models = {
  original: { encodeInput: Original.encodeInput, decodeOutput: Original.decodeOutput },
};


