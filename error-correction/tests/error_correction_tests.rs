// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for the surface-code error correction layer.

use error_correction::{CodeDistance, Pauli, SurfaceCode};

#[test]
fn code_size_scales_with_distance() {
    assert_eq!(CodeDistance(3).physical_qubits(), 9);
    assert_eq!(CodeDistance(5).physical_qubits(), 25);
}

#[test]
fn no_errors_means_no_syndrome() {
    let code = SurfaceCode::new(CodeDistance(3));
    let syn = code.measure_syndrome();
    assert!(syn.fired.iter().all(|f| !f));
}

#[test]
fn single_error_produces_syndrome_and_is_corrected() {
    let mut code = SurfaceCode::new(CodeDistance(3));
    code.inject_error(0, Pauli::X);
    let syn = code.measure_syndrome();
    assert!(syn.fired.iter().any(|f| *f));
    let corrections = code.decode_and_correct(&syn);
    assert!(!corrections.is_empty());
    // After correction the syndrome should be clean.
    let syn2 = code.measure_syndrome();
    assert!(syn2.fired.iter().all(|f| !f));
}

#[test]
fn corrections_are_dispatched_as_pulses() {
    let mut code = SurfaceCode::new(CodeDistance(3));
    code.inject_error(0, Pauli::X);
    let syn = code.measure_syndrome();
    let gates = code.decode_and_correct(&syn);
    let pulses = code.corrections_to_pulses(&gates, 0);
    assert_eq!(pulses.len(), gates.len());
    for p in &pulses {
        assert_eq!(p.duration_ns, 20);
    }
}
