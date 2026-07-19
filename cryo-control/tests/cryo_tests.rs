// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for the cryogenic control layer against simulated thermal profiles.

use cryo_control::{regulate, HeaterControl, PidController, SimulatedCryostat, Stage};

#[test]
fn nominal_temperatures_descend() {
    assert!(Stage::Room.nominal_temperature_k() > Stage::Plate4K.nominal_temperature_k());
    assert!(Stage::Plate4K.nominal_temperature_k() > Stage::Plate100mK.nominal_temperature_k());
    assert!(
        Stage::Plate100mK.nominal_temperature_k() > Stage::MixingChamber.nominal_temperature_k()
    );
}

#[test]
fn heater_raises_stage_temperature() {
    let mut cryo = SimulatedCryostat::new();
    let target = Stage::MixingChamber.nominal_temperature_k();
    cryo.set_power(Stage::MixingChamber, 0.9);
    for _ in 0..200 {
        cryo.step(1.0);
    }
    assert!(cryo.thermal_state().mixing_chamber_k > target);
}

#[test]
fn pid_holds_mixing_chamber_near_target() {
    let mut cryo = SimulatedCryostat::new();
    let mut pid = PidController::new(0.8, 0.05, 0.1);
    let stage = Stage::MixingChamber;
    let target = stage.nominal_temperature_k();
    for _ in 0..500 {
        let current = cryo.thermal_state().temperature(stage);
        regulate(&mut pid, &mut cryo, stage, current, target, 1.0);
        cryo.step(1.0);
    }
    let final_t = cryo.thermal_state().temperature(stage);
    assert!(
        (final_t - target).abs() < 0.02,
        "final temp {final_t} vs target {target}"
    );
}

#[test]
fn pid_power_clamped() {
    let mut cryo = SimulatedCryostat::new();
    let mut pid = PidController::new(5.0, 0.0, 0.0);
    let stage = Stage::MixingChamber;
    let p = regulate(
        &mut pid,
        &mut cryo,
        stage,
        1.0,
        stage.nominal_temperature_k(),
        1.0,
    );
    assert!((0.0..=1.0).contains(&p));
}
