use bevy::prelude::*;

const GAS_CONSTANT: f64 = 8.314;

#[derive(Component)]
pub struct GasData {
    pub name: String,
    pub moles: f64,
    pub temperature: f64,
}

#[derive(Component)]
pub struct Pressure {
    pub pascal: f64
}

#[derive(Component)]
pub struct Passable {
    pub passable: bool,
}

pub struct PressurePlugin {
    pub world_size: (i64, i64),
}

impl Plugin for PressurePlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn setup_pressure_system(mut commands: Commands) {
    
}