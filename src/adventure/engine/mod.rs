//! Adapts the adventure's independent presentation and controls to Raylib.
//!
//! System: Adventure platform boundary. Domain state stays in the sibling story
//! and combat modules; no fighting-game adapter is loaded here.

pub mod actors;
pub mod assets;
pub mod audio;
pub mod biography;
pub mod capture;
pub mod chapter;
pub mod chapter_audio;
pub mod ep_arrival;
pub mod landscape;
pub mod locomotion;
pub mod morning;
pub mod neighborhood;
pub mod opening;
pub mod pieces;
pub mod render;
pub mod street;
pub mod traffic;
pub mod typography;

pub mod production;
