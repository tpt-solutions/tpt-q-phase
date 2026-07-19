// SPDX-License-Identifier: MIT OR Apache-2.0

//! Simulated cryostat backend for development without real hardware.

use crate::state::{HeaterControl, Stage, ThermalState};

/// A simple lumped thermal model: each stage relaxes toward a cooling target
/// (set by its nominal temperature) while heater power pushes it upward.
/// Adjacent stages exchange a small heat flow so cooling cascades downward.
pub struct SimulatedCryostat {
    state: ThermalState,
    powers: [f64; 4],
}

impl SimulatedCryostat {
    pub fn new() -> Self {
        Self {
            state: ThermalState {
                room_k: Stage::Room.nominal_temperature_k(),
                plate_4k_k: Stage::Plate4K.nominal_temperature_k(),
                plate_100mk_k: Stage::Plate100mK.nominal_temperature_k(),
                mixing_chamber_k: Stage::MixingChamber.nominal_temperature_k(),
            },
            powers: [0.0; 4],
        }
    }

    pub fn thermal_state(&self) -> &ThermalState {
        &self.state
    }

    /// Advance the thermal simulation by `dt` seconds.
    pub fn step(&mut self, dt: f64) {
        let targets = [
            Stage::Room.nominal_temperature_k(),
            Stage::Plate4K.nominal_temperature_k(),
            Stage::Plate100mK.nominal_temperature_k(),
            Stage::MixingChamber.nominal_temperature_k(),
        ];
        let mut next = [
            self.state.room_k,
            self.state.plate_4k_k,
            self.state.plate_100mk_k,
            self.state.mixing_chamber_k,
        ];
        for (i, t) in targets.iter().enumerate() {
            // Relaxation toward cooling target.
            let relax = (t - next[i]) * 0.05;
            // Heater adds warmth, scaled per stage (smaller stages heat faster).
            let heat = self.powers[i] * (1.0 / (i as f64 + 1.0)) * 0.5;
            next[i] += (relax + heat) * dt;
        }
        // Cascade: colder stages pull a little heat from warmer neighbors.
        for i in (1..4).rev() {
            let flow = (next[i - 1] - next[i]) * 0.001 * dt;
            next[i - 1] -= flow;
            next[i] += flow;
        }
        self.state.room_k = next[0];
        self.state.plate_4k_k = next[1];
        self.state.plate_100mk_k = next[2];
        self.state.mixing_chamber_k = next[3];
    }
}

impl HeaterControl for SimulatedCryostat {
    fn set_power(&mut self, stage: Stage, power: f64) {
        let idx = stage_index(stage);
        self.powers[idx] = power.clamp(0.0, 1.0);
    }

    fn power(&self, stage: Stage) -> f64 {
        self.powers[stage_index(stage)]
    }
}

fn stage_index(s: Stage) -> usize {
    match s {
        Stage::Room => 0,
        Stage::Plate4K => 1,
        Stage::Plate100mK => 2,
        Stage::MixingChamber => 3,
    }
}

impl Default for SimulatedCryostat {
    fn default() -> Self {
        Self::new()
    }
}
