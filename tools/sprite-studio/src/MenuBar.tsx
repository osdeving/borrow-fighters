import { useState } from "react";

type MenuBarProps = {
  autosaveEnabled: boolean;
  canRedo: boolean;
  canSave: boolean;
  canUndo: boolean;
  leftPanelOpen: boolean;
  rightPanelOpen: boolean;
  timelineOpen: boolean;
  onExportPng: () => void;
  onExportReview: () => void;
  onHelp: () => void;
  onOpen: () => void;
  onRedo: () => void;
  onReload: () => void;
  onRunValidation: () => void;
  onSave: () => void;
  onToggleAutosave: () => void;
  onToggleLeftPanel: () => void;
  onToggleRightPanel: () => void;
  onToggleTimeline: () => void;
  onUndo: () => void;
};

type MenuKey = "file" | "edit" | "view" | "help";

export function MenuBar({
  autosaveEnabled,
  canRedo,
  canSave,
  canUndo,
  leftPanelOpen,
  rightPanelOpen,
  timelineOpen,
  onExportPng,
  onExportReview,
  onHelp,
  onOpen,
  onRedo,
  onReload,
  onRunValidation,
  onSave,
  onToggleAutosave,
  onToggleLeftPanel,
  onToggleRightPanel,
  onToggleTimeline,
  onUndo,
}: MenuBarProps) {
  const [openMenu, setOpenMenu] = useState<MenuKey | null>(null);

  function runMenuAction(action: () => void) {
    action();
    setOpenMenu(null);
  }

  return (
    <nav
      className="menu-bar"
      aria-label="Application menu"
      onMouseLeave={() => setOpenMenu(null)}
    >
      <div
        className="menu-logo"
        role="img"
        aria-label="Borrow Fighters"
        title="Borrow Fighters"
      >
        <span className="logo-mark-cut" />
        <span className="logo-mark-core" />
        <span className="logo-mark-spark" />
      </div>

      <div
        className={openMenu === "file" ? "menu-group open" : "menu-group"}
        onMouseEnter={() => setOpenMenu("file")}
      >
        <button
          className="menu-trigger"
          type="button"
          aria-haspopup="menu"
          aria-expanded={openMenu === "file"}
          onClick={() => setOpenMenu(openMenu === "file" ? null : "file")}
        >
          File
        </button>
        {openMenu === "file" && (
          <div className="menu-dropdown" role="menu">
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onOpen)}
            >
              Open Manifest...
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onReload)}
            >
              Reload
            </button>
            <button
              disabled={!canSave}
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onSave)}
            >
              Save JSON
              <kbd>Ctrl+S</kbd>
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onExportPng)}
            >
              Export PNG Review
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onExportReview)}
            >
              Export Review JSON
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onRunValidation)}
            >
              Run Runtime Validation
            </button>
          </div>
        )}
      </div>

      <div
        className={openMenu === "edit" ? "menu-group open" : "menu-group"}
        onMouseEnter={() => setOpenMenu("edit")}
      >
        <button
          className="menu-trigger"
          type="button"
          aria-haspopup="menu"
          aria-expanded={openMenu === "edit"}
          onClick={() => setOpenMenu(openMenu === "edit" ? null : "edit")}
        >
          Edit
        </button>
        {openMenu === "edit" && (
          <div className="menu-dropdown" role="menu">
            <button
              disabled={!canUndo}
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onUndo)}
            >
              Undo
              <kbd>Ctrl+Z</kbd>
            </button>
            <button
              disabled={!canRedo}
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onRedo)}
            >
              Redo
              <kbd>Ctrl+Y</kbd>
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onToggleAutosave)}
            >
              {autosaveEnabled ? "Disable Autosave" : "Enable Autosave"}
            </button>
          </div>
        )}
      </div>

      <div
        className={openMenu === "view" ? "menu-group open" : "menu-group"}
        onMouseEnter={() => setOpenMenu("view")}
      >
        <button
          className="menu-trigger"
          type="button"
          aria-haspopup="menu"
          aria-expanded={openMenu === "view"}
          onClick={() => setOpenMenu(openMenu === "view" ? null : "view")}
        >
          View
        </button>
        {openMenu === "view" && (
          <div className="menu-dropdown" role="menu">
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onToggleLeftPanel)}
            >
              {leftPanelOpen ? "Hide" : "Show"} Browser Panel
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onToggleRightPanel)}
            >
              {rightPanelOpen ? "Hide" : "Show"} Inspector
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onToggleTimeline)}
            >
              {timelineOpen ? "Hide" : "Show"} Timeline
            </button>
          </div>
        )}
      </div>

      <div
        className={openMenu === "help" ? "menu-group open" : "menu-group"}
        onMouseEnter={() => setOpenMenu("help")}
      >
        <button
          className="menu-trigger"
          type="button"
          aria-haspopup="menu"
          aria-expanded={openMenu === "help"}
          onClick={() => setOpenMenu(openMenu === "help" ? null : "help")}
        >
          Help
        </button>
        {openMenu === "help" && (
          <div className="menu-dropdown" role="menu">
            <button
              type="button"
              role="menuitem"
              onClick={() => runMenuAction(onHelp)}
            >
              Sprite Studio Guide
            </button>
          </div>
        )}
      </div>
    </nav>
  );
}
