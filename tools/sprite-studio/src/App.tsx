import { useEffect, useMemo, useRef, useState } from "react";
import { open, save as saveDialog } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import {
  DEFAULT_MANIFEST_PATH,
  BODY_METRICS_PATH,
  EDIT_MODES,
  PRESET_MANIFESTS,
  cloneManifest,
  clipFrameNames,
  clampRect,
  defaultBox,
  formatCoordinate,
  frameByName,
  inferCharacterId,
  toNumber,
} from "./manifest";
import {
  clampPoint,
  handlePoints,
  hitTestFrame,
  localPointFromEvent,
  moveRect,
  resizeRect,
  targetKey,
} from "./geometry";
import { validateManifest, visualScaleSummary } from "./validation";
import { HelpCenter } from "./HelpCenter";
import { MenuBar } from "./MenuBar";
import { Timeline } from "./Timeline";
import type {
  BoxKind,
  BodyMetricsCatalog,
  CharacterBodyMetrics,
  DragIntent,
  DragState,
  EditMode,
  RuntimeValidationResult,
  SaveResult,
  SelectionTarget,
  SpriteDocument,
  SpriteFrame,
  SpriteManifest,
  SpritePoint,
  SpriteRect,
  TextDocument,
  ValidationIssue,
} from "./types";
import "./App.css";

const HISTORY_LIMIT = 60;
type CombatPreset =
  | "standing_hurt"
  | "crouch_hurt"
  | "mid_strike"
  | "low_strike"
  | "projectile_origin"
  | "clear_frame";

