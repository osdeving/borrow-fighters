export type SpritePoint = {
  x: number;
  y: number;
};

export type SpriteRect = {
  x: number;
  y: number;
  w: number;
  h: number;
  label?: string;
};

export type FrameCombat = {
  hurtboxes?: SpriteRect[];
  hitboxes?: SpriteRect[];
  projectile_origin?: SpritePoint;
};

export type SpriteFrame = {
  name: string;
  clip: string;
  duration_ms: number;
  pivot: SpritePoint;
  frame: SpriteRect;
  trimmed_bounds?: SpriteRect;
  source_crop?: SpriteRect;
  combat?: FrameCombat;
};

export type SpriteClip = {
  name: string;
  frames: string[];
  loop: boolean;
};

export type SpriteManifest = {
  schema: string;
  image: string;
  source?: string;
  cell: {
    w: number;
    h: number;
  };
  default_pivot: SpritePoint;
  scale?: number | null;
  notes?: string[];
  frames: SpriteFrame[];
  clips: SpriteClip[];
};

export type SpriteDocument = {
  repo_root: string;
  manifest_path: string;
  atlas_path: string;
  manifest_json: string;
  atlas_data_url: string;
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
