import { loadSetting } from "@/settings/settings";

export const themes = [
  "anarcandy",
  "cardinal",
  "chess7",
  "companion",
  "disguised",
  "dubrovny",
  "fantasy",
  "fresca",
  "gioco",
  "horsey",
  "icpieces",
  "leipzig",
  "libra",
  "maestro",
  "shapes",
  "spatial",
  "staunty",
  "tatiana",
] as const;

const licenses = {
  ByNcSa: "CC-BY-NC-SA 4.0",
  BySa: "CC-BY-SA 4.0",
  Mit: "MIT",
} as const;

export const themesLicenses: Record<PieceTheme, { by: string, license?: typeof licenses[keyof typeof licenses] }> = {
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
  chess7: { by: "Style-7"},
  companion: { by: "David L. Brown"},
  fantasy: { by: "Maurizio Monge"},
  spatial: { by: "Maurizio Monge"},
  leipzig: { by: "Armando Hernandez Marroquin"},
};

export type PieceTheme = typeof themes[number];

export function loadPiece(piece: string): string {
  const theme: PieceTheme = loadSetting("theme");
  return `assets/img/chesspieces/${theme}/${piece}.svg`;
}
