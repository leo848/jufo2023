import type { PieceTheme } from "../chess/loadPieces";

type Settings = {
  theme: PieceTheme,
  onlyShowLegalMoves: boolean,
  showActivation: boolean,
  autoPlay: {
    black: boolean,
    white: boolean,
  }
}

const defaultSettings: Settings = {
  theme: "maestro",
  onlyShowLegalMoves: true,
  showActivation: true,
  autoPlay: {
    black: false,
    white: false,
  }
} as const;

let settings: Settings = { ...defaultSettings };

export function loadSettings(): Settings {
  // Defensive deep copy
  return deepCopy(settings);
}

export function loadSetting<K extends keyof Settings>(key: K): Settings[K] {
  return settings[key];
}

export function saveSettings(newSettings: Settings): void {
  settings = newSettings;
}

export function resetSettings(): void {
  settings = { ...defaultSettings };
}

function deepCopy<T>(obj: T): T {
  return JSON.parse(JSON.stringify(obj));
}