function App() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [manifestPath, setManifestPath] = useState(DEFAULT_MANIFEST_PATH);
  const [document, setDocument] = useState<SpriteDocument | null>(null);
  const [manifest, setManifest] = useState<SpriteManifest | null>(null);
  const [selectedClip, setSelectedClip] = useState<string | null>(null);
  const [selectedFrame, setSelectedFrame] = useState<string | null>(null);
  const [editMode, setEditMode] = useState<EditMode>("inspect");
  const [zoom, setZoom] = useState(2);
  const [dirty, setDirty] = useState(false);
  const [status, setStatus] = useState("Ready");
  const [mouseLocal, setMouseLocal] = useState<SpritePoint | null>(null);
  const [selection, setSelection] = useState<SelectionTarget | null>(null);
  const [dragState, setDragState] = useState<DragState | null>(null);
  const [undoStack, setUndoStack] = useState<SpriteManifest[]>([]);
  const [redoStack, setRedoStack] = useState<SpriteManifest[]>([]);
  const [showGrid, setShowGrid] = useState(true);
  const [showBounds, setShowBounds] = useState(true);
  const [showScaleGuide, setShowScaleGuide] = useState(true);
  const [snapEnabled, setSnapEnabled] = useState(true);
  const [snapStep, setSnapStep] = useState(4);
  const [hoverIntent, setHoverIntent] = useState<DragIntent | null>(null);
  const [reviewNotes, setReviewNotes] = useState("");
  const [runtimeValidation, setRuntimeValidation] =
    useState<RuntimeValidationResult | null>(null);
  const [runtimeValidationRunning, setRuntimeValidationRunning] =
    useState(false);
  const [leftPanelOpen, setLeftPanelOpen] = useState(true);
  const [rightPanelOpen, setRightPanelOpen] = useState(true);
  const [timelineOpen, setTimelineOpen] = useState(true);
  const [helpOpen, setHelpOpen] = useState(false);
  const [autosaveEnabled, setAutosaveEnabled] = useState(true);
  const [lastAutosavePath, setLastAutosavePath] = useState<string | null>(null);
  const [lastBackupPath, setLastBackupPath] = useState<string | null>(null);
  const [bodyCatalog, setBodyCatalog] = useState<BodyMetricsCatalog | null>(
    null,
  );
  const [bodyMetricsPath, setBodyMetricsPath] = useState(BODY_METRICS_PATH);
  const [selectedCharacterId, setSelectedCharacterId] = useState("rust");
  const [bodyDirty, setBodyDirty] = useState(false);

  const frameNames = useMemo(
    () => clipFrameNames(manifest, selectedClip),
    [manifest, selectedClip],
  );
  const timelineFrames = useMemo(
    () =>
      frameNames
        .map((frameName) => frameByName(manifest, frameName))
        .filter((frame): frame is SpriteFrame => !!frame)
        .map((frame) => ({ ...frame, selected: frame.name === selectedFrame })),
    [frameNames, manifest, selectedFrame],
  );
  const activeFrame = useMemo(
    () => frameByName(manifest, selectedFrame),
    [manifest, selectedFrame],
  );
  const manifestJson = useMemo(
    () => (manifest ? JSON.stringify(manifest, null, 2) : ""),
    [manifest],
  );
  const validationIssues = useMemo(
    () => validateManifest(manifest),
    [manifest],
  );
  const hasValidationErrors = validationIssues.some(
    (issue) => issue.severity === "error",
  );
  const scaleSummary = useMemo(
    () => visualScaleSummary(activeFrame, manifest?.scale ?? 1),
    [activeFrame, manifest?.scale],
  );

  useEffect(() => {
    void loadManifest(DEFAULT_MANIFEST_PATH);
  }, []);

  useEffect(() => {
    drawFrame(document?.atlas_data_url, activeFrame, zoom, canvasRef.current);
  }, [document?.atlas_data_url, activeFrame, zoom]);

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      const target = event.target as HTMLElement | null;
      const isTextInput =
        target?.tagName === "INPUT" || target?.tagName === "TEXTAREA";
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") {
        event.preventDefault();
        void saveManifest();
        return;
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "z") {
        event.preventDefault();
        if (event.shiftKey) {
          redo();
        } else {
          undo();
        }
        return;
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "y") {
        event.preventDefault();
        redo();
        return;
      }
      if (!isTextInput && event.key === "ArrowLeft") {
        event.preventDefault();
        moveFrame(-1);
      }
      if (!isTextInput && event.key === "ArrowRight") {
        event.preventDefault();
        moveFrame(1);
      }
      if (!isTextInput && event.key === "F1") {
        event.preventDefault();
        setHelpOpen(true);
      }
      if (!isTextInput && (event.key === "=" || event.key === "+")) {
        event.preventDefault();
        changeZoom(0.25);
      }
      if (!isTextInput && event.key === "-") {
        event.preventDefault();
        changeZoom(-0.25);
      }
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  });

  useEffect(() => {
    if (!autosaveEnabled || !dirty || !document || !manifest) {
      return;
    }
    const timeout = window.setTimeout(() => {
      void autosaveManifest();
    }, 2500);
    return () => window.clearTimeout(timeout);
  }, [autosaveEnabled, dirty, document?.manifest_path, manifestJson]);

  async function loadManifest(path: string) {
    try {
      setStatus(`Loading ${path}`);
      const loaded = await invoke<SpriteDocument>("load_sprite_manifest", {
        path,
      });
      const parsed = JSON.parse(loaded.manifest_json) as SpriteManifest;
      const firstClip = parsed.clips[0]?.name ?? null;
      const firstFrame =
        parsed.clips[0]?.frames[0] ?? parsed.frames[0]?.name ?? null;

      setDocument(loaded);
      setManifest(parsed);
      setManifestPath(loaded.manifest_path);
      setSelectedCharacterId(inferCharacterId(loaded.manifest_path));
      setSelectedClip(firstClip);
      setSelectedFrame(firstFrame);
      setSelection(null);
      setUndoStack([]);
      setRedoStack([]);
      setDirty(false);
      setStatus(`Loaded ${loaded.manifest_path}`);
      void loadBodyMetrics();
    } catch (error) {
      setStatus(errorMessage(error));
    }
  }

  async function chooseManifestFile() {
    try {
      const selected = await open({
        defaultPath: `${document?.repo_root ?? ""}/assets/placeholder`,
        filters: [{ name: "Borrow Fighters sprite manifest", extensions: ["json"] }],
        multiple: false,
        title: "Open sprite manifest",
      });
      const path = dialogPath(selected);
      if (!path) {
        setStatus("Open cancelled.");
        return;
      }
      await loadManifest(path);
    } catch (error) {
      setStatus(errorMessage(error));
    }
  }

  async function loadBodyMetrics(path = BODY_METRICS_PATH) {
    try {
      const loaded = await invoke<TextDocument>("load_repo_text_file", { path });
      const parsed = JSON.parse(loaded.contents) as BodyMetricsCatalog;
      setBodyCatalog(parsed);
      setBodyMetricsPath(loaded.path);
      setBodyDirty(false);
    } catch (error) {
      setStatus(`Body metrics not loaded: ${errorMessage(error)}`);
    }
  }

  async function saveBodyMetrics() {
    if (!bodyCatalog) {
      return;
    }
    const invalid = bodyCatalog.characters.some(
      (entry) =>
        entry.body.width <= 0 ||
        entry.body.standing_height <= 0 ||
        entry.body.crouch_height <= 0,
    );
    if (invalid) {
      setStatus("Fix body metrics before saving.");
      return;
    }
    try {
      const contents = `${JSON.stringify(bodyCatalog, null, 2)}\n`;
      const result = await invoke<SaveResult>("save_repo_text_file", {
        path: bodyMetricsPath,
        contents,
      });
      setBodyDirty(false);
      setStatus(`Saved ${result.path} (${result.bytes_written} bytes)`);
    } catch (error) {
      setStatus(errorMessage(error));
    }
  }

  async function runRuntimeValidation() {
    try {
      setRuntimeValidationRunning(true);
      setStatus("Running runtime sprite validation...");
      const result = await invoke<RuntimeValidationResult>(
        "validate_runtime_assets",
      );
      setRuntimeValidation(result);
      setStatus(
        result.success
          ? `Runtime validation passed: ${result.command}`
          : `Runtime validation failed: ${result.command}`,
      );
    } catch (error) {
      setStatus(errorMessage(error));
    } finally {
      setRuntimeValidationRunning(false);
    }
  }

  async function exportReviewJson() {
    if (!document || !manifest || !activeFrame) {
      return;
    }
    const fileName = `${activeFrame.name}-sprite-studio-review.json`;
    try {
      const selected = await saveDialog({
        defaultPath: `${document.repo_root}/${fileName}`,
        filters: [{ name: "Sprite Studio review", extensions: ["json"] }],
        title: "Export Sprite Studio review JSON",
      });
      if (!selected) {
        setStatus("Review export cancelled.");
        return;
      }
      const payload = {
        schema: "borrow-fighters.sprite-studio-review.v1",
        created_by: "Borrow Fighters Sprite Studio",
        manifest_path: document.manifest_path,
        atlas_path: document.atlas_path,
        clip: selectedClip,
        frame: activeFrame.name,
        frame_data: activeFrame,
        manifest_scale: manifest.scale ?? 1,
        visual_scale: scaleSummary,
        body_metrics_character: selectedCharacterId,
        body_metrics: activeBodyMetrics,
        validation_issues: validationIssues,
        runtime_validation: runtimeValidation,
        notes: reviewNotes,
      };
      const result = await invoke<SaveResult>("save_repo_text_file", {
        path: selected,
        contents: `${JSON.stringify(payload, null, 2)}\n`,
      });
      setStatus(`Exported ${result.path}`);
    } catch (error) {
      setStatus(errorMessage(error));
    }
  }

  async function autosaveManifest() {
    if (!document || !manifest) {
      return;
    }
    try {
      const result = await invoke<SaveResult>("autosave_sprite_manifest", {
        path: document.manifest_path,
        manifestJson,
      });
      setLastAutosavePath(result.path);
      setStatus(`Autosaved ${result.path}`);
    } catch (error) {
      setStatus(`Autosave failed: ${errorMessage(error)}`);
    }
  }

  async function saveManifest() {
    if (!document || !manifest) {
      return;
    }
    if (hasValidationErrors) {
      setStatus("Fix validation errors before saving.");
      return;
    }
    try {
      const result = await invoke<SaveResult>("save_sprite_manifest", {
        path: document.manifest_path,
        manifestJson,
      });
      setDirty(false);
      setLastBackupPath(result.backup_path ?? null);
      setStatus(
        result.backup_path
          ? `Saved ${result.path}; backup ${result.backup_path}`
          : `Saved ${result.path} (${result.bytes_written} bytes)`,
      );
    } catch (error) {
      setStatus(errorMessage(error));
    }
  }

  function pushHistory(snapshot = manifest) {
    if (!snapshot) {
      return;
    }
    setUndoStack((stack) => [
      ...stack.slice(Math.max(0, stack.length - HISTORY_LIMIT + 1)),
      cloneManifest(snapshot),
    ]);
    setRedoStack([]);
  }

  function updateManifest(
    mutator: (next: SpriteManifest) => void,
    options: { history?: boolean } = {},
  ) {
    if (!manifest) {
      return;
    }
    if (options.history !== false) {
      pushHistory(manifest);
    }
    const next = cloneManifest(manifest);
    mutator(next);
    setManifest(next);
    setDirty(true);
  }

  function updateActiveFrame(
    mutator: (frame: SpriteFrame) => void,
    options: { history?: boolean } = {},
  ) {
    if (!activeFrame) {
      return;
    }
    updateManifest((next) => {
      const nextFrame = next.frames.find(
        (frame) => frame.name === activeFrame.name,
      );
      if (nextFrame) {
        mutator(nextFrame);
      }
    }, options);
  }

  function undo() {
    if (!manifest || undoStack.length === 0) {
      return;
    }
    const previous = undoStack[undoStack.length - 1];
    setUndoStack((stack) => stack.slice(0, -1));
    setRedoStack((stack) => [...stack, cloneManifest(manifest)]);
    setManifest(cloneManifest(previous));
    setDirty(true);
    setStatus("Undo");
  }

  function redo() {
    if (!manifest || redoStack.length === 0) {
      return;
    }
    const next = redoStack[redoStack.length - 1];
    setRedoStack((stack) => stack.slice(0, -1));
    setUndoStack((stack) => [...stack, cloneManifest(manifest)]);
    setManifest(cloneManifest(next));
    setDirty(true);
    setStatus("Redo");
  }

  function handleClipSelection(clipName: string) {
    const names = clipFrameNames(manifest, clipName);
    setSelectedClip(clipName);
    setSelectedFrame(names[0] ?? null);
    setSelection(null);
  }

  function moveFrame(delta: number) {
    if (!frameNames.length || !selectedFrame) {
      return;
    }
    const index = frameNames.indexOf(selectedFrame);
    const nextIndex = (index + delta + frameNames.length) % frameNames.length;
    setSelectedFrame(frameNames[nextIndex]);
    setSelection(null);
  }

  function changeZoom(delta: number) {
    setZoom((current) => Math.min(5, Math.max(0.5, current + delta)));
  }

  function handleStageWheel(event: React.WheelEvent<HTMLDivElement>) {
    event.preventDefault();
    changeZoom(event.deltaY < 0 ? 0.25 : -0.25);
  }

  function snapPoint(point: SpritePoint, frame: SpriteFrame): SpritePoint {
    if (!snapEnabled || snapStep <= 1) {
      return clampPoint(point, frame);
    }
    return clampPoint(
      {
        x: Math.round(point.x / snapStep) * snapStep,
        y: Math.round(point.y / snapStep) * snapStep,
      },
      frame,
    );
  }

  function seedCombatPreset(preset: CombatPreset) {
    if (!activeFrame) {
      return;
    }
    const scale = manifest?.scale ?? 1;
    const metrics = activeBodyMetrics ?? {
      width: 76,
      standing_height: 168,
      crouch_height: 96,
    };
    updateActiveFrame((frame) => {
      frame.combat ??= {};
      const width = metrics.width / scale;
      const standing = metrics.standing_height / scale;
      const crouch = metrics.crouch_height / scale;
      const centerX = frame.pivot.x;
      const floorY = frame.pivot.y;

      if (preset === "clear_frame") {
        frame.combat = {};
        return;
      }
      if (preset === "standing_hurt") {
        frame.combat.hurtboxes = [
          clampRect(
            {
              x: centerX - width / 2,
              y: floorY - standing,
              w: width,
              h: standing,
              label: "body",
            },
            frame,
          ),
        ];
      }
      if (preset === "crouch_hurt") {
        frame.combat.hurtboxes = [
          clampRect(
            {
              x: centerX - width * 0.58,
              y: floorY - crouch,
              w: width * 1.16,
              h: crouch,
              label: "crouch_body",
            },
            frame,
          ),
        ];
      }
      if (preset === "mid_strike") {
        frame.combat.hitboxes = [
          clampRect(
            {
              x: centerX + width * 0.32,
              y: floorY - standing * 0.68,
              w: 54 / scale,
              h: 34 / scale,
              label: "mid_strike",
            },
            frame,
          ),
        ];
      }
      if (preset === "low_strike") {
        frame.combat.hitboxes = [
          clampRect(
            {
              x: centerX + width * 0.26,
              y: floorY - standing * 0.36,
              w: 68 / scale,
              h: 30 / scale,
              label: "low_strike",
            },
            frame,
          ),
        ];
      }
      if (preset === "projectile_origin") {
        frame.combat.projectile_origin = clampPoint(
          {
            x: centerX + width * 0.34,
            y: floorY - standing * 0.63,
          },
          frame,
        );
      }
    });
  }

  function beginStageEdit(event: React.PointerEvent<HTMLDivElement>) {
    if (!activeFrame) {
      return;
    }
    const rawPoint = localPointFromEvent(event, zoom);
    const point = snapPoint(rawPoint, activeFrame);
    setMouseLocal(rawPoint);

    const hit = hitTestFrame(activeFrame, rawPoint);
    if (hit) {
      beginDrag(event, hit, point);
      return;
    }

    if (editMode === "pivot") {
      const intent: DragIntent = { action: "move", target: { type: "pivot" } };
      const nextPivot = clampPoint(point, activeFrame);
      pushHistory();
      updateActiveFrame((frame) => {
        frame.pivot = nextPivot;
      }, { history: false });
      setSelection(intent.target);
      setDragState({
        ...intent,
        startMouse: point,
        startPivot: nextPivot,
      });
      event.currentTarget.setPointerCapture(event.pointerId);
      return;
    }

    if (editMode === "projectile") {
      const intent: DragIntent = {
        action: "move",
        target: { type: "projectile" },
      };
      const nextProjectile = clampPoint(point, activeFrame);
      pushHistory();
      updateActiveFrame((frame) => {
        frame.combat ??= {};
        frame.combat.projectile_origin = nextProjectile;
      }, { history: false });
      setSelection(intent.target);
      setDragState({
        ...intent,
        startMouse: point,
        startProjectile: nextProjectile,
      });
      event.currentTarget.setPointerCapture(event.pointerId);
      return;
    }

    if (editMode === "hurtbox" || editMode === "hitbox") {
      const kind = editMode === "hurtbox" ? "hurtboxes" : "hitboxes";
      let addedIndex = 0;
      updateActiveFrame((frame) => {
        frame.combat ??= {};
        frame.combat[kind] ??= [];
        const boxes = frame.combat[kind] ?? [];
        boxes.push(defaultBox(point.x, point.y, frame, editMode));
        addedIndex = boxes.length - 1;
      });
      setSelection({ type: "box", kind, index: addedIndex });
      return;
    }

    setSelection(null);
  }

  function beginDrag(
    event: React.PointerEvent<HTMLDivElement>,
    intent: DragIntent,
    point: SpritePoint,
  ) {
    if (!activeFrame) {
      return;
    }
    pushHistory();
    setSelection(intent.target);
    const base = {
      ...intent,
      startMouse: point,
      startPivot:
        intent.target.type === "pivot" ? { ...activeFrame.pivot } : undefined,
      startProjectile:
        intent.target.type === "projectile" &&
        activeFrame.combat?.projectile_origin
          ? { ...activeFrame.combat.projectile_origin }
          : undefined,
      startRect:
        intent.target.type === "box"
          ? {
              ...((activeFrame.combat?.[intent.target.kind] ?? [])[
                intent.target.index
              ] as SpriteRect),
            }
          : undefined,
    };
    setDragState(base);
    event.currentTarget.setPointerCapture(event.pointerId);
  }

  function continueStageEdit(event: React.PointerEvent<HTMLDivElement>) {
    if (!activeFrame) {
      return;
    }
    const rawPoint = localPointFromEvent(event, zoom);
    const point = snapPoint(rawPoint, activeFrame);
    setMouseLocal(rawPoint);
    if (!dragState) {
      setHoverIntent(hitTestFrame(activeFrame, rawPoint));
      return;
    }
    setHoverIntent(null);
    const delta = {
      x: point.x - dragState.startMouse.x,
      y: point.y - dragState.startMouse.y,
    };

    updateActiveFrame((frame) => {
      if (dragState.target.type === "pivot" && dragState.startPivot) {
        frame.pivot = clampPoint(
          {
            x: dragState.startPivot.x + delta.x,
            y: dragState.startPivot.y + delta.y,
          },
          frame,
        );
      }
      if (
        dragState.target.type === "projectile" &&
        dragState.startProjectile
      ) {
        frame.combat ??= {};
        frame.combat.projectile_origin = clampPoint(
          {
            x: dragState.startProjectile.x + delta.x,
            y: dragState.startProjectile.y + delta.y,
          },
          frame,
        );
      }
      if (dragState.target.type === "box" && dragState.startRect) {
        const boxes = frame.combat?.[dragState.target.kind];
        if (!boxes?.[dragState.target.index]) {
          return;
        }
        boxes[dragState.target.index] =
          dragState.action === "resize"
            ? resizeRect(dragState.startRect, dragState.handle, delta, frame)
            : moveRect(dragState.startRect, delta, frame);
      }
    }, { history: false });
  }

  function endStageEdit(event: React.PointerEvent<HTMLDivElement>) {
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
    setDragState(null);
  }

  function leaveStage() {
    setHoverIntent(null);
    setMouseLocal(null);
  }

  function updateBox(
    kind: BoxKind,
    index: number,
    patch: Partial<SpriteRect>,
  ) {
    updateActiveFrame((frame) => {
      const boxes = frame.combat?.[kind];
      if (!boxes?.[index]) {
        return;
      }
      boxes[index] = { ...boxes[index], ...patch };
    });
  }

  function updateBodyMetrics(patch: Partial<CharacterBodyMetrics>) {
    if (!bodyCatalog) {
      return;
    }
    const next = JSON.parse(JSON.stringify(bodyCatalog)) as BodyMetricsCatalog;
    let entry = next.characters.find((item) => item.id === selectedCharacterId);
    if (!entry) {
      entry = {
        id: selectedCharacterId,
        body: { width: 76, standing_height: 168, crouch_height: 96 },
      };
      next.characters.push(entry);
    }
    entry.body = { ...entry.body, ...patch };
    setBodyCatalog(next);
    setBodyDirty(true);
  }

  function removeBox(kind: BoxKind, index: number) {
    updateActiveFrame((frame) => {
      frame.combat?.[kind]?.splice(index, 1);
    });
    if (selection?.type === "box" && selection.kind === kind && selection.index === index) {
      setSelection(null);
    }
  }

  async function exportReviewImage() {
    if (!document?.atlas_data_url || !activeFrame) {
      return;
    }
    try {
      const dataUrl = await makeReviewImage(
        document.atlas_data_url,
        activeFrame,
        zoom,
        showBounds,
      );
      const link = window.document.createElement("a");
      link.href = dataUrl;
      link.download = `${activeFrame.name}-sprite-studio-review.png`;
      link.click();
      setStatus(`Exported ${link.download}`);
    } catch (error) {
      setStatus(errorMessage(error));
    }
  }

  const canvasWidth = activeFrame ? activeFrame.frame.w * zoom : 0;
  const canvasHeight = activeFrame ? activeFrame.frame.h * zoom : 0;
  const frameIndex = selectedFrame ? frameNames.indexOf(selectedFrame) : -1;
  const selectedKey = targetKey(selection);
  const activeBodyMetrics =
    bodyCatalog?.characters.find((entry) => entry.id === selectedCharacterId)
      ?.body ?? null;
  const stageCursor = cursorForIntent(dragState ?? hoverIntent, editMode);
  const workspaceClassName = [
    "workspace",
    leftPanelOpen ? "" : "left-collapsed",
    rightPanelOpen ? "" : "right-collapsed",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <main className="studio-shell">
      <header className="app-chrome">
        <MenuBar
          autosaveEnabled={autosaveEnabled}
          canRedo={redoStack.length > 0}
          canSave={dirty && !hasValidationErrors}
          canUndo={undoStack.length > 0}
          leftPanelOpen={leftPanelOpen}
          rightPanelOpen={rightPanelOpen}
          timelineOpen={timelineOpen}
          onExportPng={() => void exportReviewImage()}
          onExportReview={() => void exportReviewJson()}
          onHelp={() => setHelpOpen(true)}
          onOpen={() => void chooseManifestFile()}
          onRedo={redo}
          onReload={() => void loadManifest(manifestPath)}
          onRunValidation={() => void runRuntimeValidation()}
          onSave={() => void saveManifest()}
          onToggleAutosave={() => setAutosaveEnabled((enabled) => !enabled)}
          onToggleLeftPanel={() => setLeftPanelOpen((open) => !open)}
          onToggleRightPanel={() => setRightPanelOpen((open) => !open)}
          onToggleTimeline={() => setTimelineOpen((open) => !open)}
          onUndo={undo}
        />
        <div className="toolbar" aria-label="Main toolbar">
          <div className="toolbar-group">
            <button
              className="toolbar-button primary-action"
              type="button"
              onClick={() => void chooseManifestFile()}
            >
              <span className="toolbar-icon open-icon" aria-hidden="true" />
              Open
            </button>
            <button
              className="toolbar-button"
              disabled={!dirty || hasValidationErrors}
              type="button"
              onClick={() => void saveManifest()}
            >
              <span className="toolbar-icon save-icon" aria-hidden="true" />
              Save
            </button>
          </div>
          <div className="toolbar-divider" />
          <div className="toolbar-group">
            <button
              className="toolbar-button"
              disabled={!undoStack.length}
              type="button"
              title="Undo"
              onClick={undo}
            >
              <span className="toolbar-icon undo-icon" aria-hidden="true" />
              Undo
            </button>
            <button
              className="toolbar-button"
              disabled={!redoStack.length}
              type="button"
              title="Redo"
              onClick={redo}
            >
              <span className="toolbar-icon redo-icon" aria-hidden="true" />
              Redo
            </button>
          </div>
          <div className="toolbar-spacer" />
          <div className="toolbar-path" title={document?.repo_root ?? "Repository not loaded"}>
            {manifestPath}
          </div>
          <div className="toolbar-group toolbar-views">
            <button
              className={
                leftPanelOpen ? "toolbar-button toggle active" : "toolbar-button toggle"
              }
              type="button"
              title={leftPanelOpen ? "Hide browser panel" : "Show browser panel"}
              onClick={() => setLeftPanelOpen((open) => !open)}
            >
              <span className="toolbar-icon browser-icon" aria-hidden="true" />
              Browser
            </button>
            <button
              className={
                rightPanelOpen
                  ? "toolbar-button toggle active"
                  : "toolbar-button toggle"
              }
              type="button"
              title={rightPanelOpen ? "Hide inspector" : "Show inspector"}
              onClick={() => setRightPanelOpen((open) => !open)}
            >
              <span className="toolbar-icon inspector-icon" aria-hidden="true" />
              Inspector
            </button>
            <button
              className={
                timelineOpen ? "toolbar-button toggle active" : "toolbar-button toggle"
              }
              type="button"
              title={timelineOpen ? "Hide timeline" : "Show timeline"}
              onClick={() => setTimelineOpen((open) => !open)}
            >
              <span className="toolbar-icon timeline-icon" aria-hidden="true" />
              Timeline
            </button>
            <button
              className="toolbar-button"
              type="button"
              title="Help"
              onClick={() => setHelpOpen(true)}
            >
              <span className="toolbar-icon help-icon" aria-hidden="true" />
              Help
            </button>
          </div>
        </div>
      </header>

      <section className={workspaceClassName}>
        {leftPanelOpen && (
        <aside className="sidebar">
          <section className="panel">
            <h2>Manifest</h2>
            <label>
              Path
              <input
                value={manifestPath}
                onChange={(event) => setManifestPath(event.currentTarget.value)}
              />
            </label>
            <button onClick={() => void chooseManifestFile()}>
              Browse Manifest
            </button>
            <div className="preset-grid">
              {PRESET_MANIFESTS.map(([label, path]) => (
                <button key={path} onClick={() => void loadManifest(path)}>
                  {label}
                </button>
              ))}
            </div>
            <dl className="metadata">
              <dt>Atlas</dt>
              <dd>{document?.atlas_path ?? "-"}</dd>
              <dt>Schema</dt>
              <dd>{manifest?.schema ?? "-"}</dd>
              <dt>Image</dt>
              <dd>{manifest?.image ?? "-"}</dd>
            </dl>
          </section>

          <section className="panel">
            <h2>Clips</h2>
            <div className="clip-list">
              {manifest?.clips.map((clip) => (
                <button
                  className={clip.name === selectedClip ? "selected" : ""}
                  key={clip.name}
                  onClick={() => handleClipSelection(clip.name)}
                >
                  <span>{clip.name}</span>
                  <small>{clip.frames.length} frames</small>
                </button>
              ))}
            </div>
          </section>

          <section className="panel">
            <h2>Frames</h2>
            <div className="section-meta">
              {frameIndex >= 0 ? frameIndex + 1 : 0} / {frameNames.length}
            </div>
            <div className="frame-list">
              {frameNames.map((frameName) => (
                <button
                  className={frameName === selectedFrame ? "selected" : ""}
                  key={frameName}
                  onClick={() => {
                    setSelectedFrame(frameName);
                    setSelection(null);
                  }}
                >
                  {frameName}
                </button>
              ))}
            </div>
          </section>
        </aside>
        )}

        <section className="stage-column">
          <div className="toolstrip">
            <div className="segmented">
              {EDIT_MODES.map((mode) => (
                <button
                  className={mode.id === editMode ? "selected" : ""}
                  key={mode.id}
                  onClick={() => setEditMode(mode.id)}
                >
                  {mode.label}
                </button>
              ))}
            </div>
            <div className="view-toggles">
              <label>
                <input
                  checked={showGrid}
                  type="checkbox"
                  onChange={(event) => setShowGrid(event.currentTarget.checked)}
                />
                Grid
              </label>
              <label>
                <input
                  checked={showBounds}
                  type="checkbox"
                  onChange={(event) => setShowBounds(event.currentTarget.checked)}
                />
                Bounds
              </label>
              <label>
                <input
                  checked={showScaleGuide}
                  type="checkbox"
                  onChange={(event) =>
                    setShowScaleGuide(event.currentTarget.checked)
                  }
                />
                Scale guide
              </label>
              <label>
                <input
                  checked={snapEnabled}
                  type="checkbox"
                  onChange={(event) => setSnapEnabled(event.currentTarget.checked)}
                />
                Snap
              </label>
              <label className="snap-step">
                Step
                <select
                  disabled={!snapEnabled}
                  value={snapStep}
                  onChange={(event) =>
                    setSnapStep(toNumber(event.currentTarget.value, 4))
                  }
                >
                  <option value="1">1 px</option>
                  <option value="2">2 px</option>
                  <option value="4">4 px</option>
                  <option value="8">8 px</option>
                  <option value="16">16 px</option>
                </select>
              </label>
            </div>
            <label className="inline-field">
              Zoom
              <input
                min="0.5"
                max="5"
                step="0.25"
                type="range"
                value={zoom}
                onChange={(event) =>
                  setZoom(toNumber(event.currentTarget.value, 2))
                }
              />
              <span>{zoom.toFixed(2)}x</span>
            </label>
          </div>

          <div className="stage-scroll">
            {activeFrame && (
              <div
                className="frame-stage"
                onPointerCancel={endStageEdit}
                onPointerDown={beginStageEdit}
                onPointerLeave={leaveStage}
                onPointerMove={continueStageEdit}
                onPointerUp={endStageEdit}
                onWheel={handleStageWheel}
                style={{
                  width: `${canvasWidth}px`,
                  height: `${canvasHeight}px`,
                  cursor: stageCursor,
                }}
              >
                <canvas ref={canvasRef} />
                <svg
                  className="frame-overlay"
                  style={{ width: canvasWidth, height: canvasHeight }}
                  viewBox={`0 0 ${activeFrame.frame.w} ${activeFrame.frame.h}`}
                >
                  <defs>
                    <pattern
                      height="16"
                      id="grid"
                      patternUnits="userSpaceOnUse"
                      width="16"
                    >
                      <path d="M 16 0 L 0 0 0 16" />
                    </pattern>
                  </defs>
                  {showGrid && (
                    <rect
                      className="grid-fill"
                      height={activeFrame.frame.h}
                      width={activeFrame.frame.w}
                      x="0"
                      y="0"
                    />
                  )}
                  {showBounds && renderRect(activeFrame.trimmed_bounds, "trimmed")}
                  {showBounds && renderRect(activeFrame.source_crop, "source")}
                  {showScaleGuide &&
                    renderScaleGuide(activeFrame, manifest?.scale ?? 1)}
                  {activeFrame.combat?.hurtboxes?.map((box, index) =>
                    renderEditableRect(
                      box,
                      "hurt",
                      { type: "box", kind: "hurtboxes", index },
                      selectedKey,
                    ),
                  )}
                  {activeFrame.combat?.hitboxes?.map((box, index) =>
                    renderEditableRect(
                      box,
                      "hit",
                      { type: "box", kind: "hitboxes", index },
                      selectedKey,
                    ),
                  )}
                  <line
                    className="pivot-line"
                    x1={activeFrame.pivot.x}
                    x2={activeFrame.pivot.x}
                    y1="0"
                    y2={activeFrame.frame.h}
                  />
                  <line
                    className="pivot-line"
                    x1="0"
                    x2={activeFrame.frame.w}
                    y1={activeFrame.pivot.y}
                    y2={activeFrame.pivot.y}
                  />
                  <circle
                    className={
                      selectedKey === "pivot"
                        ? "pivot-point selected-point"
                        : "pivot-point"
                    }
                    cx={activeFrame.pivot.x}
                    cy={activeFrame.pivot.y}
                    r="4"
                  />
                  {activeFrame.combat?.projectile_origin && (
                    <g
                      className={
                        selectedKey === "projectile"
                          ? "projectile-origin selected-point"
                          : "projectile-origin"
                      }
                    >
                      <line
                        x1={activeFrame.combat.projectile_origin.x - 12}
                        x2={activeFrame.combat.projectile_origin.x + 12}
                        y1={activeFrame.combat.projectile_origin.y}
                        y2={activeFrame.combat.projectile_origin.y}
                      />
                      <line
                        x1={activeFrame.combat.projectile_origin.x}
                        x2={activeFrame.combat.projectile_origin.x}
                        y1={activeFrame.combat.projectile_origin.y - 12}
                        y2={activeFrame.combat.projectile_origin.y + 12}
                      />
                      <circle
                        cx={activeFrame.combat.projectile_origin.x}
                        cy={activeFrame.combat.projectile_origin.y}
                        r="5"
                      />
                    </g>
                  )}
                </svg>
              </div>
            )}
          </div>

          {timelineOpen && (
            <Timeline
              clipName={selectedClip}
              frameIndex={Math.max(0, frameIndex)}
              frames={timelineFrames}
              onNext={() => moveFrame(1)}
              onPrevious={() => moveFrame(-1)}
              onSelect={(frameName) => {
                setSelectedFrame(frameName);
                setSelection(null);
              }}
            />
          )}

          <footer className="statusbar">
            <span>{status}</span>
            <span>{dirty ? "Unsaved changes" : "Clean"}</span>
            <span>{autosaveEnabled ? "Autosave on" : "Autosave off"}</span>
            <span>autosave {lastAutosavePath ?? "-"}</span>
            <span>backup {lastBackupPath ?? "-"}</span>
            <span>
              selected {selectedKey || "-"} | local{" "}
              {formatCoordinate(mouseLocal?.x ?? null)},{" "}
              {formatCoordinate(mouseLocal?.y ?? null)}
            </span>
          </footer>
        </section>

        {rightPanelOpen && (
        <aside className="inspector">
          <section className="panel">
            <h2>Frame Data</h2>
            {activeFrame ? (
              <div className="property-grid">
                <label>
                  Manifest scale
                  <input
                    step="0.01"
                    type="number"
                    value={manifest?.scale ?? 1}
                    onChange={(event) =>
                      updateManifest((next) => {
                        next.scale = toNumber(event.currentTarget.value, 1);
                      })
                    }
                  />
                </label>
                <label>
                  Pivot X
                  <input
                    type="number"
                    value={activeFrame.pivot.x}
                    onChange={(event) =>
                      updateActiveFrame((frame) => {
                        frame.pivot.x = Math.round(
                          toNumber(event.currentTarget.value, frame.pivot.x),
                        );
                      })
                    }
                  />
                </label>
                <label>
                  Pivot Y
                  <input
                    type="number"
                    value={activeFrame.pivot.y}
                    onChange={(event) =>
                      updateActiveFrame((frame) => {
                        frame.pivot.y = Math.round(
                          toNumber(event.currentTarget.value, frame.pivot.y),
                        );
                      })
                    }
                  />
                </label>
                <label>
                  Duration ms
                  <input
                    type="number"
                    value={activeFrame.duration_ms}
                    onChange={(event) =>
                      updateActiveFrame((frame) => {
                        frame.duration_ms = Math.max(
                          1,
                          Math.round(
                            toNumber(
                              event.currentTarget.value,
                              frame.duration_ms,
                            ),
                          ),
                        );
                      })
                    }
                  />
                </label>
              </div>
            ) : (
              <p>No frame selected.</p>
            )}
          </section>

          <ScalePanel summary={scaleSummary} />

          <FrameCombatSummary activeFrame={activeFrame} />

          <BodyMetricsPanel
            bodyDirty={bodyDirty}
            bodyMetrics={activeBodyMetrics}
            bodyMetricsPath={bodyMetricsPath}
            catalog={bodyCatalog}
            onCharacterChange={setSelectedCharacterId}
            onReload={() => void loadBodyMetrics(bodyMetricsPath)}
            onSave={() => void saveBodyMetrics()}
            onUpdate={updateBodyMetrics}
            selectedCharacterId={selectedCharacterId}
          />

          <CombatPresetPanel
            disabled={!activeFrame}
            onSeed={seedCombatPreset}
          />

          <section className="panel">
            <h2>Projectile Origin</h2>
            <div className="property-grid">
              <label>
                X
                <input
                  disabled={!activeFrame}
                  type="number"
                  value={activeFrame?.combat?.projectile_origin?.x ?? ""}
                  onChange={(event) =>
                    updateActiveFrame((frame) => {
                      frame.combat ??= {};
                      frame.combat.projectile_origin ??= { x: 0, y: 0 };
                      frame.combat.projectile_origin.x = Math.round(
                        toNumber(event.currentTarget.value, 0),
                      );
                    })
                  }
                />
              </label>
              <label>
                Y
                <input
                  disabled={!activeFrame}
                  type="number"
                  value={activeFrame?.combat?.projectile_origin?.y ?? ""}
                  onChange={(event) =>
                    updateActiveFrame((frame) => {
                      frame.combat ??= {};
                      frame.combat.projectile_origin ??= { x: 0, y: 0 };
                      frame.combat.projectile_origin.y = Math.round(
                        toNumber(event.currentTarget.value, 0),
                      );
                    })
                  }
                />
              </label>
            </div>
            <button
              disabled={!activeFrame?.combat?.projectile_origin}
              onClick={() =>
                updateActiveFrame((frame) => {
                  delete frame.combat?.projectile_origin;
                })
              }
            >
              Clear Projectile Origin
            </button>
          </section>

          <BoxEditor
            activeFrame={activeFrame}
            kind="hurtboxes"
            onRemove={removeBox}
            onSelect={(target) => setSelection(target)}
            onUpdate={updateBox}
            selection={selection}
            title="Hurtboxes"
          />
          <BoxEditor
            activeFrame={activeFrame}
            kind="hitboxes"
            onRemove={removeBox}
            onSelect={(target) => setSelection(target)}
            onUpdate={updateBox}
            selection={selection}
            title="Hitboxes"
          />

          <ValidationPanel
            issues={validationIssues}
            onRunRuntime={() => void runRuntimeValidation()}
            runtimeResult={runtimeValidation}
            runtimeRunning={runtimeValidationRunning}
          />

          <ReviewPanel
            notes={reviewNotes}
            onExport={() => void exportReviewJson()}
            onNotesChange={setReviewNotes}
          />

          <section className="panel json-panel">
            <h2>JSON Preview</h2>
            <textarea readOnly value={manifestJson} />
          </section>
        </aside>
        )}
      </section>
      <HelpCenter open={helpOpen} onClose={() => setHelpOpen(false)} />
    </main>
  );
}

