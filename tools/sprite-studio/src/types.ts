import type { SpritePivot as SpritePoint, SpriteCombatBox as SpriteRect } from "./sprite.generated";

export type {
  SpritePivot as SpritePoint,
  SpriteCombatBox as SpriteRect,
  SpriteFrameCombat as FrameCombat,
  SpriteFrame,
  SpriteClip,
  SpriteManifest,
} from "./sprite.generated";

export type SpriteAtlas = {
  path: string;
  data_url: string;
};

export type SpriteDocument = {
  repo_root: string;
  manifest_path: string;
  manifest_json: string;
  atlases: Record<string, SpriteAtlas>;
};

export type SaveResult = {
  path: string;
  bytes_written: number;
  backup_path?: string | null;
};

export type RuntimeValidationResult = {
  success: boolean;
  command: string;
  stdout: string;
  stderr: string;
};

export type TextDocument = {
  path: string;
  contents: string;
};

export type CharacterBodyMetrics = {
  width: number;
  standing_height: number;
  crouch_height: number;
};

export type BodyMetricsEntry = {
  id: string;
  body: CharacterBodyMetrics;
};

export type BodyMetricsCatalog = {
  schema: string;
  characters: BodyMetricsEntry[];
};

export type EditMode = "inspect" | "pivot" | "projectile" | "hurtbox" | "hitbox";

export type BoxKind = "hurtboxes" | "hitboxes";

export type SelectionTarget =
  | { type: "pivot" }
  | { type: "projectile" }
  | { type: "box"; kind: BoxKind; index: number };

export type ResizeHandle =
  | "nw"
  | "n"
  | "ne"
  | "e"
  | "se"
  | "s"
  | "sw"
  | "w";

export type DragIntent =
  | { action: "move"; target: SelectionTarget }
  | { action: "resize"; target: Extract<SelectionTarget, { type: "box" }>; handle: ResizeHandle };

export type DragState = DragIntent & {
  startMouse: SpritePoint;
  startPivot?: SpritePoint;
  startProjectile?: SpritePoint;
  startRect?: SpriteRect;
};

export type ValidationIssue = {
  severity: "error" | "warning";
  message: string;
  frameName?: string;
};
