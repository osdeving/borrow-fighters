//! Simulates one player, stable actor IDs and staged encounters at sixty ticks.
//!
//! System: Adventure production combat. Input, AI, boxes, projectiles and events
//! are shared by the external lab and chapter; clocks advance only through tick.

use super::spec::{CharacterSpec, CombatCatalog, MoveSpec, TICKS_PER_SECOND};
use crate::math::{rect::Rect, vec2::Vec2};
use std::{collections::BTreeSet, sync::Arc};

const DT: f32 = 1.0 / TICKS_PER_SECOND as f32;
const BUFFER_TICKS: u32 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectileId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}
impl Facing {
    pub const fn sign(self) -> f32 {
        if matches!(self, Self::Right) {
            1.0
        } else {
            -1.0
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Idle,
    Start,
    Run,
    Stop,
    Turn,
    Jump,
    Fall,
    Land,
    Attack,
    Guard,
    Parry,
    Hurt,
    Knockout,
    Inactive,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    pub movement: f32,
    pub jump: bool,
    pub light: bool,
    pub kick: bool,
    pub spin: bool,
    pub linker: bool,
    pub guard: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Ongoing,
    Victory,
    Defeat,
}

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub floor_y: f32,
}
impl Bounds {
    fn validate(self) -> Result<(), String> {
        if ![self.left, self.right, self.floor_y]
            .iter()
            .all(|v| v.is_finite())
            || self.right - self.left < 64.0
        {
            return Err("invalid production bounds".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct EnemySpawn {
    pub character: String,
    pub x: f32,
    pub facing: Facing,
}

#[derive(Clone, Copy, Debug)]
enum Command {
    Jump,
    Light,
    Kick,
    Spin,
    Linker,
}

/// Readable actor snapshot, with transient contact bookkeeping kept private.
#[derive(Clone, Debug)]
pub struct Actor {
    pub id: ActorId,
    pub character: String,
    pub team: Team,
    pub position: Vec2,
    pub velocity: Vec2,
    pub facing: Facing,
    pub hp: u32,
    pub max_hp: u32,
    pub grounded: bool,
    pub stride_distance: f32,
    pub action: Action,
    pub action_ticks: u32,
    pub move_id: Option<String>,
    pub active: bool,
    move_clip: String,
    hit_targets: BTreeSet<ActorId>,
    buffer: Option<(Command, u32)>,
    stun: u32,
    parry_remaining: u32,
    parry_cooldown: u32,
    guard_previous: bool,
    guard_pressed: bool,
    bot_wait: u32,
}

impl Actor {
    fn new(id: ActorId, c: &CharacterSpec, team: Team, x: f32, floor: f32, facing: Facing) -> Self {
        Self {
            id,
            character: c.id.clone(),
            team,
            position: Vec2::new(x, floor),
            velocity: Vec2::ZERO,
            facing,
            hp: c.hp,
            max_hp: c.hp,
            grounded: true,
            stride_distance: 0.0,
            action: Action::Idle,
            action_ticks: 0,
            move_id: None,
            active: true,
            move_clip: String::new(),
            hit_targets: BTreeSet::new(),
            buffer: None,
            stun: 0,
            parry_remaining: 0,
            parry_cooldown: 0,
            guard_previous: false,
            guard_pressed: false,
            bot_wait: c
                .ai
                .as_ref()
                .map_or(0, |ai| ai.initial_delay + id.0 % 4 * 7),
        }
    }

    pub fn clip_id(&self) -> &str {
        match self.action {
            Action::Idle => "idle",
            Action::Start => "start",
            Action::Run => "run",
            Action::Stop => "stop",
            Action::Turn => "turn",
            Action::Jump => "jump",
            Action::Fall => "fall",
            Action::Land => "land",
            Action::Attack => &self.move_clip,
            Action::Guard => "guard",
            Action::Parry => "parry",
            Action::Hurt => "hurt",
            Action::Knockout => "knockout",
            Action::Inactive => "arrival",
        }
    }

    fn action(&mut self, action: Action) {
        if self.action != action {
            self.action = action;
            self.action_ticks = 0;
        }
    }

    fn neutral(&mut self) {
        self.action = Action::Idle;
        self.action_ticks = 0;
        self.move_id = None;
        self.move_clip.clear();
        self.buffer = None;
        self.stun = 0;
        self.parry_remaining = 0;
        self.guard_previous = false;
        self.guard_pressed = false;
        self.hit_targets.clear();
    }

    fn input(&mut self, input: Input) {
        self.guard_pressed |= input.guard && !self.guard_previous;
        self.guard_previous = input.guard;
        let command = if input.spin {
            Some(Command::Spin)
        } else if input.linker {
            Some(Command::Linker)
        } else if input.kick {
            Some(Command::Kick)
        } else if input.light {
            Some(Command::Light)
        } else if input.jump {
            Some(Command::Jump)
        } else {
            None
        };
        if let Some(command) = command {
            self.buffer = Some((command, BUFFER_TICKS));
        }
    }
}

#[derive(Clone, Debug)]
pub struct Projectile {
    pub id: ProjectileId,
    pub owner: ActorId,
    pub team: Team,
    pub position: Vec2,
    pub previous: Vec2,
    pub velocity: Vec2,
    pub facing: Facing,
    pub rect: [f32; 4],
    pub remaining: u32,
    pub visual_id: String,
    pub move_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    MoveStarted {
        actor: ActorId,
        move_id: String,
    },
    MoveFinished {
        actor: ActorId,
        move_id: String,
    },
    Hit {
        attacker: ActorId,
        target: ActorId,
        move_id: String,
        position: Vec2,
        damage: u32,
    },
    Blocked {
        attacker: ActorId,
        target: ActorId,
        position: Vec2,
    },
    Parried {
        attacker: ActorId,
        defender: ActorId,
        position: Vec2,
    },
    ProjectileSpawned {
        owner: ActorId,
        projectile: ProjectileId,
        move_id: String,
    },
    Knockout {
        actor: ActorId,
        by: ActorId,
    },
    Landed {
        actor: ActorId,
    },
}

/// An encounter owns only physical actors. Dialogue and wave progression stay in the host.
#[derive(Clone, Debug)]
pub struct Simulation {
    pub content: Arc<CombatCatalog>,
    pub bounds: Bounds,
    pub ticks: u64,
    pub hitstop_remaining: u32,
    actors: Vec<Actor>,
    projectiles: Vec<Projectile>,
    next_actor: u32,
    next_projectile: u32,
    encounter_started: bool,
}

impl Simulation {
    pub fn new(
        content: Arc<CombatCatalog>,
        bounds: Bounds,
        player_character: &str,
        player_x: f32,
    ) -> Result<Self, String> {
        bounds.validate()?;
        if !player_x.is_finite() {
            return Err("nonfinite player spawn".into());
        }
        let spec = content
            .characters
            .get(player_character)
            .ok_or_else(|| format!("missing character {player_character}"))?;
        let mut player = Actor::new(
            ActorId(1),
            spec,
            Team::Player,
            player_x,
            bounds.floor_y,
            Facing::Right,
        );
        clamp_actor(&mut player, spec, bounds);
        Ok(Self {
            content,
            bounds,
            ticks: 0,
            hitstop_remaining: 0,
            actors: vec![player],
            projectiles: vec![],
            next_actor: 2,
            next_projectile: 1,
            encounter_started: false,
        })
    }
    pub fn player_id(&self) -> ActorId {
        self.actors[0].id
    }
    pub fn player(&self) -> &Actor {
        &self.actors[0]
    }
    pub fn actors(&self) -> &[Actor] {
        &self.actors
    }
    pub fn actor(&self, id: ActorId) -> Option<&Actor> {
        self.actors.iter().find(|a| a.id == id)
    }
    pub fn projectiles(&self) -> &[Projectile] {
        &self.projectiles
    }

    pub fn outcome(&self) -> Outcome {
        if self.player().hp == 0 {
            Outcome::Defeat
        } else if self.encounter_started
            && self
                .actors
                .iter()
                .filter(|a| a.team == Team::Enemy)
                .all(|a| a.hp == 0)
        {
            Outcome::Victory
        } else {
            Outcome::Ongoing
        }
    }

    /// Removes the wave and its projectiles, preserving player health and location.
    pub fn clear_encounter(&mut self) {
        self.actors.truncate(1);
        self.projectiles.clear();
        self.encounter_started = false;
        self.hitstop_remaining = 0;
        self.actors[0].neutral();
        self.actors[0].velocity = Vec2::ZERO;
        if self.actors[0].hp == 0 {
            self.actors[0].action = Action::Knockout;
        }
    }

    /// Validates every spawn before replacing the previous wave.
    pub fn begin_encounter(&mut self, spawns: &[EnemySpawn]) -> Result<Vec<ActorId>, String> {
        if spawns.is_empty() {
            return Err("encounter requires enemies".into());
        }
        for spawn in spawns {
            self.validate_spawn(spawn)?;
        }
        self.clear_encounter();
        spawns
            .iter()
            .map(|spawn| self.spawn_enemy(spawn, true))
            .collect()
    }

    fn validate_spawn(&self, spawn: &EnemySpawn) -> Result<(), String> {
        if !spawn.x.is_finite() || !self.content.characters.contains_key(&spawn.character) {
            return Err(format!("invalid enemy spawn {}", spawn.character));
        }
        Ok(())
    }

    /// Inactive enemies can be staged during a descent and activated on ground contact.
    pub fn spawn_enemy(&mut self, spawn: &EnemySpawn, active: bool) -> Result<ActorId, String> {
        self.validate_spawn(spawn)?;
        let id = ActorId(self.next_actor);
        self.next_actor = self.next_actor.checked_add(1).ok_or("actor ID exhausted")?;
        let c = &self.content.characters[&spawn.character];
        let mut actor = Actor::new(
            id,
            c,
            Team::Enemy,
            spawn.x,
            self.bounds.floor_y,
            spawn.facing,
        );
        actor.active = active;
        if !active {
            actor.action = Action::Inactive;
        }
        clamp_actor(&mut actor, c, self.bounds);
        self.actors.push(actor);
        self.encounter_started = true;
        Ok(id)
    }

    pub fn set_actor_active(&mut self, id: ActorId, active: bool) -> Result<(), String> {
        let actor = self
            .actors
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or("unknown actor")?;
        actor.active = active;
        actor.neutral();
        actor.velocity = Vec2::ZERO;
        actor.action = if actor.hp == 0 {
            Action::Knockout
        } else if active {
            Action::Idle
        } else {
            Action::Inactive
        };
        Ok(())
    }

    pub fn set_actor_position(
        &mut self,
        id: ActorId,
        position: Vec2,
        facing: Facing,
    ) -> Result<(), String> {
        if !position.x.is_finite() || !position.y.is_finite() {
            return Err("nonfinite actor pose".into());
        }
        let actor = self
            .actors
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or("unknown actor")?;
        actor.position = position;
        actor.facing = facing;
        actor.velocity = Vec2::ZERO;
        actor.grounded = position.y >= self.bounds.floor_y;
        clamp_actor(
            actor,
            &self.content.characters[&actor.character],
            self.bounds,
        );
        Ok(())
    }

    pub fn stage_player(&mut self, x: f32, facing: Facing) -> Result<(), String> {
        self.set_actor_position(self.player_id(), Vec2::new(x, self.bounds.floor_y), facing)?;
        self.actors[0].neutral();
        self.actors[0].active = true;
        if self.actors[0].hp == 0 {
            self.actors[0].action = Action::Knockout;
        }
        Ok(())
    }

    /// Public checkpoint reset; callers decide which wave follows the restored player.
    pub fn reset_player(&mut self, x: f32) -> Result<(), String> {
        if !x.is_finite() {
            return Err("nonfinite player reset".into());
        }
        self.clear_encounter();
        self.actors[0].hp = self.actors[0].max_hp;
        self.actors[0].stride_distance = 0.0;
        self.actors[0].parry_cooldown = 0;
        self.stage_player(x, Facing::Right)
    }

    pub fn set_bounds(&mut self, bounds: Bounds) -> Result<(), String> {
        bounds.validate()?;
        self.bounds = bounds;
        for actor in &mut self.actors {
            if actor.grounded {
                actor.position.y = bounds.floor_y;
            }
            clamp_actor(actor, &self.content.characters[&actor.character], bounds);
        }
        Ok(())
    }

    pub fn hitboxes(&self, id: ActorId) -> Vec<Rect> {
        self.actor(id)
            .map_or_else(Vec::new, |a| hitboxes(a, &self.content))
    }
    pub fn hurtboxes(&self, id: ActorId) -> Vec<Rect> {
        self.actor(id)
            .map_or_else(Vec::new, |a| hurtboxes(a, &self.content))
    }

    /// Advances one fixed update and returns one-shot events in stable actor order.
    pub fn tick(&mut self, input: Input) -> Vec<Event> {
        self.tick_with_controls(&[(self.player_id(), input)])
    }

    /// Lab controls can override a bot without creating a second combat implementation.
    pub fn tick_with_controls(&mut self, controls: &[(ActorId, Input)]) -> Vec<Event> {
        self.ticks = self.ticks.saturating_add(1);
        let mut events = Vec::new();
        for (id, input) in controls {
            if let Some(actor) = self.actors.iter_mut().find(|a| a.id == *id) {
                actor.input(*input);
            }
        }
        if self.hitstop_remaining > 0 {
            self.hitstop_remaining -= 1;
            return events;
        }
        if self.player().hp == 0 {
            return events;
        }
        let old: Vec<_> = self
            .actors
            .iter()
            .map(|a| (a.position, a.grounded))
            .collect();
        let player_x = self.player().position.x;
        for actor in &mut self.actors {
            if !actor.active || actor.hp == 0 {
                actor.action_ticks = actor.action_ticks.saturating_add(1);
                continue;
            }
            let c = &self.content.characters[&actor.character];
            let input = controls
                .iter()
                .find(|(id, _)| *id == actor.id)
                .map(|(_, input)| *input)
                .unwrap_or_else(|| bot_input(actor, c, player_x));
            if !controls.iter().any(|(id, _)| *id == actor.id) {
                actor.input(input);
            }
            step_actor(actor, c, &self.content, input, self.bounds, &mut events);
        }
        self.separate_bodies();
        for (actor, (position, grounded)) in self.actors.iter_mut().zip(old) {
            if actor.active
                && actor.grounded
                && grounded
                && matches!(
                    actor.action,
                    Action::Idle
                        | Action::Start
                        | Action::Run
                        | Action::Stop
                        | Action::Turn
                        | Action::Land
                )
            {
                actor.stride_distance += (actor.position.x - position.x).abs();
            }
        }
        self.emit_projectiles(&mut events);
        let mut contacts = self.melee_contacts();
        self.projectile_contacts(&mut contacts);
        for contact in contacts {
            self.resolve(contact, &mut events);
        }
        events
    }

    fn separate_bodies(&mut self) {
        for i in 0..self.actors.len() {
            for j in i + 1..self.actors.len() {
                let (left, right) = self.actors.split_at_mut(j);
                let a = &mut left[i];
                let b = &mut right[0];
                if !a.active
                    || !b.active
                    || a.hp == 0
                    || b.hp == 0
                    || (a.position.y - b.position.y).abs() > 130.0
                {
                    continue;
                }
                let ca = &self.content.characters[&a.character];
                let cb = &self.content.characters[&b.character];
                let minimum = (ca.body_width + cb.body_width) * 0.5;
                let delta = b.position.x - a.position.x;
                if delta.abs() < minimum {
                    let sign = if delta >= 0.0 { 1.0 } else { -1.0 };
                    let push = (minimum - delta.abs()) * 0.5;
                    a.position.x -= sign * push;
                    b.position.x += sign * push;
                    clamp_actor(a, ca, self.bounds);
                    clamp_actor(b, cb, self.bounds);
                }
            }
        }
    }

    fn emit_projectiles(&mut self, events: &mut Vec<Event>) {
        for actor in &self.actors {
            if !actor.active || actor.hp == 0 || actor.action != Action::Attack {
                continue;
            }
            let Some(id) = &actor.move_id else {
                continue;
            };
            let Some(spec) = &self.content.moves[id].projectile else {
                continue;
            };
            if actor.action_ticks != spec.spawn_tick {
                continue;
            }
            let id = ProjectileId(self.next_projectile);
            self.next_projectile += 1;
            let position = Vec2::new(
                actor.position.x + spec.offset[0] * actor.facing.sign(),
                actor.position.y + spec.offset[1],
            );
            self.projectiles.push(Projectile {
                id,
                owner: actor.id,
                team: actor.team,
                position,
                previous: position,
                velocity: Vec2::new(spec.velocity[0] * actor.facing.sign(), spec.velocity[1]),
                facing: actor.facing,
                rect: spec.rect,
                remaining: spec.life_ticks,
                visual_id: spec.visual_id.clone(),
                move_id: actor.move_id.clone().expect("cast"),
            });
            events.push(Event::ProjectileSpawned {
                owner: actor.id,
                projectile: id,
                move_id: actor.move_id.clone().expect("cast"),
            });
        }
    }

    fn melee_contacts(&mut self) -> Vec<Contact> {
        let mut contacts = vec![];
        for attacker in &self.actors {
            let boxes = hitboxes(attacker, &self.content);
            if boxes.is_empty() {
                continue;
            }
            for target in &self.actors {
                if target.team == attacker.team || attacker.hit_targets.contains(&target.id) {
                    continue;
                }
                if boxes.iter().any(|hit| {
                    hurtboxes(target, &self.content)
                        .iter()
                        .any(|hurt| hit.intersects(*hurt))
                }) {
                    contacts.push(Contact {
                        attacker: attacker.id,
                        target: target.id,
                        move_id: attacker.move_id.clone().expect("active move"),
                        direction: attacker.facing.sign(),
                        projectile: false,
                        defense: defense(target, attacker.position.x),
                    });
                }
            }
        }
        for contact in &contacts {
            if let Some(a) = self.actors.iter_mut().find(|a| a.id == contact.attacker) {
                a.hit_targets.insert(contact.target);
            }
        }
        contacts
    }

    fn projectile_contacts(&mut self, contacts: &mut Vec<Contact>) {
        let mut removed = BTreeSet::new();
        for p in &mut self.projectiles {
            p.previous = p.position;
            p.position.x += p.velocity.x * DT;
            p.position.y += p.velocity.y * DT;
            p.remaining = p.remaining.saturating_sub(1);
            let start = project(p.rect, p.previous, p.facing);
            let delta = Vec2::new(p.position.x - p.previous.x, p.position.y - p.previous.y);
            let nearest = self
                .actors
                .iter()
                .filter(|a| a.team != p.team)
                .flat_map(|a| {
                    hurtboxes(a, &self.content)
                        .into_iter()
                        .filter_map(move |r| sweep(start, delta, r).map(|time| (time, a)))
                })
                .min_by(|(t, a), (u, b)| t.total_cmp(u).then(a.id.cmp(&b.id)));
            if let Some((_, target)) = nearest {
                contacts.push(Contact {
                    attacker: p.owner,
                    target: target.id,
                    move_id: p.move_id.clone(),
                    direction: p.facing.sign(),
                    projectile: true,
                    defense: defense(target, p.previous.x),
                });
                removed.insert(p.id);
            }
            if p.remaining == 0
                || p.position.x < self.bounds.left - 100.0
                || p.position.x > self.bounds.right + 100.0
            {
                removed.insert(p.id);
            }
        }
        self.projectiles.retain(|p| !removed.contains(&p.id));
    }

    fn resolve(&mut self, contact: Contact, events: &mut Vec<Event>) {
        let Some(target_index) = self.actors.iter().position(|a| a.id == contact.target) else {
            return;
        };
        if self.actors[target_index].hp == 0 {
            return;
        }
        let m = &self.content.moves[&contact.move_id];
        let target = &mut self.actors[target_index];
        let position = Vec2::new(target.position.x, target.position.y - 95.0);
        self.hitstop_remaining = self.hitstop_remaining.max(m.hitstop);
        match contact.defense {
            Defense::Parry => {
                target.action(Action::Parry);
                target.stun = 8;
                let duration = self.content.characters[&target.character].guard.parry_stun;
                if !contact.projectile
                    && let Some(attacker) =
                        self.actors.iter_mut().find(|a| a.id == contact.attacker)
                    && attacker.hp > 0
                {
                    attacker.action(Action::Hurt);
                    attacker.stun = duration;
                    attacker.move_id = None;
                    attacker.buffer = None;
                    attacker.velocity = Vec2::ZERO;
                }
                events.push(Event::Parried {
                    attacker: contact.attacker,
                    defender: contact.target,
                    position,
                });
            }
            Defense::Guard => {
                target.stun = self.content.characters[&target.character].guard.blockstun;
                target.velocity.x = contact.direction * m.knockback * 0.2;
                events.push(Event::Blocked {
                    attacker: contact.attacker,
                    target: contact.target,
                    position,
                });
            }
            Defense::Open => {
                let damage = m.damage.min(target.hp);
                target.hp -= damage;
                target.move_id = None;
                target.buffer = None;
                target.parry_remaining = 0;
                target.velocity.x = contact.direction * m.knockback;
                target.action = if target.hp == 0 {
                    Action::Knockout
                } else {
                    Action::Hurt
                };
                target.action_ticks = 0;
                target.stun = m.hitstun;
                events.push(Event::Hit {
                    attacker: contact.attacker,
                    target: contact.target,
                    move_id: contact.move_id,
                    position,
                    damage,
                });
                if target.hp == 0 {
                    events.push(Event::Knockout {
                        actor: target.id,
                        by: contact.attacker,
                    });
                }
            }
        }
    }
}

fn approach(value: f32, target: f32, amount: f32) -> f32 {
    if value < target {
        (value + amount).min(target)
    } else {
        (value - amount).max(target)
    }
}

fn clamp_actor(actor: &mut Actor, c: &CharacterSpec, bounds: Bounds) {
    let half = (c.body_width * 0.5).min((bounds.right - bounds.left) * 0.5);
    let before = actor.position.x;
    actor.position.x = actor
        .position
        .x
        .clamp(bounds.left + half, bounds.right - half);
    if actor.position.x != before {
        actor.velocity.x = 0.0;
    }
    if actor.position.y >= bounds.floor_y {
        actor.position.y = bounds.floor_y;
        actor.velocity.y = 0.0;
        actor.grounded = true;
    }
}

fn bot_input(actor: &Actor, c: &CharacterSpec, player_x: f32) -> Input {
    if actor.team != Team::Enemy {
        return Input::default();
    }
    let Some(ai) = &c.ai else {
        return Input::default();
    };
    let delta = player_x - actor.position.x;
    if delta.abs() > ai.attack_range {
        Input {
            movement: delta.signum(),
            ..Input::default()
        }
    } else if actor.bot_wait <= 1
        && !matches!(
            actor.action,
            Action::Attack | Action::Hurt | Action::Knockout
        )
    {
        Input {
            light: true,
            movement: delta.signum() * 0.01,
            ..Input::default()
        }
    } else {
        Input::default()
    }
}

fn start_move(actor: &mut Actor, m: &MoveSpec, c: &CharacterSpec, events: &mut Vec<Event>) {
    actor.action = Action::Attack;
    actor.action_ticks = 0;
    actor.move_id = Some(m.id.clone());
    actor.move_clip = m.animation_id.clone();
    actor.hit_targets.clear();
    actor.buffer = None;
    actor.parry_remaining = 0;
    actor.bot_wait = c.ai.as_ref().map_or(0, |ai| ai.cooldown_ticks);
    events.push(Event::MoveStarted {
        actor: actor.id,
        move_id: m.id.clone(),
    });
}

fn step_actor(
    actor: &mut Actor,
    c: &CharacterSpec,
    catalog: &CombatCatalog,
    input: Input,
    bounds: Bounds,
    events: &mut Vec<Event>,
) {
    let previous_facing = actor.facing;
    let was_grounded = actor.grounded;
    actor.action_ticks = actor.action_ticks.saturating_add(1);
    actor.stun = actor.stun.saturating_sub(1);
    actor.parry_remaining = actor.parry_remaining.saturating_sub(1);
    actor.parry_cooldown = actor.parry_cooldown.saturating_sub(1);
    actor.bot_wait = actor.bot_wait.saturating_sub(1);
    let movement = if input.movement.is_finite() {
        input.movement.clamp(-1.0, 1.0)
    } else {
        0.0
    };
    if actor.action == Action::Attack {
        let m = &catalog.moves[actor.move_id.as_ref().expect("attack has move")];
        if actor.action_ticks >= m.duration() {
            events.push(Event::MoveFinished {
                actor: actor.id,
                move_id: m.id.clone(),
            });
            actor.action(Action::Idle);
            actor.move_id = None;
        } else if matches!(actor.buffer, Some((Command::Light, _)))
            && let (Some(next), Some([start, end])) = (&m.combo_next, m.combo_window)
            && (start..end).contains(&actor.action_ticks)
        {
            start_move(actor, &catalog.moves[next], c, events);
        }
    }
    if matches!(actor.action, Action::Hurt | Action::Parry) && actor.stun == 0 {
        actor.action(Action::Idle);
    }
    let locked = actor.action == Action::Attack || actor.stun > 0;
    if !locked && actor.grounded {
        let requested = actor.buffer.and_then(|(command, _)| match command {
            Command::Light => c.loadout.light.first().map(String::as_str),
            Command::Kick => c.loadout.kick.as_deref(),
            Command::Spin => c.loadout.spin.as_deref(),
            Command::Linker => c.loadout.linker.as_deref(),
            Command::Jump => None,
        });
        if let Some(id) = requested {
            if movement.abs() > 0.0 && actor.velocity.x.abs() < 8.0 {
                actor.facing = if movement > 0.0 {
                    Facing::Right
                } else {
                    Facing::Left
                };
            }
            start_move(actor, &catalog.moves[id], c, events);
        } else if matches!(actor.buffer, Some((Command::Jump, _))) {
            actor.velocity.y = -c.movement.jump_speed;
            actor.grounded = false;
            actor.action(Action::Jump);
            actor.buffer = None;
        } else if input.guard {
            actor.action(Action::Guard);
            if actor.guard_pressed && actor.parry_cooldown == 0 {
                actor.parry_remaining = c.guard.parry_ticks;
                actor.parry_cooldown = c.guard.parry_cooldown;
            }
        } else if actor.action == Action::Guard {
            actor.action(Action::Idle);
            actor.parry_remaining = 0;
        }
    }
    actor.guard_pressed = false;
    let locked = actor.action == Action::Attack || actor.stun > 0;
    if matches!(actor.action, Action::Guard | Action::Parry) && actor.stun == 0 {
        actor.velocity.x = 0.0;
    } else if locked {
        actor.velocity.x = approach(
            actor.velocity.x,
            0.0,
            c.movement.braking
                * DT
                * if actor.action == Action::Hurt {
                    0.25
                } else {
                    1.0
                },
        );
    } else {
        let target = movement * c.movement.run_speed;
        let rate = if movement == 0.0 {
            c.movement.braking
        } else if actor.velocity.x * target < 0.0 {
            c.movement.turn_acceleration
        } else {
            c.movement.acceleration
        };
        actor.velocity.x = approach(actor.velocity.x, target, rate * DT);
        if actor.velocity.x.abs() > 8.0 {
            actor.facing = if actor.velocity.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            };
        }
    }
    actor.position.x += actor.velocity.x * DT;
    if !actor.grounded {
        actor.velocity.y += c.movement.gravity * DT;
        actor.position.y += actor.velocity.y * DT;
    }
    clamp_actor(actor, c, bounds);
    if !was_grounded && actor.grounded {
        events.push(Event::Landed { actor: actor.id });
        if !locked {
            actor.action(Action::Land);
        }
    }
    if !locked && !matches!(actor.action, Action::Guard | Action::Parry) {
        if !actor.grounded {
            actor.action(if actor.velocity.y < 0.0 {
                Action::Jump
            } else {
                Action::Fall
            });
        } else if actor.action == Action::Land && actor.action_ticks < 6 {
        } else if previous_facing != actor.facing {
            actor.action(Action::Turn);
        } else if actor.action == Action::Turn && actor.action_ticks < 8 {
        } else if movement != 0.0 && actor.velocity.x.abs() > 0.5 {
            actor.action(if actor.velocity.x.abs() < c.movement.run_speed * 0.9 {
                Action::Start
            } else {
                Action::Run
            });
        } else if actor.velocity.x.abs() > 0.5
            || actor.action == Action::Stop && actor.action_ticks < 8
        {
            actor.action(Action::Stop);
        } else {
            actor.action(Action::Idle);
        }
    }
    if let Some((_, ttl)) = &mut actor.buffer {
        *ttl = ttl.saturating_sub(1);
        if *ttl == 0 {
            actor.buffer = None;
        }
    }
}

fn project([x, y, w, h]: [f32; 4], at: Vec2, facing: Facing) -> Rect {
    Rect::new(
        at.x + if facing == Facing::Right { x } else { -x - w },
        at.y + y,
        w,
        h,
    )
}
fn hitboxes(actor: &Actor, catalog: &CombatCatalog) -> Vec<Rect> {
    if !actor.active || actor.hp == 0 || actor.action != Action::Attack {
        return vec![];
    }
    actor.move_id.as_ref().map_or_else(Vec::new, |id| {
        catalog.moves[id]
            .hitboxes
            .iter()
            .filter(|b| (b.start..b.end).contains(&actor.action_ticks))
            .map(|b| project(b.rect, actor.position, actor.facing))
            .collect()
    })
}
fn hurtboxes(actor: &Actor, catalog: &CombatCatalog) -> Vec<Rect> {
    if !actor.active || actor.hp == 0 {
        return vec![];
    }
    if let Some(id) = &actor.move_id {
        let boxes: Vec<_> = catalog.moves[id]
            .hurtboxes
            .iter()
            .filter(|b| (b.start..b.end).contains(&actor.action_ticks))
            .map(|b| project(b.rect, actor.position, actor.facing))
            .collect();
        if !boxes.is_empty() {
            return boxes;
        }
    }
    catalog.characters[&actor.character]
        .hurtboxes
        .iter()
        .map(|b| project(*b, actor.position, actor.facing))
        .collect()
}

#[derive(Clone, Copy)]
enum Defense {
    Open,
    Guard,
    Parry,
}
fn defense(actor: &Actor, source_x: f32) -> Defense {
    if actor.grounded
        && matches!(actor.action, Action::Guard | Action::Parry)
        && (source_x - actor.position.x) * actor.facing.sign() >= 0.0
    {
        if actor.parry_remaining > 0 {
            Defense::Parry
        } else {
            Defense::Guard
        }
    } else {
        Defense::Open
    }
}
struct Contact {
    attacker: ActorId,
    target: ActorId,
    move_id: String,
    direction: f32,
    projectile: bool,
    defense: Defense,
}

/// Swept AABB time of impact prevents a fast Linker projectile tunnelling between ticks.
fn sweep(start: Rect, delta: Vec2, target: Rect) -> Option<f32> {
    if start.intersects(target) {
        return Some(0.0);
    }
    let axis = |min: f32, max: f32, tmin: f32, tmax: f32, d: f32| {
        if d.abs() < 0.00001 {
            if max <= tmin || min >= tmax {
                None
            } else {
                Some((f32::NEG_INFINITY, f32::INFINITY))
            }
        } else {
            let a = (tmin - max) / d;
            let b = (tmax - min) / d;
            Some((a.min(b), a.max(b)))
        }
    };
    let (ex, lx) = axis(start.x, start.right(), target.x, target.right(), delta.x)?;
    let (ey, ly) = axis(start.y, start.bottom(), target.y, target.bottom(), delta.y)?;
    let enter = ex.max(ey);
    let leave = lx.min(ly);
    (enter <= leave && (0.0..=1.0).contains(&enter)).then_some(enter)
}
