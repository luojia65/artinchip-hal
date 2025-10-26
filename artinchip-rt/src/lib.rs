//! Bare-metal ROM runtime for ArtInChip chips.
#![no_std]

pub use artinchip_rt_macros::pbp_entry;

pub mod pbp;
