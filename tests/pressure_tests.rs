use bevy::prelude::*;
use rust_station_13::game_code::pressure::*;

const TEST_WORLD_SIZE: (i64, i64) = (32, 32);

#[test]
fn simple_pressure_test_1() {
    // Prepare the world
    let mut app = App::new();
    let app = app.add_plugins(PressurePlugin { world_size: TEST_WORLD_SIZE })
        .add_systems(Startup, simple_pressure_test_1_setup);
    

    // Run a update to ensure stability:
    app.update();
    
    // Compare the values against expectations:
    
}

fn simple_pressure_test_1_setup(mut commands: Commands) {

}