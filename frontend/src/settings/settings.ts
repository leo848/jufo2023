import type {EvaluationModelName, PlayModelName} from "@/neural-models/model";
import type { PieceTheme } from "../chess/loadPieces";

type Settings = {
  theme: PieceTheme,
  onlyShowLegalMoves: boolean,
  showActivation: boolean,
  show: {
    continuation: boolean,
    capturedPieces: boolean,
    neuralOutput: boolean,
    evaluation: boolean,
  }
  maxMoves: number,
  autoPlay: {
    black: boolean,
    white: boolean,
  }
  temperature: number,
  playModelName: PlayModelName,
  evaluationModelName: EvaluationModelName,
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
    evaluation: true,
  },
  temperature: 0.5,
  playModelName: "20mmatestrain-512neurons-4layers-2",
  evaluationModelName: "20mevaltrain-1024neurons-4layers",
} as const;

let settings: Settings = Object.assign({}, defaultSettings, JSON.parse(localStorage.getItem("settings") || "{}"));

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
  settings = deepCopy(newSettings);
  localStorage.setItem("settings", JSON.stringify(settings));
}

export function resetSettings(): void {
  settings = { ...defaultSettings };
  localStorage.setItem("settings", "{}");
}

function deepCopy<T>(obj: T): T {
  return JSON.parse(JSON.stringify(obj));
}
