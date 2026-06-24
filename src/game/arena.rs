//! Defines the playable arena rotation.
//!
//! System: Match runtime. This module keeps arena identity and rotation order
//! testable without Raylib textures or render code.

/// Prototype arena identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArenaId {
    Sirius,
    Fortaleza,
    JavaStreet,
    BioTic,
    PortoDigital,
    ValeDoPinhao,
}

impl ArenaId {
    /// First arena used when the application starts.
    pub const STARTING_ARENA: Self = Self::Sirius;

    /// Ordered arena cycle used after each finished match.
    pub const ROTATION: [Self; 6] = [
        Self::Sirius,
        Self::Fortaleza,
        Self::JavaStreet,
        Self::BioTic,
        Self::PortoDigital,
        Self::ValeDoPinhao,
    ];

    /// Returns the next arena in the prototype rotation.
    pub const fn next(self) -> Self {
        match self {
            Self::Sirius => Self::Fortaleza,
            Self::Fortaleza => Self::JavaStreet,
            Self::JavaStreet => Self::BioTic,
            Self::BioTic => Self::PortoDigital,
            Self::PortoDigital => Self::ValeDoPinhao,
            Self::ValeDoPinhao => Self::Sirius,
        }
    }

    /// Returns the previous arena in the prototype rotation.
    pub const fn previous(self) -> Self {
        match self {
            Self::Sirius => Self::ValeDoPinhao,
            Self::Fortaleza => Self::Sirius,
            Self::JavaStreet => Self::Fortaleza,
            Self::BioTic => Self::JavaStreet,
            Self::PortoDigital => Self::BioTic,
            Self::ValeDoPinhao => Self::PortoDigital,
        }
    }

    /// Returns a short label for UI and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sirius => "Sirius Light Ring",
            Self::Fortaleza => "Tech Coast Beacon",
            Self::JavaStreet => "Java Street Terminal",
            Self::BioTic => "BioTIC Garden",
            Self::PortoDigital => "Porto Digital Cache",
            Self::ValeDoPinhao => "Pinhao Smart Grid",
        }
    }

    /// Returns the Brazilian location used as arena context.
    pub const fn location(self) -> &'static str {
        match self {
            Self::Sirius => "Campinas, SP",
            Self::Fortaleza => "Fortaleza, CE",
            Self::JavaStreet => "Sao Paulo, SP",
            Self::BioTic => "Brasilia, DF",
            Self::PortoDigital => "Recife, PE",
            Self::ValeDoPinhao => "Curitiba, PR",
        }
    }

    /// Returns the quick concept hook shown in the menu.
    pub const fn concept(self) -> &'static str {
        match self {
            Self::Sirius => "anel de luz sincrotron",
            Self::Fortaleza => "sensores oceanicos e energia limpa",
            Self::JavaStreet => "rua terminal de legado Java",
            Self::BioTic => "jardins biotech e cidade planejada",
            Self::PortoDigital => "pontes, cache antigo e neon",
            Self::ValeDoPinhao => "smart city, chuva e sensores",
        }
    }
}
