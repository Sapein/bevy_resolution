//! `bevy_resolution` aims to provide a simple, easy, and convenient way to set and manage window
//! resolutions. It provides conveniences for commonly-supported Window Resolutions *and* also
//! provides a way to manage window resolution scaling for arbitrary resolutions.
//! 
//! This crate requires bevy version `0.15.0-rc.3`, as it relies on features introduced in the
//! release candidate.
//!
//! ## Features
//! - Provides an easy, but powerful, API for resolution management
//! - Provides convenience wrappers for common 16:9 and 4:3 resolutions
//! - Can support any resolution with any Aspect Ratio and uses Bevy's built-in `AspectRatio`
//! 
//! ## Feature Flags
//! This crate exposes the following feature flags:
//! - `bevy_window`: Enables support for usage with `bevy_window`, this is required for usage with `bevy_window::WindowResolution`
//! 
//! The `bevy_window` feature is enabled by default.
//! 
pub mod common;
pub mod resolutions;
