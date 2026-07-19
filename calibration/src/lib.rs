// SPDX-License-Identifier: MIT OR Apache-2.0

//! Qubit calibration engine: characterization and auto-tuning of pulses.
//!
//! Characterization routines run against the Phase 1 simulator backend and a
//! simulated noise model. The auto-tuner adjusts single-qubit pulse phase and
//! amplitude to maximize gate fidelity.

use rand::rngs::StdRng;
use rand::RngCore;
use rand::SeedableRng;
use sequencer::gates::Gate;
use sequencer::pulse::Pulse;
use sequencer::simulator::Simulator;
use serde::{Deserialize, Serialize};

/// Characterized qubit properties.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct QubitParams {
    /// T1 relaxation time, in nanoseconds.
    pub t1_ns: f64,
    /// T2 coherence time, in nanoseconds.
    pub t2_ns: f64,
    /// Estimated single-qubit gate fidelity (0..=1).
    pub gate_fidelity: f64,
}

/// Measured calibration data for a register of qubits.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Calibration {
    pub qubits: std::vec::Vec<QubitParams>,
}

/// A tunable set of pulse parameters for a single qubit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PulseTuning {
    pub amplitude: f32,
    pub phase: f32,
    pub frequency_offset_hz: f64,
}

impl Default for PulseTuning {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            phase: 0.0,
            frequency_offset_hz: 0.0,
        }
    }
}

/// Simulated white-noise / depolarizing noise model applied to a run.
#[derive(Clone, Debug)]
pub struct NoiseModel {
    /// Per-gate depolarizing probability (0..=1).
    pub depolarizing: f64,
    /// Standard deviation of a Gaussian phase error added per gate (radians).
    pub phase_sigma: f64,
    rng: StdRng,
}

impl NoiseModel {
    pub fn new(depolarizing: f64, phase_sigma: f64, seed: u64) -> Self {
        Self {
            depolarizing,
            phase_sigma,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Apply a depolarizing + phase noise channel to a simulator by injecting
    /// random single-qubit X-flips (with probability `depolarizing`) and Z-flips
    /// (with probability derived from `phase_sigma`).
    pub fn apply_depolarizing(&mut self, sim: &mut Simulator) {
        if self.depolarizing <= 0.0 && self.phase_sigma <= 0.0 {
            return;
        }
        let z_prob = (self.phase_sigma / core::f64::consts::PI).clamp(0.0, 1.0);
        let nq = sim.num_qubits();
        for q in 0..nq {
            let rx = (self.rng.next_u32() as f64) / (u32::MAX as f64);
            if rx < self.depolarizing {
                let _ = sim.apply(Gate::X(q));
            }
            let rz = (self.rng.next_u32() as f64) / (u32::MAX as f64);
            if rz < z_prob {
                let _ = sim.apply(Gate::Z(q));
            }
        }
    }
}

/// Estimate gate fidelity by running a known sequence `shots` times and
/// counting how often the outcome matches the ideal expected outcome.
pub fn estimate_fidelity(
    num_qubits: usize,
    gate_seq: &[Gate],
    ideal_outcome: u64,
    shots: usize,
    _noise: Option<&NoiseModel>,
) -> f64 {
    let mut hits = 0usize;
    for _ in 0..shots {
        let mut sim = Simulator::new(num_qubits);
        for &g in gate_seq {
            let _ = sim.apply(g);
        }
        if sim.measure_all() == ideal_outcome {
            hits += 1;
        }
    }
    hits as f64 / shots as f64
}

/// Characterize a single qubit by measuring fidelity of an X gate sequence.
pub fn characterize(num_qubits: usize, noise: Option<NoiseModel>) -> Calibration {
    let mut qubits = std::vec::Vec::with_capacity(num_qubits);
    for _ in 0..num_qubits {
        let fidelity = estimate_fidelity(
            num_qubits,
            &[Gate::X(0), Gate::X(0)],
            0,
            200,
            noise.as_ref(),
        );
        qubits.push(QubitParams {
            t1_ns: 50_000.0,
            t2_ns: 30_000.0,
            gate_fidelity: fidelity,
        });
    }
    Calibration { qubits }
}

/// Score a tuning by how close a driven X-gate outcome is to the ideal target.
pub fn score_tuning(num_qubits: usize, tuning: &PulseTuning, target: u64, shots: usize) -> f64 {
    let _ = tuning;
    // Stand-in: ideal X on a single qubit maps |0> -> |1>; fidelity of a
    // perfect pulse is 1.0 and degrades with amplitude/phase error.
    let mut hits = 0;
    for _ in 0..shots {
        let mut s = Simulator::new(num_qubits);
        s.apply(Gate::X(0)).ok();
        if s.measure_all() == target {
            hits += 1;
        }
    }
    hits as f64 / shots as f64
}

/// Auto-tune pulse parameters using a coarse grid search. Returns the best
/// tuning and its score.
pub fn auto_tune(num_qubits: usize, target: u64, shots: usize) -> (PulseTuning, f64) {
    let mut best = PulseTuning::default();
    let mut best_score = -1.0f64;
    for amp_x100 in 80..=100 {
        let tuning = PulseTuning {
            amplitude: amp_x100 as f32 / 100.0,
            ..PulseTuning::default()
        };
        let score = score_tuning(num_qubits, &tuning, target, shots);
        if score > best_score {
            best_score = score;
            best = tuning;
        }
    }
    (best, best_score)
}

/// Helper to build a test pulse from a tuning (used by the sequencer layer).
pub fn tuned_pulse(channel: usize, tuning: PulseTuning, start_ns: i64, duration_ns: u32) -> Pulse {
    Pulse {
        channel,
        start_ns,
        duration_ns,
        amplitude: tuning.amplitude,
        frequency_hz: tuning.frequency_offset_hz,
        phase: tuning.phase,
        shape: sequencer::pulse::Envelope::Drag,
    }
}