function FrameCombatSummary({
  activeFrame,
}: {
  activeFrame: SpriteFrame | null;
}) {
  const hurtboxes = activeFrame?.combat?.hurtboxes ?? [];
  const hitboxes = activeFrame?.combat?.hitboxes ?? [];
  const projectileOrigin = activeFrame?.combat?.projectile_origin ?? null;
  const hasCombat =
    hurtboxes.length > 0 || hitboxes.length > 0 || projectileOrigin !== null;

  return (
    <section className="panel combat-summary">
      <h2>Frame Combat</h2>
      {activeFrame ? (
        <>
          <dl className="summary-list">
            <div>
              <dt>Hurtboxes</dt>
              <dd>{hurtboxes.length}</dd>
            </div>
            <div>
              <dt>Hitboxes</dt>
              <dd>{hitboxes.length}</dd>
            </div>
            <div>
              <dt>Projectile origin</dt>
              <dd>{projectileOrigin ? "yes" : "no"}</dd>
            </div>
          </dl>
          {!hasCombat && (
            <p className="summary-note">
              No frame metadata here. The game uses runtime fallback boxes for
              this frame.
            </p>
          )}
          {projectileOrigin && hitboxes.length === 0 && (
            <p className="summary-note">
              Projectile damage boxes are still defined by the Rust
              ProjectileSpec. This frame only stores where the projectile starts.
            </p>
          )}
        </>
      ) : (
        <p>No frame selected.</p>
      )}
    </section>
  );
}

