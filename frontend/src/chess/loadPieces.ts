import { loadSetting } from "@/settings/settings";

const pieces = [
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
  "wikipedia",
] as const;

export type PieceTheme = typeof pieces[number];

export function loadPiece(piece: string): string {
  const theme: PieceTheme = loadSetting("theme");
  return `assets/img/chesspieces/${theme}/${piece}.svg`;
}
