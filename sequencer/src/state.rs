// SPDX-License-Identifier: MIT OR Apache-2.0

//! Qubit state representation.
//!
//! A pure quantum state over `n` qubits is stored as a complex state vector of
//! length `2^n`. The amplitude of basis state `|i>` is accessed at index `i`.

use core::ops::Index;
use num_complex::Complex32;

/// A quantum state vector of `2^n` complex amplitudes.
#[derive(Clone, Debug)]
pub struct QubitState {
    /// Number of qubits.
    num_qubits: usize,
    /// Amplitudes, indexed by computational basis state.
    amplitudes: alloc::vec::Vec<Complex32>,
}

impl QubitState {
    /// Create the all-zero `|00...0>` state over `num_qubits` qubits.
    pub fn zero(num_qubits: usize) -> Self {
        let mut amplitudes = alloc::vec::Vec::with_capacity(1 << num_qubits);
        for i in 0..(1usize << num_qubits) {
            amplitudes.push(if i == 0 {
                Complex32::new(1.0, 0.0)
            } else {
                Complex32::new(0.0, 0.0)
            });
        }
        Self {
            num_qubits,
            amplitudes,
        }
    }

    /// Number of qubits represented by this state.
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Dimension of the state vector (`2^num_qubits`).
    pub fn dim(&self) -> usize {
        self.amplitudes.len()
    }

    /// Squared magnitude (probability) of basis state `index`.
    pub fn probability(&self, index: usize) -> f32 {
        self.amplitudes[index].norm_sqr()
    }

    /// Inner mutating access to amplitudes for gate application.
    pub(crate) fn amplitudes_mut(&mut self) -> &mut alloc::vec::Vec<Complex32> {
        &mut self.amplitudes
    }

    /// Verify the state vector is normalized to within `tol`.
    pub fn is_normalized(&self, tol: f32) -> bool {
        let total: f32 = self.amplitudes.iter().map(|a| a.norm_sqr()).sum();
        (total - 1.0).abs() <= tol
    }
}

impl Index<usize> for QubitState {
    type Output = Complex32;
    fn index(&self, index: usize) -> &Complex32 {
        &self.amplitudes[index]
    }
}
