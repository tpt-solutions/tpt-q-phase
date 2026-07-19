// SPDX-License-Identifier: MIT OR Apache-2.0

//! Pulse parameter types: amplitude, frequency, phase and timing.
//!
//! These describe the microwave envelope parameters emitted by the pulse
//! sequencer. The [`schedule`] module turns gate sequences into streams of
//! these pulses with nanosecond timing.

/// Pulse amplitude, dimensionless (drive strength 0..=1).
pub type Amplitude = f32;

/// Carrier frequency in Hz.
pub type Frequency = f64;

/// Phase offset in radians.
pub type Phase = f32;

/// Time offset from sequence start, in nanoseconds.
pub type TimestampNs = i64;

/// Duration of a pulse, in nanoseconds.
pub type DurationNs = u32;

/// A microwave control pulse targeting a single qubit channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pulse {
    /// Qubit/channel index this pulse is routed to.
    pub channel: usize,
    /// Start time relative to sequence start, in nanoseconds.
    pub start_ns: TimestampNs,
    /// Pulse length, in nanoseconds.
    pub duration_ns: DurationNs,
    /// Peak amplitude (0..=1).
    pub amplitude: Amplitude,
    /// Carrier frequency in Hz.
    pub frequency_hz: Frequency,
    /// Phase offset in radians.
    pub phase: Phase,
    /// Envelope shape.
    pub shape: Envelope,
}

/// Pulse envelope shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Envelope {
    /// Rectangular (constant amplitude).
    Square,
    /// Raised-cosine (Gaussian-derivative-like) smoothing.
    Gaussian,
    /// Drag-corrected Gaussian (used for leakage suppression).
    Drag,
}

impl Pulse {
    /// End time of this pulse, in nanoseconds (exclusive).
    pub fn end_ns(&self) -> TimestampNs {
        self.start_ns + self.duration_ns as TimestampNs
    }
}
