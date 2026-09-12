//! Validates versioned character packs and move timing without loading art or files.
//!
//! System: Adventure production data. Local rectangles use a feet origin, forward
//! positive X and negative Y above the floor; every tick interval is half-open.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Shared fixed simulation frequency for lab and chapters.
pub const TICKS_PER_SECOND: u32 = 60;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MovementSpec {
    pub run_speed: f32,
    pub acceleration: f32,
    pub braking: f32,
    pub turn_acceleration: f32,
    pub jump_speed: f32,
    pub gravity: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardSpec {
    pub parry_ticks: u32,
    pub parry_cooldown: u32,
    pub blockstun: u32,
    pub parry_stun: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Loadout {
    pub light: Vec<String>,
    pub kick: Option<String>,
    pub spin: Option<String>,
    pub linker: Option<String>,
}

impl Loadout {
    pub fn moves(&self) -> impl Iterator<Item = &str> {
        self.light
            .iter()
            .map(String::as_str)
            .chain(self.kick.as_deref())
            .chain(self.spin.as_deref())
            .chain(self.linker.as_deref())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AiSpec {
    pub attack_range: f32,
    pub cooldown_ticks: u32,
    pub initial_delay: u32,
}

/// Physical content and links to the separately authored visual source files.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterSpec {
    pub schema_version: u32,
    pub id: String,
    pub name_key: String,
    pub combat: String,
    pub rig: String,
    pub clips: String,
    pub hp: u32,
    pub body_width: f32,
    pub hurtboxes: Vec<[f32; 4]>,
    pub movement: MovementSpec,
    pub guard: GuardSpec,
    pub loadout: Loadout,
    pub ai: Option<AiSpec>,
}

/// One rectangle active during [start,end), measured from the move's first tick.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HitboxWindow {
    pub start: u32,
    pub end: u32,
    pub rect: [f32; 4],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectileSpec {
    pub spawn_tick: u32,
    pub offset: [f32; 2],
    pub velocity: [f32; 2],
    pub rect: [f32; 4],
    pub life_ticks: u32,
    pub visual_id: String,
}

/// An authored strike, with one contact per target per execution.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MoveSpec {
    pub id: String,
    pub animation_id: String,
    pub startup: u32,
    pub active: u32,
    pub recovery: u32,
    pub damage: u32,
    pub hitstun: u32,
    pub knockback: f32,
    pub hitstop: u32,
    pub hitboxes: Vec<HitboxWindow>,
    #[serde(default)]
    pub hurtboxes: Vec<HitboxWindow>,
    pub combo_next: Option<String>,
    pub combo_window: Option<[u32; 2]>,
    pub projectile: Option<ProjectileSpec>,
}

impl MoveSpec {
    pub fn duration(&self) -> u32 {
        self.startup + self.active + self.recovery
    }
    pub fn active_at(&self, tick: u32) -> bool {
        (self.startup..self.startup + self.active).contains(&tick)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MoveLibrary {
    pub schema_version: u32,
    pub tick_hz: u32,
    pub moves: Vec<MoveSpec>,
}

#[derive(Clone, Debug)]
pub struct CharacterPack {
    pub character: CharacterSpec,
    pub combat: MoveLibrary,
}

impl CharacterPack {
    /// Parses source strings; the application chooses where those strings come from.
    pub fn from_json(character: &str, combat: &str) -> Result<Self, String> {
        let pack = Self {
            character: serde_json::from_str(character).map_err(|e| e.to_string())?,
            combat: serde_json::from_str(combat).map_err(|e| e.to_string())?,
        };
        CombatCatalog::new(vec![pack.clone()])?;
        Ok(pack)
    }
}

/// Fully validated immutable input to each simulation instance.
#[derive(Clone, Debug)]
pub struct CombatCatalog {
    pub characters: BTreeMap<String, CharacterSpec>,
    pub moves: BTreeMap<String, MoveSpec>,
}

fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"._-".contains(&c))
}

fn rectangle(rect: &[f32; 4]) -> bool {
    rect.iter().all(|v| v.is_finite() && v.abs() <= 4000.0) && rect[2] > 0.0 && rect[3] > 0.0
}

fn content_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains('\\')
        && !value.contains(':')
        && value
            .split('/')
            .all(|part| !part.is_empty() && part != ".." && part != ".")
}

impl CombatCatalog {
    /// Validates the whole candidate before a host swaps content or creates actors.
    pub fn new(packs: Vec<CharacterPack>) -> Result<Self, String> {
        if packs.is_empty() {
            return Err("production catalog has no characters".into());
        }
        let mut result = Self {
            characters: BTreeMap::new(),
            moves: BTreeMap::new(),
        };
        for pack in packs {
            let c = pack.character;
            if c.schema_version != 1
                || !identifier(&c.id)
                || c.name_key.is_empty()
                || ![&c.combat, &c.rig, &c.clips]
                    .into_iter()
                    .all(|p| content_path(p))
                || !(1..=10000).contains(&c.hp)
                || !c.body_width.is_finite()
                || !(1.0..=500.0).contains(&c.body_width)
                || c.hurtboxes.is_empty()
                || !c.hurtboxes.iter().all(rectangle)
                || c.loadout.light.is_empty()
                || c.loadout.light.len() > 3
            {
                return Err(format!("invalid character {}", c.id));
            }
            let m = &c.movement;
            if ![
                m.run_speed,
                m.acceleration,
                m.braking,
                m.turn_acceleration,
                m.jump_speed,
                m.gravity,
            ]
            .iter()
            .all(|v| v.is_finite() && *v > 0.0 && *v <= 10000.0)
                || c.guard.parry_ticks > 30
                || c.guard.parry_cooldown < c.guard.parry_ticks
                || c.guard.blockstun > 180
                || c.guard.parry_stun > 180
            {
                return Err(format!("invalid movement or guard for {}", c.id));
            }
            if let Some(ai) = &c.ai
                && (!ai.attack_range.is_finite()
                    || !(1.0..=1000.0).contains(&ai.attack_range)
                    || !(1..=600).contains(&ai.cooldown_ticks)
                    || ai.initial_delay > 600)
            {
                return Err(format!("invalid AI for {}", c.id));
            }
            if pack.combat.schema_version != 1
                || pack.combat.tick_hz != TICKS_PER_SECOND
                || pack.combat.moves.is_empty()
            {
                return Err(format!("invalid move library for {}", c.id));
            }
            for m in pack.combat.moves {
                if !identifier(&m.id)
                    || !identifier(&m.animation_id)
                    || m.startup > 180
                    || !(1..=120).contains(&m.active)
                    || m.recovery > 180
                    || !(1..=10000).contains(&m.damage)
                    || m.hitstun > 240
                    || m.hitstop > 30
                    || !m.knockback.is_finite()
                    || !(0.0..=2000.0).contains(&m.knockback)
                    || m.hitboxes.is_empty() && m.projectile.is_none()
                {
                    return Err(format!("invalid move {}", m.id));
                }
                if m.hitboxes.iter().any(|b| {
                    b.start < m.startup
                        || b.end > m.startup + m.active
                        || b.start >= b.end
                        || !rectangle(&b.rect)
                }) || m
                    .hurtboxes
                    .iter()
                    .any(|b| b.start >= b.end || b.end > m.duration() || !rectangle(&b.rect))
                {
                    return Err(format!("invalid box window in {}", m.id));
                }
                match (&m.combo_next, m.combo_window) {
                    (None, None) => {}
                    (Some(next), Some([start, end]))
                        if identifier(next)
                            && start >= m.startup + m.active
                            && start < end
                            && end <= m.duration() => {}
                    _ => return Err(format!("invalid combo window in {}", m.id)),
                }
                if let Some(p) = &m.projectile
                    && (p.spawn_tick < m.startup
                        || p.spawn_tick >= m.startup + m.active
                        || !p
                            .offset
                            .iter()
                            .chain(p.velocity.iter())
                            .all(|v| v.is_finite() && v.abs() <= 4000.0)
                        || p.velocity[0] <= 0.0
                        || !rectangle(&p.rect)
                        || !(1..=1200).contains(&p.life_ticks)
                        || !identifier(&p.visual_id))
                {
                    return Err(format!("invalid projectile in {}", m.id));
                }
                let id = m.id.clone();
                if result.moves.insert(id.clone(), m).is_some() {
                    return Err(format!("duplicate move {id}"));
                }
            }
            let id = c.id.clone();
            if result.characters.insert(id.clone(), c).is_some() {
                return Err(format!("duplicate character {id}"));
            }
        }
        for c in result.characters.values() {
            let allowed: BTreeSet<_> = c.loadout.moves().collect();
            for id in &allowed {
                if !result.moves.contains_key(*id) {
                    return Err(format!("{} references missing move {id}", c.id));
                }
            }
            for (index, id) in c.loadout.light.iter().enumerate() {
                let m = &result.moves[id];
                if m.combo_next.as_ref() != c.loadout.light.get(index + 1) {
                    return Err(format!("{} has inconsistent light combo", c.id));
                }
            }
            for id in allowed {
                let mut seen = BTreeSet::new();
                let mut next = Some(id);
                while let Some(move_id) = next {
                    if !seen.insert(move_id) {
                        return Err(format!("combo cycle at {move_id}"));
                    }
                    let m = result
                        .moves
                        .get(move_id)
                        .ok_or_else(|| format!("missing combo move {move_id}"))?;
                    if !c.loadout.moves().any(|id| id == move_id) {
                        return Err(format!("foreign combo move {move_id}"));
                    }
                    next = m.combo_next.as_deref();
                }
            }
        }
        Ok(result)
    }
}