function BodyMetricsPanel({
  bodyDirty,
  bodyMetrics,
  bodyMetricsPath,
  catalog,
  onCharacterChange,
  onReload,
  onSave,
  onUpdate,
  selectedCharacterId,
}: {
  bodyDirty: boolean;
  bodyMetrics: CharacterBodyMetrics | null;
  bodyMetricsPath: string;
  catalog: BodyMetricsCatalog | null;
  onCharacterChange: (id: string) => void;
  onReload: () => void;
  onSave: () => void;
  onUpdate: (patch: Partial<CharacterBodyMetrics>) => void;
  selectedCharacterId: string;
}) {
  const characterIds =
    catalog?.characters.map((entry) => entry.id) ?? ["rust", "duke", "go", "c"];
  return (
    <section className="panel">
      <h2>Body Metrics</h2>
      <dl className="metadata">
        <dt>File</dt>
        <dd>{bodyMetricsPath}</dd>
      </dl>
      <label>
        Character
        <select
          value={selectedCharacterId}
          onChange={(event) => onCharacterChange(event.currentTarget.value)}
        >
          {characterIds.map((id) => (
            <option key={id} value={id}>
              {id}
            </option>
          ))}
        </select>
      </label>
      {bodyMetrics ? (
        <div className="property-grid">
          <label>
            Width
            <input
              type="number"
              value={bodyMetrics.width}
              onChange={(event) =>
                onUpdate({ width: toNumber(event.currentTarget.value, 76) })
              }
            />
          </label>
          <label>
            Standing
            <input
              type="number"
              value={bodyMetrics.standing_height}
              onChange={(event) =>
                onUpdate({
                  standing_height: toNumber(event.currentTarget.value, 168),
                })
              }
            />
          </label>
          <label>
            Crouch
            <input
              type="number"
              value={bodyMetrics.crouch_height}
              onChange={(event) =>
                onUpdate({
                  crouch_height: toNumber(event.currentTarget.value, 96),
                })
              }
            />
          </label>
        </div>
      ) : (
        <p>No metrics found for this character.</p>
      )}
      <div className="row-actions">
        <button onClick={onReload}>Reload Metrics</button>
        <button disabled={!bodyDirty} onClick={onSave}>
          Save Metrics
        </button>
      </div>
    </section>
  );
}

