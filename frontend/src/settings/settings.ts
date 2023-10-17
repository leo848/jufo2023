import type {AutoencoderModelName, EvaluationModelName, PlayModelName} from "@/neural-models/model";
import type { PieceTheme } from "../chess/loadPieces";

type Settings = {
  theme: PieceTheme,
  onlyShowLegalMoves: boolean,
  showActivation: boolean,
  show: {
    continuation: boolean,
    autoencode: boolean,
    capturedPieces: boolean,
    neuralOutput: boolean,
    evaluation: boolean,
    infoboxes: boolean,
  }
  maxMoves: number,
  autoPlay: {
    black: boolean,
    white: boolean,
  },
  orientation: "white" | "black",
  temperature: number,
  playModelName: PlayModelName,
  evaluationModelName: EvaluationModelName,
  autoencoderModelName: AutoencoderModelName,
}

const _defaultSettings: Settings = {
  theme: "maestro",
  onlyShowLegalMoves: true,
  showActivation: true,
  maxMoves: 8,
  autoPlay: {
    black: true,
    white: false,
  },
  show: {
    autoencode: false,
    continuation: false,
    capturedPieces: true,
    neuralOutput: true,
    evaluation: true,
    infoboxes: true,
  },
  orientation: "white",
  temperature: 0.5,
  playModelName: "50M_first",
  evaluationModelName: "20mevaltrain-1024neurons-4layers",
  autoencoderModelName: "256-64-autoencoder"
} as const;

let settings: Settings = Object.assign({}, _defaultSettings, JSON.parse(localStorage.getItem("settings") || "{}"));

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
  settings = { ..._defaultSettings };
  localStorage.setItem("settings", "{}");
}

export function defaultSettings(): Settings {
  return deepCopy(_defaultSettings);
}

function deepCopy<T>(obj: T): T {
  return JSON.parse(JSON.stringify(obj));
}
