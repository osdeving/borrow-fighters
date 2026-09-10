//! Samples Rust's short phone conversation and its visible delivery milestones.
//!
//! System: Adventure chapter direction. Rendering and audio read one fixed clock;
//! this module neither sends messages nor advances a second timer.

/// Rust sends the first message after taking out and using the phone.
pub const PHONE_FIRST_SENT_TICK: u32 = 210;
/// Python's reply appears after the wait and typing indicator.
pub const PHONE_REPLY_TICK: u32 = 510;
/// Rust sends his final reply.
pub const PHONE_LAST_SENT_TICK: u32 = 720;
/// Rust starts putting the device away.
pub const PHONE_STOW_TICK: u32 = 810;
/// The device is fully pocketed and exploration may resume.
pub const PHONE_DONE_TICK: u32 = 870;
/// Text keys in the exact authored message order.
pub const PHONE_MESSAGE_KEYS: [&str; 3] = ["phone.first", "phone.python", "phone.last"];

/// An actor or on-screen messenger action within the conversation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhonePhase {
    /// Taking the device out of Rust's pocket.
    Drawing,
    /// Typing the first outgoing message.
    TypingFirst,
    /// First message sent, with its delivery indicator.
    FirstSent,
    /// Rust waits for an answer without typing.
    Waiting,
    /// Python's typing indicator is active.
    PythonTyping,
    /// Python's received message is being read.
    ReadingReply,
    /// Rust writes the final outgoing message.
    TypingLast,
    /// Final message sent; Rust reads it for a moment.
    LastSent,
    /// Putting the device back in his pocket.
    Stowing,
}

/// Complete presentation snapshot of the current phone conversation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhoneView {
    /// Elapsed ticks of the complete conversation.
    pub ticks: u32,
    /// Current authored action.
    pub phase: PhonePhase,
    /// Elapsed ticks of the current action.
    pub phase_ticks: u32,
    /// Number of messages already delivered to the panel, from zero to three.
    pub message_count: usize,
    /// Fraction of the current outgoing message typed, from zero through one.
    pub typed_fraction: f32,
    /// Which outgoing message is currently being typed, if any.
    pub active_message: Option<usize>,
    /// Whether Python's remote typing indicator is active.
    pub python_typing: bool,
    /// Whether the first message has been read by Python.
    pub first_read: bool,
    /// The magnified UI appears after the device is in hand.
    pub panel_visible: bool,
    /// A socketed physical phone remains in hand until pocketing finishes.
    pub device_visible: bool,
}

impl PhoneView {
    /// Samples the requested age, or none once the phone has been put away.
    pub fn at(ticks: u32) -> Option<Self> {
        let (phase, start) = match ticks {
            0..90 => (PhonePhase::Drawing, 0),
            90..PHONE_FIRST_SENT_TICK => (PhonePhase::TypingFirst, 90),
            PHONE_FIRST_SENT_TICK..270 => (PhonePhase::FirstSent, PHONE_FIRST_SENT_TICK),
            270..390 => (PhonePhase::Waiting, 270),
            390..PHONE_REPLY_TICK => (PhonePhase::PythonTyping, 390),
            PHONE_REPLY_TICK..600 => (PhonePhase::ReadingReply, PHONE_REPLY_TICK),
            600..PHONE_LAST_SENT_TICK => (PhonePhase::TypingLast, 600),
            PHONE_LAST_SENT_TICK..PHONE_STOW_TICK => (PhonePhase::LastSent, PHONE_LAST_SENT_TICK),
            PHONE_STOW_TICK..PHONE_DONE_TICK => (PhonePhase::Stowing, PHONE_STOW_TICK),
            _ => return None,
        };
        let phase_ticks = ticks - start;
        let active_message = match phase {
            PhonePhase::TypingFirst => Some(0),
            PhonePhase::TypingLast => Some(2),
            _ => None,
        };
        Some(Self {
            ticks,
            phase,
            phase_ticks,
            message_count: usize::from(ticks >= PHONE_FIRST_SENT_TICK)
                + usize::from(ticks >= PHONE_REPLY_TICK)
                + usize::from(ticks >= PHONE_LAST_SENT_TICK),
            typed_fraction: if active_message.is_some() {
                phase_ticks as f32 / 120.0
            } else {
                0.0
            },
            active_message,
            python_typing: phase == PhonePhase::PythonTyping,
            first_read: ticks >= 390,
            panel_visible: (90..PHONE_STOW_TICK).contains(&ticks),
            device_visible: (30..PHONE_DONE_TICK - 24).contains(&ticks),
        })
    }
}