function ScalePanel({
  summary,
}: {
  summary: ReturnType<typeof visualScaleSummary>;
}) {
  return (
    <section className="panel">
      <h2>Visual Scale</h2>
      {summary ? (
        <dl className="metadata">
          <dt>Visible width</dt>
          <dd className={summary.widthOk ? "ok" : "warning"}>
            {summary.width.toFixed(1)} px
          </dd>
          <dt>Visible height</dt>
          <dd className={summary.heightOk ? "ok" : "warning"}>
            {summary.height.toFixed(1)} px
          </dd>
          <dt>Target</dt>
          <dd>110-150 px wide, 185-210 px tall</dd>
          <dt>Status</dt>
          <dd>{summary.message}</dd>
        </dl>
      ) : (
        <p>No `trimmed_bounds` or `source_crop` data on this frame.</p>
      )}
    </section>
  );
}

function CombatPresetPanel({
  disabled,
  onSeed,
}: {
  disabled: boolean;
  onSeed: (preset: CombatPreset) => void;
}) {
  const presets: Array<{ id: CombatPreset; label: string }> = [
    { id: "standing_hurt", label: "Standing Hurtbox" },
    { id: "crouch_hurt", label: "Crouch Hurtbox" },
    { id: "mid_strike", label: "Mid Strike" },
    { id: "low_strike", label: "Low Strike" },
    { id: "projectile_origin", label: "Projectile Origin" },
    { id: "clear_frame", label: "Clear Combat Data" },
  ];
  return (
    <section className="panel">
      <h2>Combat Presets</h2>
      <div className="preset-grid">
        {presets.map((preset) => (
          <button
            disabled={disabled}
            key={preset.id}
            onClick={() => onSeed(preset.id)}
          >
            {preset.label}
          </button>
        ))}
      </div>
    </section>
  );
}

