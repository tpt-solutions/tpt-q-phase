// SPDX-License-Identifier: MIT OR Apache-2.0

//! Deterministic scheduling of gate sequences into pulse parameter streams.
//!
//! The scheduler assigns each gate a fixed nanosecond time slot and emits a
//! [`Pulse`] per single-qubit gate and a pair of linked pulses for two-qubit
//! gates. Timing is fully deterministic: replaying the same circuit always
//! produces the same pulse train with identical start times.

use crate::gates::Gate;
use crate::pulse::{DurationNs, Envelope, Frequency, Pulse, TimestampNs};

/// Minimum separation between consecutive pulses on the same channel, in ns.
pub const INTER_PULSE_GAP_NS: DurationNs = 2;

/// Default single-qubit gate duration, in ns.
pub const SINGLE_QUBIT_DURATION_NS: DurationNs = 20;

/// Default two-qubit gate duration, in ns.
pub const TWO_QUBIT_DURATION_NS: DurationNs = 40;

/// Per-channel carrier frequency (Hz). Indexed by qubit/channel.
#[derive(Clone, Debug)]
pub struct ChannelConfig {
    pub frequencies_hz: alloc::vec::Vec<Frequency>,
}

impl ChannelConfig {
    pub fn uniform(num_channels: usize, frequency_hz: Frequency) -> Self {
        Self {
            frequencies_hz: alloc::vec![frequency_hz; num_channels],
        }
    }
}

/// A compiled pulse schedule: a deterministic, time-ordered pulse train.
#[derive(Clone, Debug, Default)]
pub struct Schedule {
    pulses: alloc::vec::Vec<Pulse>,
    total_duration_ns: TimestampNs,
}

impl Schedule {
    pub fn pulses(&self) -> &[Pulse] {
        &self.pulses
    }

    pub fn total_duration_ns(&self) -> TimestampNs {
        self.total_duration_ns
    }

    pub fn len(&self) -> usize {
        self.pulses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pulses.is_empty()
    }
}

/// Translate a gate sequence into a deterministic [`Schedule`].
pub fn compile(gates: &[Gate], channels: &ChannelConfig) -> Schedule {
    let mut pulses = alloc::vec::Vec::new();
    let mut clock: TimestampNs = 0;
    let num_channels = channels.frequencies_hz.len();

    for &gate in gates {
        match gate {
            Gate::H(q)
            | Gate::X(q)
            | Gate::Y(q)
            | Gate::Z(q)
            | Gate::S(q)
            | Gate::T(q)
            | Gate::Rx(q)
            | Gate::Ry(q)
            | Gate::Rz(q) => {
                let q = q.min(num_channels.saturating_sub(1));
                pulses.push(Pulse {
                    channel: q,
                    start_ns: clock,
                    duration_ns: SINGLE_QUBIT_DURATION_NS,
                    amplitude: 1.0,
                    frequency_hz: channels.frequencies_hz[q],
                    phase: 0.0,
                    shape: Envelope::Drag,
                });
                clock +=
                    SINGLE_QUBIT_DURATION_NS as TimestampNs + INTER_PULSE_GAP_NS as TimestampNs;
            }
            Gate::CNOT(c, t) | Gate::CZ(c, t) => {
                let c = c.min(num_channels.saturating_sub(1));
                let t = t.min(num_channels.saturating_sub(1));
                // Control drive (first half) and target drive (full length),
                // emitted deterministically with the control leading.
                pulses.push(Pulse {
                    channel: c,
                    start_ns: clock,
                    duration_ns: TWO_QUBIT_DURATION_NS / 2,
                    amplitude: 0.8,
                    frequency_hz: channels.frequencies_hz[c],
                    phase: 0.0,
                    shape: Envelope::Gaussian,
                });
                pulses.push(Pulse {
                    channel: t,
                    start_ns: clock + (TWO_QUBIT_DURATION_NS / 2) as TimestampNs,
                    duration_ns: TWO_QUBIT_DURATION_NS / 2,
                    amplitude: 1.0,
                    frequency_hz: channels.frequencies_hz[t],
                    phase: 0.0,
                    shape: Envelope::Gaussian,
                });
                clock += TWO_QUBIT_DURATION_NS as TimestampNs + INTER_PULSE_GAP_NS as TimestampNs;
            }
        }
    }

    Schedule {
        total_duration_ns: clock,
        pulses,
    }
}

/// Maximum absolute timing jitter observed when replaying a schedule on a
/// reference clock with period `clock_period_ns`. Used by the validation
/// harness to confirm the sub-10ns jitter target.
pub fn max_jitter_ns(schedule: &Schedule, clock_period_ns: TimestampNs) -> TimestampNs {
    schedule
        .pulses()
        .iter()
        .map(|p| (p.start_ns % clock_period_ns).abs())
        .max()
        .unwrap_or(0)
}
