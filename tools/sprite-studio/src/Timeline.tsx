import type { SpriteFrame } from "./types";

type TimelineFrame = SpriteFrame & {
  selected: boolean;
};

type TimelineProps = {
  clipName: string | null;
  frameIndex: number;
  frames: TimelineFrame[];
  onNext: () => void;
  onPrevious: () => void;
  onSelect: (frameName: string) => void;
};

export function Timeline({
  clipName,
  frameIndex,
  frames,
  onNext,
  onPrevious,
  onSelect,
}: TimelineProps) {
  const totalDuration = frames.reduce(
    (duration, frame) => duration + frame.duration_ms,
    0,
  );
  const frameWidths = frames.map((frame) =>
    Math.max(56, Math.min(164, frame.duration_ms * 0.78)),
  );
  const columns = frames
    .map((_, index) => `${frameWidths[index]}px`)
    .join(" ");
  const selectedIndex = frames.findIndex((frame) => frame.selected);
  const playheadX =
    selectedIndex >= 0
      ? frameWidths
          .slice(0, selectedIndex)
          .reduce((offset, width) => offset + width, 0) +
        frameWidths[selectedIndex] / 2
      : 0;
  const stripWidth = frameWidths.reduce((total, width) => total + width, 12);

  return (
    <section className="timeline-panel" aria-label="Clip timeline">
      <div className="timeline-header">
        <div className="timeline-summary">
          <strong>{clipName ?? "No clip"}</strong>
          <span>
            {frames.length ? frameIndex + 1 : 0} / {frames.length} frames ·{" "}
            {totalDuration} ms
          </span>
        </div>
        <div className="timeline-transport" aria-label="Frame transport">
          <button
            className="transport-button"
            type="button"
            title="Previous frame"
            onClick={onPrevious}
          >
            <span aria-hidden="true">|&lt;</span>
            <span className="visually-hidden">Previous frame</span>
          </button>
          <span className="timeline-current-frame">
            {frames.length ? frameIndex + 1 : 0}
          </span>
          <button
            className="transport-button"
            type="button"
            title="Next frame"
            onClick={onNext}
          >
            <span aria-hidden="true">&gt;|</span>
            <span className="visually-hidden">Next frame</span>
          </button>
        </div>
      </div>
      <div className="timeline-editor">
        <div
          className="timeline-strip"
          style={{ minWidth: `${Math.max(stripWidth, 1)}px` }}
        >
          {frames.length > 0 && (
            <span
              className="timeline-playhead"
              style={{ left: `${playheadX}px` }}
            />
          )}
          <div className="timeline-ruler" style={{ gridTemplateColumns: columns }}>
            {frames.map((frame, index) => (
              <span key={`${frame.name}-ruler`}>
                <i />
                {index + 1}
              </span>
            ))}
          </div>
          <div className="timeline-track" style={{ gridTemplateColumns: columns }}>
            {frames.map((frame, index) => (
              <button
                className={
                  frame.selected ? "timeline-frame active" : "timeline-frame"
                }
                key={frame.name}
                type="button"
                onClick={() => onSelect(frame.name)}
              >
                <span className="timeline-frame-index">F{index + 1}</span>
                <span className="timeline-frame-name">{frame.name}</span>
                <span className="timeline-frame-duration">
                  {frame.duration_ms} ms
                </span>
                <span className="timeline-markers">
                  {(frame.combat?.hurtboxes?.length ?? 0) > 0 && (
                    <i className="marker hurt" title="Hurtbox" />
                  )}
                  {(frame.combat?.hitboxes?.length ?? 0) > 0 && (
                    <i className="marker hit" title="Hitbox" />
                  )}
                  {frame.combat?.projectile_origin && (
                    <i className="marker projectile" title="Projectile origin" />
                  )}
                </span>
              </button>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}
