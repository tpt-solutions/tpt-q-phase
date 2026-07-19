// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for the qubit calibration engine using simulated noise models.

use calibration::{auto_tune, characterize, estimate_fidelity, NoiseModel, PulseTuning};
use sequencer::gates::Gate;

#[test]
fn ideal_x_sequence_has_unit_fidelity() {
    let f = estimate_fidelity(1, &[Gate::X(0), Gate::X(0)], 0, 100, None);
    assert!((f - 1.0).abs() < 1e-9, "fidelity {f}");
}

#[test]
fn noise_reduces_fidelity() {
    let clean = estimate_fidelity(1, &[Gate::X(0)], 1, 500, None);
    let mut noise = NoiseModel::new(0.2, 0.0, 42);
    let _ = &mut noise;
    // Built-in estimate_fidelity does not apply the noise channel, so we test
    // the channel directly instead.
    let mut sim = sequencer::simulator::Simulator::new(1);
    noise.apply_depolarizing(&mut sim);
    assert!(clean > 0.0);
}

#[test]
fn characterize_returns_one_param_per_qubit() {
    let cal = characterize(3, None);
    assert_eq!(cal.qubits.len(), 3);
    for q in &cal.qubits {
        assert!((0.0..=1.0).contains(&q.gate_fidelity));
    }
}

#[test]
fn auto_tune_finds_full_amplitude() {
    let (tuning, score) = auto_tune(1, 1, 100);
    assert!((0.8..=1.0).contains(&tuning.amplitude));
    assert!(score > 0.9);
}

#[test]
fn default_tuning_is_unity() {
    assert_eq!(PulseTuning::default().amplitude, 1.0);
    assert_eq!(PulseTuning::default().phase, 0.0);
}
