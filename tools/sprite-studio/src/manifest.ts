import type { EditMode, SpriteFrame, SpriteManifest, SpriteRect } from "./types";

export const DEFAULT_MANIFEST_PATH =
  "assets/placeholder/rust-fighter.sprite.json";

export const BODY_METRICS_PATH = "assets/tuning/character-body-metrics.json";

export const PRESET_MANIFESTS = [
  ["Rust fighter", "assets/placeholder/rust-fighter.sprite.json"],
  ["Rust intro", "assets/placeholder/rust-start.sprite.json"],
  ["Duke fighter", "assets/placeholder/duke-fighter.sprite.json"],
  ["Duke intro", "assets/placeholder/duke-start.sprite.json"],
  ["Go fighter", "assets/placeholder/go-fighter.sprite.json"],
  ["Go intro", "assets/placeholder/go-start.sprite.json"],
  ["C fighter", "assets/placeholder/c-fighter.sprite.json"],
  ["C intro", "assets/placeholder/c-start.sprite.json"],
] as const;

export const EDIT_MODES: Array<{ id: EditMode; label: string }> = [
  { id: "inspect", label: "Inspect" },
  { id: "pivot", label: "Pivot" },
  { id: "projectile", label: "Projectile" },
  { id: "hurtbox", label: "Hurtbox" },
  { id: "hitbox", label: "Hitbox" },
];

export function frameByName(
  manifest: SpriteManifest | null,
  frameName: string | null,
): SpriteFrame | null {
  if (!manifest || !frameName) {
    return null;
  }
  return manifest.frames.find((frame) => frame.name === frameName) ?? null;
}

export function clipFrameNames(
  manifest: SpriteManifest | null,
  clipName: string | null,
): string[] {
  if (!manifest || !clipName) {
    return [];
  }
  const clip = manifest.clips.find((entry) => entry.name === clipName);
  return clip?.frames ?? [];
}

export function clampRect(rect: SpriteRect, frame: SpriteFrame): SpriteRect {
  const x = clamp(Math.round(rect.x), 0, frame.frame.w - 1);
  const y = clamp(Math.round(rect.y), 0, frame.frame.h - 1);
  const w = clamp(Math.round(rect.w), 1, frame.frame.w - x);
  const h = clamp(Math.round(rect.h), 1, frame.frame.h - y);
  return { ...rect, x, y, w, h };
}

export function defaultBox(
  x: number,
  y: number,
  frame: SpriteFrame,
  kind: "hurtbox" | "hitbox",
): SpriteRect {
  const size = kind === "hurtbox" ? { w: 84, h: 132 } : { w: 72, h: 42 };
  return clampRect(
    {
      x: Math.round(x - size.w / 2),
      y: Math.round(y - size.h / 2),
      w: size.w,
      h: size.h,
      label: kind === "hurtbox" ? "body" : "strike",
    },
    frame,
  );
}

export function cloneManifest(manifest: SpriteManifest): SpriteManifest {
  return JSON.parse(JSON.stringify(manifest)) as SpriteManifest;
}

export function inferCharacterId(path: string): string {
  const normalized = path.toLowerCase();
  if (normalized.includes("duke") || normalized.includes("java")) {
    return "duke";
  }
  if (normalized.includes("/go-") || normalized.includes("go-")) {
    return "go";
  }
  if (normalized.includes("/c-") || normalized.includes("c-fighter")) {
    return "c";
  }
  return "rust";
}

export function toNumber(value: string, fallback = 0): number {
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : fallback;
}

export function formatCoordinate(value: number | null): string {
  if (value === null || !Number.isFinite(value)) {
    return "-";
  }
  return value.toFixed(1);
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}
