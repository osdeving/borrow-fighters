//! Centralizes prototype feature flags.
//!
//! Feature flags are the runtime contract for optional prototype behavior. New
//! experiments should add one flag here, then consume it through this API.

/// Identifies one runtime feature flag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeatureFlag {
    PlayerOneCpu,
    PlayerTwoCpu,
    CpuCanAttack,
    PlayerOneTakesDamage,
    PlayerTwoTakesDamage,
    ShowHud,
    ShowControlsHelp,
    ShowCombatDebug,
    GamepadInput,
}

/// Every feature flag exposed in the preferences screen.
pub const PREFERENCE_FLAGS: [FeatureFlag; 9] = [
    FeatureFlag::PlayerOneCpu,
    FeatureFlag::PlayerTwoCpu,
    FeatureFlag::CpuCanAttack,
    FeatureFlag::PlayerOneTakesDamage,
    FeatureFlag::PlayerTwoTakesDamage,
    FeatureFlag::ShowHud,
    FeatureFlag::ShowControlsHelp,
    FeatureFlag::ShowCombatDebug,
    FeatureFlag::GamepadInput,
];

/// Runtime feature flag values for the prototype.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeatureFlags {
    player_one_cpu: bool,
    player_two_cpu: bool,
    cpu_can_attack: bool,
    player_one_takes_damage: bool,
    player_two_takes_damage: bool,
    show_hud: bool,
    show_controls_help: bool,
    show_combat_debug: bool,
    gamepad_input: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            player_one_cpu: false,
            player_two_cpu: true,
            cpu_can_attack: true,
            player_one_takes_damage: true,
            player_two_takes_damage: true,
            show_hud: true,
            show_controls_help: false,
            show_combat_debug: false,
            gamepad_input: true,
        }
    }
}

impl FeatureFlags {
    /// Returns whether a feature flag is enabled.
    pub const fn enabled(self, flag: FeatureFlag) -> bool {
        match flag {
            FeatureFlag::PlayerOneCpu => self.player_one_cpu,
            FeatureFlag::PlayerTwoCpu => self.player_two_cpu,
            FeatureFlag::CpuCanAttack => self.cpu_can_attack,
            FeatureFlag::PlayerOneTakesDamage => self.player_one_takes_damage,
            FeatureFlag::PlayerTwoTakesDamage => self.player_two_takes_damage,
            FeatureFlag::ShowHud => self.show_hud,
            FeatureFlag::ShowControlsHelp => self.show_controls_help,
            FeatureFlag::ShowCombatDebug => self.show_combat_debug,
            FeatureFlag::GamepadInput => self.gamepad_input,
        }
    }

    /// Sets a feature flag value.
    pub fn set(&mut self, flag: FeatureFlag, enabled: bool) {
        match flag {
            FeatureFlag::PlayerOneCpu => self.player_one_cpu = enabled,
            FeatureFlag::PlayerTwoCpu => self.player_two_cpu = enabled,
            FeatureFlag::CpuCanAttack => self.cpu_can_attack = enabled,
            FeatureFlag::PlayerOneTakesDamage => self.player_one_takes_damage = enabled,
            FeatureFlag::PlayerTwoTakesDamage => self.player_two_takes_damage = enabled,
            FeatureFlag::ShowHud => self.show_hud = enabled,
            FeatureFlag::ShowControlsHelp => self.show_controls_help = enabled,
            FeatureFlag::ShowCombatDebug => self.show_combat_debug = enabled,
            FeatureFlag::GamepadInput => self.gamepad_input = enabled,
        }
    }

    /// Flips a feature flag value.
    pub fn toggle(&mut self, flag: FeatureFlag) {
        self.set(flag, !self.enabled(flag));
    }
}

impl FeatureFlag {
    /// Short label shown in the preferences screen.
    pub const fn label(self) -> &'static str {
        match self {
            FeatureFlag::PlayerOneCpu => "Player 1 usa IA",
            FeatureFlag::PlayerTwoCpu => "Player 2 usa IA",
            FeatureFlag::CpuCanAttack => "IA pode dar golpes",
            FeatureFlag::PlayerOneTakesDamage => "Player 1 recebe dano",
            FeatureFlag::PlayerTwoTakesDamage => "Player 2 recebe dano",
            FeatureFlag::ShowHud => "Mostrar HUD",
            FeatureFlag::ShowControlsHelp => "Mostrar ajuda de controles",
            FeatureFlag::ShowCombatDebug => "Mostrar debug de combate",
            FeatureFlag::GamepadInput => "Entrada por gamepad",
        }
    }

    /// Short description shown in the preferences screen.
    pub const fn description(self) -> &'static str {
        match self {
            FeatureFlag::PlayerOneCpu => "Quando ligado, o Player 1 tambem vira CPU.",
            FeatureFlag::PlayerTwoCpu => "Liga o sparring dummy do Player 2.",
            FeatureFlag::CpuCanAttack => "Quando desligado, a IA se move e defende, mas nao ataca.",
            FeatureFlag::PlayerOneTakesDamage => {
                "Quando desligado, o Player 1 fica invencivel para playtest."
            }
            FeatureFlag::PlayerTwoTakesDamage => {
                "Quando desligado, o Player 2 fica invencivel para playtest."
            }
            FeatureFlag::ShowHud => "Barras de vida, titulo e status no topo.",
            FeatureFlag::ShowControlsHelp => "Texto de comandos no rodape durante a luta.",
            FeatureFlag::ShowCombatDebug => "Hitboxes, hurtboxes, labels e colisao corpo-corpo.",
            FeatureFlag::GamepadInput => "Usa controles detectados pelo Raylib quando disponiveis.",
        }
    }
}
