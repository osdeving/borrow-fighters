import type { PointerEvent } from "react";
import type {
  BoxKind,
  DragIntent,
  ResizeHandle,
  SelectionTarget,
  SpriteFrame,
  SpritePoint,
  SpriteRect,
} from "./types";

const POINT_RADIUS = 8;
const HANDLE_RADIUS = 7;

export function localPointFromEvent(
  event: PointerEvent<HTMLElement>,
  zoom: number,
): SpritePoint {
  const rect = event.currentTarget.getBoundingClientRect();
  return {
    x: (event.clientX - rect.left) / zoom,
    y: (event.clientY - rect.top) / zoom,
  };
}

export function hitTestFrame(frame: SpriteFrame, point: SpritePoint): DragIntent | null {
  const projectile = frame.combat?.projectile_origin;
  if (projectile && distance(point, projectile) <= POINT_RADIUS) {
    return { action: "move", target: { type: "projectile" } };
  }
  if (distance(point, frame.pivot) <= POINT_RADIUS) {
    return { action: "move", target: { type: "pivot" } };
  }

  const hitboxIntent = hitTestBoxes(frame.combat?.hitboxes ?? [], "hitboxes", point);
  if (hitboxIntent) {
    return hitboxIntent;
  }
  return hitTestBoxes(frame.combat?.hurtboxes ?? [], "hurtboxes", point);
}

export function targetKey(target: SelectionTarget | null): string {
  if (!target) {
    return "";
  }
  if (target.type === "box") {
    return `${target.kind}:${target.index}`;
  }
  return target.type;
}

export function clampPoint(point: SpritePoint, frame: SpriteFrame): SpritePoint {
  return {
    x: clamp(Math.round(point.x), 0, frame.frame.w),
    y: clamp(Math.round(point.y), 0, frame.frame.h),
  };
}

export function moveRect(rect: SpriteRect, delta: SpritePoint, frame: SpriteFrame): SpriteRect {
  return {
    ...rect,
    x: clamp(Math.round(rect.x + delta.x), 0, frame.frame.w - rect.w),
    y: clamp(Math.round(rect.y + delta.y), 0, frame.frame.h - rect.h),
  };
}

export function resizeRect(
  rect: SpriteRect,
  handle: ResizeHandle,
  delta: SpritePoint,
  frame: SpriteFrame,
): SpriteRect {
  let left = rect.x;
  let top = rect.y;
  let right = rect.x + rect.w;
  let bottom = rect.y + rect.h;

  if (handle.includes("w")) {
    left += delta.x;
  }
  if (handle.includes("e")) {
    right += delta.x;
  }
  if (handle.includes("n")) {
    top += delta.y;
  }
  if (handle.includes("s")) {
    bottom += delta.y;
  }

  left = clamp(Math.round(left), 0, frame.frame.w - 1);
  top = clamp(Math.round(top), 0, frame.frame.h - 1);
  right = clamp(Math.round(right), left + 1, frame.frame.w);
  bottom = clamp(Math.round(bottom), top + 1, frame.frame.h);

  return {
    ...rect,
    x: left,
    y: top,
    w: right - left,
    h: bottom - top,
  };
}

export function handlePoints(rect: SpriteRect): Array<{ handle: ResizeHandle; point: SpritePoint }> {
  const midX = rect.x + rect.w / 2;
  const midY = rect.y + rect.h / 2;
  const right = rect.x + rect.w;
  const bottom = rect.y + rect.h;
  return [
    { handle: "nw", point: { x: rect.x, y: rect.y } },
    { handle: "n", point: { x: midX, y: rect.y } },
    { handle: "ne", point: { x: right, y: rect.y } },
    { handle: "e", point: { x: right, y: midY } },
    { handle: "se", point: { x: right, y: bottom } },
    { handle: "s", point: { x: midX, y: bottom } },
    { handle: "sw", point: { x: rect.x, y: bottom } },
    { handle: "w", point: { x: rect.x, y: midY } },
  ];
}

function hitTestBoxes(
  boxes: SpriteRect[],
  kind: BoxKind,
  point: SpritePoint,
): DragIntent | null {
  for (let index = boxes.length - 1; index >= 0; index -= 1) {
    const box = boxes[index];
    for (const handle of handlePoints(box)) {
      if (distance(point, handle.point) <= HANDLE_RADIUS) {
        return {
          action: "resize",
          target: { type: "box", kind, index },
          handle: handle.handle,
        };
      }
    }
    if (point.x >= box.x && point.y >= box.y && point.x <= box.x + box.w && point.y <= box.y + box.h) {
      return { action: "move", target: { type: "box", kind, index } };
    }
  }
  return null;
}

function distance(a: SpritePoint, b: SpritePoint): number {
  return Math.hypot(a.x - b.x, a.y - b.y);
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}