function ValidationPanel({
  issues,
  onRunRuntime,
  runtimeResult,
  runtimeRunning,
}: {
  issues: ValidationIssue[];
  onRunRuntime: () => void;
  runtimeResult: RuntimeValidationResult | null;
  runtimeRunning: boolean;
}) {
  const errors = issues.filter((issue) => issue.severity === "error").length;
  const warnings = issues.filter((issue) => issue.severity === "warning").length;
  return (
    <section className="panel">
      <h2>Validation</h2>
      <p>
        {errors} errors, {warnings} warnings
      </p>
      <div className="issue-list">
        {issues.map((issue, index) => (
          <div className={`issue ${issue.severity}`} key={`${issue.message}-${index}`}>
            <strong>{issue.severity}</strong>
            <span>
              {issue.frameName ? `${issue.frameName}: ` : ""}
              {issue.message}
            </span>
          </div>
        ))}
      </div>
      <button disabled={runtimeRunning} onClick={onRunRuntime}>
        {runtimeRunning ? "Running..." : "Run Runtime Validation"}
      </button>
      {runtimeResult && (
        <div
          className={
            runtimeResult.success
              ? "runtime-validation ok-panel"
              : "runtime-validation error-panel"
          }
        >
          <strong>{runtimeResult.success ? "Runtime OK" : "Runtime Failed"}</strong>
          <code>{runtimeResult.command}</code>
          {(runtimeResult.stderr || runtimeResult.stdout) && (
            <pre>
              {(runtimeResult.stderr || runtimeResult.stdout).slice(0, 1800)}
            </pre>
          )}
        </div>
      )}
    </section>
  );
}

