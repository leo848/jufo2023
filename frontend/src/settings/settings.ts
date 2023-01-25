import {defaultModel} from "@/neural-models/model";
import type { PieceTheme } from "../chess/loadPieces";

type Settings = {
  theme: PieceTheme,
  onlyShowLegalMoves: boolean,
  showActivation: boolean,
  show: {
    continuation: boolean,
    capturedPieces: boolean,
    neuralOutput: boolean,
  }
  maxMoves: number,
  autoPlay: {
    black: boolean,
    white: boolean,
  }
  temperature: number,
  model: typeof defaultModel,
}

const defaultSettings: Settings = {
  theme: "maestro",
  onlyShowLegalMoves: true,
  showActivation: true,
  maxMoves: 8,
  autoPlay: {
    black: false,
    white: false,
  },
  show: {
    continuation: false,
    capturedPieces: true,
    neuralOutput: true,
  },
  temperature: 0.5,
  model: defaultModel,
} as const;

let settings: Settings = { ...defaultSettings };

export function loadSettings(): Settings {
  // Defensive deep copy
  return deepCopy(settings);
}

export function loadSetting<K extends keyof Settings>(key: K): Settings[K] {
  if (!settings.hasOwnProperty(key)) {
    throw new Error(`Invalid setting key: ${key}`);
  }
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
