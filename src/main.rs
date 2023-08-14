 use rusty_engine::prelude::*;

const GRAVITATIONAL_CONSTANT: f32 = -9.8;
const FLOW_CONSTANT: f32 = 0.0;
const FRICTIONAL_FORCE: f32 = 0.1;
const ROTI_ROTATIONAL_ACCEL: f32 = 0.2;

 struct GameState {
     roti_linear_velocity: f32,
     roti_rotational_velocity: f32,
 }

 fn main() {
     // Create a game
     let mut game = Game::new();

     let sprite = game.add_sprite("roti", SpritePreset::RacingBarrelRed);
     sprite.scale = 1.0;

     let velocity_param_display = game.add_text("velocity_param_display", "Velocity: 0.0      Rotation: 0.0");
     velocity_param_display.translation = Vec2::new(250.0, 320.0);

     game.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);
     // Add one or more functions with logic for your game. When the game is run, the logic
     // functions will run in the order they were added.
     game.add_logic(game_logic);
     // Run the game, with an initial state
     let initial_game_state = GameState {
         roti_linear_velocity: 0.0,
         roti_rotational_velocity: 0.0

     };
     game.run(initial_game_state);
 }

 // Your game logic functions can be named anything, but the first parameter is always a
 // `&mut Engine`, and the second parameter is a mutable reference to your custom game
 // state struct (`&mut GameState` in this case).
 //
 // This function will be run once each frame.
 fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
     let roti = engine.sprites.get_mut("roti").unwrap();

    if engine.keyboard_state.pressed(KeyCode::Right) {
        game_state.roti_rotational_velocity = game_state.roti_rotational_velocity + ROTI_ROTATIONAL_ACCEL
    };

    game_state.roti_rotational_velocity = game_state.roti_rotational_velocity - FRICTIONAL_FORCE;

    if game_state.roti_rotational_velocity < 0.0 {
        game_state.roti_rotational_velocity = 0.0
    }

    roti.rotation += std::f32::consts::PI * engine.delta_f32 * game_state.roti_rotational_velocity;

    let velocity_param_display = engine.texts.get_mut("velocity_param_display").unwrap();
    velocity_param_display.value = format!("Velocity: {}     Rotation: {}", game_state.roti_linear_velocity, game_state.roti_rotational_velocity);

 }