function ReviewPanel({
  notes,
  onExport,
  onNotesChange,
}: {
  notes: string;
  onExport: () => void;
  onNotesChange: (notes: string) => void;
}) {
  return (
    <section className="panel">
      <h2>Review Package</h2>
      <label>
        Notes
        <textarea
          className="review-notes"
          placeholder="Context for art/combat review."
          value={notes}
          onChange={(event) => onNotesChange(event.currentTarget.value)}
        />
      </label>
      <button onClick={onExport}>Export Review JSON</button>
    </section>
  );
}

function BoxEditor({
  activeFrame,
  kind,
  onRemove,
  onSelect,
  onUpdate,
  selection,
  title,
}: {
  activeFrame: SpriteFrame | null;
  kind: BoxKind;
  onRemove: (kind: BoxKind, index: number) => void;
  onSelect: (target: SelectionTarget) => void;
  onUpdate: (kind: BoxKind, index: number, patch: Partial<SpriteRect>) => void;
  selection: SelectionTarget | null;
  title: string;
}) {
  const boxes = activeFrame?.combat?.[kind] ?? [];

  return (
    <section className="panel">
      <h2>{title}</h2>
      {boxes.length === 0 && <p>No boxes on this frame.</p>}
      {boxes.map((box, index) => {
        const selected =
          selection?.type === "box" &&
          selection.kind === kind &&
          selection.index === index;
        return (
          <div
            className={selected ? "box-card selected-card" : "box-card"}
            key={`${kind}-${index}`}
          >
            <div className="box-card-title">
              <button onClick={() => onSelect({ type: "box", kind, index })}>
                {box.label || `${kind}-${index + 1}`}
              </button>
              <button onClick={() => onRemove(kind, index)}>Remove</button>
            </div>
            <label>
              Label
              <input
                value={box.label ?? ""}
                onChange={(event) =>
                  onUpdate(kind, index, { label: event.currentTarget.value })
                }
              />
            </label>
            <div className="quad-input">
              {(["x", "y", "w", "h"] as const).map((field) => (
                <label key={field}>
                  {field.toUpperCase()}
                  <input
                    type="number"
                    value={box[field]}
                    onChange={(event) =>
                      onUpdate(kind, index, {
                        [field]: Math.round(
                          toNumber(event.currentTarget.value, box[field]),
                        ),
                      })
                    }
                  />
                </label>
              ))}
            </div>
          </div>
        );
      })}
    </section>
  );
}

