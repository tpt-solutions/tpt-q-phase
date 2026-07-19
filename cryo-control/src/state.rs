// SPDX-License-Identifier: MIT OR Apache-2.0

//! Cryogenic control layer (software model).
//!
//! Defines the temperature-stage model, a heater-control abstraction, a
//! simulated cryostat backend, and a PID control loop. All of this runs
//! without real hardware and is intended to be replaced by physical sensor
//! and heater drivers in Phase 8.

use serde::{Deserialize, Serialize};

/// Cooling stages of a dilution refrigerator, from room temperature down to
/// the mixing-chamber plate where the qubits live.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stage {
    /// Room temperature (300 K) — top plate.
    Room,
    /// 4 K plate.
    Plate4K,
    /// 100 mK plate.
    Plate100mK,
    /// 10 mK mixing-chamber plate (qubit operating temperature).
    MixingChamber,
}

impl Stage {
    /// Nominal target temperature for each stage in Kelvin.
    pub fn nominal_temperature_k(&self) -> f64 {
        match self {
            Stage::Room => 300.0,
            Stage::Plate4K => 4.0,
            Stage::Plate100mK => 0.1,
            Stage::MixingChamber => 0.01,
        }
    }

    /// All stages in order, hottest first.
    pub fn all() -> [Stage; 4] {
        [
            Stage::Room,
            Stage::Plate4K,
            Stage::Plate100mK,
            Stage::MixingChamber,
        ]
    }
}

/// A snapshot of temperatures across all cryostat stages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThermalState {
    pub room_k: f64,
    pub plate_4k_k: f64,
    pub plate_100mk_k: f64,
    pub mixing_chamber_k: f64,
}

impl ThermalState {
    pub fn at(stage: Stage) -> f64 {
        stage.nominal_temperature_k()
    }

    pub fn temperature(&self, stage: Stage) -> f64 {
        match stage {
            Stage::Room => self.room_k,
            Stage::Plate4K => self.plate_4k_k,
            Stage::Plate100mK => self.plate_100mk_k,
            Stage::MixingChamber => self.mixing_chamber_k,
        }
    }
}

/// Heater actuator abstraction. The physical implementation (Phase 8) drives a
/// real heater; the simulator below integrates a thermal model instead.
pub trait HeaterControl {
    /// Set heater power (0.0 ..= 1.0) on the given stage.
    fn set_power(&mut self, stage: Stage, power: f64);
    /// Read current heater power on the given stage.
    fn power(&self, stage: Stage) -> f64;
}
