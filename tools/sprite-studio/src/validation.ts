import type {
  SpriteFrame,
  SpriteManifest,
  SpritePoint,
  SpriteRect,
  ValidationIssue,
} from "./types";

export function validateManifest(manifest: SpriteManifest | null): ValidationIssue[] {
  if (!manifest) {
    return [{ severity: "error", message: "No manifest loaded." }];
  }

  const issues: ValidationIssue[] = [];
  if (manifest.schema !== "borrow-fighters.sprite.v1") {
    issues.push({
      severity: "warning",
      message: `Unexpected schema: ${manifest.schema || "<missing>"}.`,
    });
  }
  if (!manifest.image) {
    issues.push({ severity: "error", message: "Manifest field `image` is required." });
  }
  if (!isPositive(manifest.cell?.w) || !isPositive(manifest.cell?.h)) {
    issues.push({ severity: "error", message: "Manifest `cell.w` and `cell.h` must be positive." });
  }
  if (manifest.scale !== undefined && manifest.scale !== null && manifest.scale <= 0) {
    issues.push({ severity: "error", message: "Manifest `scale` must be positive when present." });
  }

  const frameNames = new Set<string>();
  for (const frame of manifest.frames) {
    if (frameNames.has(frame.name)) {
      issues.push({
        severity: "error",
        message: `Duplicate frame name: ${frame.name}.`,
        frameName: frame.name,
      });
    }
    frameNames.add(frame.name);
    validateFrame(frame, issues);
  }

  for (const clip of manifest.clips) {
    if (!clip.name) {
      issues.push({ severity: "error", message: "Clip name cannot be empty." });
    }
    if (!clip.frames.length) {
      issues.push({
        severity: "warning",
        message: `Clip ${clip.name || "<empty>"} has no frames.`,
      });
    }
    for (const frameName of clip.frames) {
      if (!frameNames.has(frameName)) {
        issues.push({
          severity: "error",
          message: `Clip ${clip.name} references missing frame ${frameName}.`,
          frameName,
        });
      }
    }
  }

  return issues;
}

export function visualScaleSummary(frame: SpriteFrame | null, scale: number) {
  const bounds = frame?.trimmed_bounds ?? frame?.source_crop;
  if (!frame || !bounds) {
    return null;
  }
  const width = bounds.w * scale;
  const height = bounds.h * scale;
  const heightOk = height >= 185 && height <= 210;
  const widthOk = width >= 110 && width <= 150;
  return {
    width,
    height,
    heightOk,
    widthOk,
    message:
      heightOk && widthOk
        ? "Inside current Rust/C visual target."
        : "Outside current target; adjust atlas art, pivot or manifest scale before gameplay boxes.",
  };
}

function validateFrame(frame: SpriteFrame, issues: ValidationIssue[]) {
  if (!frame.name) {
    issues.push({ severity: "error", message: "Frame name cannot be empty." });
  }
  if (!isPositive(frame.frame?.w) || !isPositive(frame.frame?.h)) {
    issues.push({
      severity: "error",
      frameName: frame.name,
      message: "Frame rectangle must have positive width and height.",
    });
  }
  if (!pointInsideFrame(frame.pivot, frame)) {
    issues.push({
      severity: "warning",
      frameName: frame.name,
      message: "Pivot is outside the frame rectangle.",
    });
  }
  if (!isPositive(frame.duration_ms)) {
    issues.push({
      severity: "error",
      frameName: frame.name,
      message: "Frame duration must be positive.",
    });
  }

  for (const [index, box] of (frame.combat?.hurtboxes ?? []).entries()) {
    validateBox(box, frame, `hurtbox ${index + 1}`, issues);
  }
  for (const [index, box] of (frame.combat?.hitboxes ?? []).entries()) {
    validateBox(box, frame, `hitbox ${index + 1}`, issues);
  }
  const origin = frame.combat?.projectile_origin;
  if (origin && !pointInsideFrame(origin, frame)) {
    issues.push({
      severity: "error",
      frameName: frame.name,
      message: "Projectile origin must be inside the frame.",
    });
  }
}

function validateBox(
  box: SpriteRect,
  frame: SpriteFrame,
  label: string,
  issues: ValidationIssue[],
) {
  if (!isPositive(box.w) || !isPositive(box.h)) {
    issues.push({
      severity: "error",
      frameName: frame.name,
      message: `${label} must have positive width and height.`,
    });
  }
  if (box.x < 0 || box.y < 0 || box.x + box.w > frame.frame.w || box.y + box.h > frame.frame.h) {
    issues.push({
      severity: "error",
      frameName: frame.name,
      message: `${label} must stay inside the frame.`,
    });
  }
  if (box.label !== undefined && box.label.trim() === "") {
    issues.push({
      severity: "error",
      frameName: frame.name,
      message: `${label} label cannot be empty when present.`,
    });
  }
}

function pointInsideFrame(point: SpritePoint | undefined, frame: SpriteFrame): boolean {
  return (
    !!point &&
    point.x >= 0 &&
    point.y >= 0 &&
    point.x <= frame.frame.w &&
    point.y <= frame.frame.h
  );
}

function isPositive(value: number | undefined): boolean {
  return typeof value === "number" && Number.isFinite(value) && value > 0;
}