function renderEditableRect(
  rect: SpriteRect,
  className: string,
  target: Extract<SelectionTarget, { type: "box" }>,
  selectedKey: string,
) {
  const selected = targetKey(target) === selectedKey;
  return (
    <g
      className={`overlay-rect ${className} ${selected ? "selected-box" : ""}`}
      key={targetKey(target)}
    >
      <rect height={rect.h} width={rect.w} x={rect.x} y={rect.y} />
      {rect.label && (
        <text x={rect.x + 4} y={Math.max(12, rect.y + 12)}>
          {rect.label}
        </text>
      )}
      {selected &&
        handlePoints(rect).map(({ handle, point }) => (
          <rect
            className="resize-handle"
            height="6"
            key={handle}
            width="6"
            x={point.x - 3}
            y={point.y - 3}
          />
        ))}
    </g>
  );
}

function renderRect(rect: SpriteRect | undefined, className: string) {
  if (!rect) {
    return null;
  }
  return (
    <g className={`overlay-rect ${className}`}>
      <rect height={rect.h} width={rect.w} x={rect.x} y={rect.y} />
      {rect.label && (
        <text x={rect.x + 4} y={Math.max(12, rect.y + 12)}>
          {rect.label}
        </text>
      )}
    </g>
  );
}

function renderScaleGuide(frame: SpriteFrame, scale: number) {
  const safeScale = scale > 0 ? scale : 1;
  const targetMin = {
    w: 110 / safeScale,
    h: 185 / safeScale,
  };
  const targetMax = {
    w: 150 / safeScale,
    h: 210 / safeScale,
  };
  const minRect = {
    x: frame.pivot.x - targetMin.w / 2,
    y: frame.pivot.y - targetMin.h,
    w: targetMin.w,
    h: targetMin.h,
  };
  const maxRect = {
    x: frame.pivot.x - targetMax.w / 2,
    y: frame.pivot.y - targetMax.h,
    w: targetMax.w,
    h: targetMax.h,
  };
  return (
    <g className="scale-guide">
      <rect
        className="scale-guide-max"
        height={maxRect.h}
        width={maxRect.w}
        x={maxRect.x}
        y={maxRect.y}
      />
      <rect
        className="scale-guide-min"
        height={minRect.h}
        width={minRect.w}
        x={minRect.x}
        y={minRect.y}
      />
      <text x={maxRect.x + 4} y={Math.max(12, maxRect.y + 12)}>
        target visual range
      </text>
    </g>
  );
}

function drawFrame(
  atlasDataUrl: string | undefined,
  frame: SpriteFrame | null,
  zoom: number,
  canvas: HTMLCanvasElement | null,
) {
  if (!atlasDataUrl || !frame || !canvas) {
    return;
  }

  const context = canvas.getContext("2d");
  if (!context) {
    return;
  }

  const image = new Image();
  image.onload = () => {
    canvas.width = Math.round(frame.frame.w * zoom);
    canvas.height = Math.round(frame.frame.h * zoom);
    context.clearRect(0, 0, canvas.width, canvas.height);
    context.imageSmoothingEnabled = false;
    context.drawImage(
      image,
      frame.frame.x,
      frame.frame.y,
      frame.frame.w,
      frame.frame.h,
      0,
      0,
      canvas.width,
      canvas.height,
    );
  };
  image.src = atlasDataUrl;
}

function makeReviewImage(
  atlasDataUrl: string,
  frame: SpriteFrame,
  zoom: number,
  showBounds: boolean,
): Promise<string> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => {
      const width = Math.round(frame.frame.w * zoom);
      const height = Math.round(frame.frame.h * zoom);
      const canvas = window.document.createElement("canvas");
      canvas.width = width;
      canvas.height = height;
      const context = canvas.getContext("2d");
      if (!context) {
        reject(new Error("Could not create export canvas."));
        return;
      }
      context.imageSmoothingEnabled = false;
      context.drawImage(
        image,
        frame.frame.x,
        frame.frame.y,
        frame.frame.w,
        frame.frame.h,
        0,
        0,
        width,
        height,
      );
      drawReviewOverlay(context, frame, zoom, showBounds);
      resolve(canvas.toDataURL("image/png"));
    };
    image.onerror = () => reject(new Error("Could not load atlas image."));
    image.src = atlasDataUrl;
  });
}

function drawReviewOverlay(
  context: CanvasRenderingContext2D,
  frame: SpriteFrame,
  zoom: number,
  showBounds: boolean,
) {
  context.save();
  context.scale(zoom, zoom);
  context.lineWidth = 2 / zoom;
  if (showBounds) {
    strokeRect(context, frame.trimmed_bounds, "#6cc7ff", [6, 4]);
    strokeRect(context, frame.source_crop, "#ffd875", [3, 3]);
  }
  for (const box of frame.combat?.hurtboxes ?? []) {
    fillStrokeRect(context, box, "rgba(85,209,124,0.22)", "#66ff99");
  }
  for (const box of frame.combat?.hitboxes ?? []) {
    fillStrokeRect(context, box, "rgba(244,106,98,0.24)", "#ff8b83");
  }
  context.setLineDash([]);
  context.strokeStyle = "#f9c74f";
  context.beginPath();
  context.moveTo(frame.pivot.x, 0);
  context.lineTo(frame.pivot.x, frame.frame.h);
  context.moveTo(0, frame.pivot.y);
  context.lineTo(frame.frame.w, frame.pivot.y);
  context.stroke();
  context.fillStyle = "#f9c74f";
  context.beginPath();
  context.arc(frame.pivot.x, frame.pivot.y, 4, 0, Math.PI * 2);
  context.fill();
  const origin = frame.combat?.projectile_origin;
  if (origin) {
    context.strokeStyle = "#d7c7ff";
    context.beginPath();
    context.moveTo(origin.x - 12, origin.y);
    context.lineTo(origin.x + 12, origin.y);
    context.moveTo(origin.x, origin.y - 12);
    context.lineTo(origin.x, origin.y + 12);
    context.stroke();
  }
  context.restore();
}

function strokeRect(
  context: CanvasRenderingContext2D,
  rect: SpriteRect | undefined,
  strokeStyle: string,
  dash: number[],
) {
  if (!rect) {
    return;
  }
  context.setLineDash(dash);
  context.strokeStyle = strokeStyle;
  context.strokeRect(rect.x, rect.y, rect.w, rect.h);
}

function fillStrokeRect(
  context: CanvasRenderingContext2D,
  rect: SpriteRect,
  fillStyle: string,
  strokeStyle: string,
) {
  context.fillStyle = fillStyle;
  context.strokeStyle = strokeStyle;
  context.fillRect(rect.x, rect.y, rect.w, rect.h);
  context.strokeRect(rect.x, rect.y, rect.w, rect.h);
}

function errorMessage(error: unknown): string {
  if (typeof error === "object" && error && "message" in error) {
    return String((error as { message: unknown }).message);
  }
  return String(error);
}

function cursorForIntent(intent: DragIntent | null, editMode: EditMode): string {
  if (intent?.action === "resize") {
    const cursors: Record<string, string> = {
      n: "ns-resize",
      s: "ns-resize",
      e: "ew-resize",
      w: "ew-resize",
      ne: "nesw-resize",
      sw: "nesw-resize",
      nw: "nwse-resize",
      se: "nwse-resize",
    };
    return cursors[intent.handle] ?? "grab";
  }
  if (intent?.action === "move") {
    return "grab";
  }
  if (editMode === "inspect") {
    return "default";
  }
  return "crosshair";
}

function dialogPath(value: string | string[] | null): string | null {
  if (!value) {
    return null;
  }
  return Array.isArray(value) ? value[0] ?? null : value;
}

export default App;
