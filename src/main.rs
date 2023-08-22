use rand::prelude::*;
use rusty_engine::prelude::*;
use rusty_vmo_tank2d::VmoHost;

const GRAVITATIONAL_CONSTANT: f32 = -20.8;
const FLOW_CONSTANT: f32 = 40.0;
const FRICTIONAL_FORCE: f32 = 0.1;
const ROTI_ROTATIONAL_ACCEL: f32 = 0.2;
const ROTI_ACCEL: f32 = 1.0;

struct GameState {
    control_module: VmoHost,
    manual_control: bool,
    roti_linear_velocity: f32,
    roti_rotational_velocity: f32,
    food_colllected: i32,
}

fn main() {
    // Create a game

    let mut game = Game::new();

    let sprite = game.add_sprite("roti", SpritePreset::RacingBarrelRed);
    sprite.scale = 1.0;
    sprite.collision = true;

    let velocity_param_display = game.add_text(
        "velocity_param_display",
        "Velocity: 0     Rotation: 0     Food: 0",
    );
    velocity_param_display.translation = Vec2::new(250.0, 320.0);

    let obstacle_presets = vec![SpritePreset::RacingBarrelBlue];

    for (i, preset) in (0..5).into_iter().enumerate() {
        let obst = game.add_sprite(format!("food{}", i), SpritePreset::RollingBallBlue);
        obst.layer = 5.0;
        obst.collision = true;
        obst.translation.x = thread_rng().gen_range(-550.00..550.0);
        obst.translation.y = thread_rng().gen_range(-250.0..250.0);
    }

    // Add one or more functions with logic for your game. When the game is run, the logic
    // functions will run in the order they were added.
    game.add_logic(game_logic);
    // Run the game, with an initial state
    //
    let control_module = VmoHost {};

    let initial_game_state = GameState {
        control_module,
        manual_control: true,
        roti_linear_velocity: 0.0,
        roti_rotational_velocity: 0.0,
        food_colllected: 0,
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

    //switch between VmoHost and manual control
    if engine.keyboard_state.pressed(KeyCode::K) {
        game_state.manual_control = !game_state.manual_control;
    }

    if game_state.manual_control {
        if engine.keyboard_state.pressed(KeyCode::Right) {
            game_state.roti_rotational_velocity =
                game_state.roti_rotational_velocity + ROTI_ROTATIONAL_ACCEL
        };

        if engine.keyboard_state.pressed(KeyCode::Up) {
            game_state.roti_linear_velocity = game_state.roti_linear_velocity + ROTI_ACCEL
        };
    } else {
        game_state.roti_linear_velocity = game_state.roti_linear_velocity + game_state.control_module.fwd_accel();
        game_state.roti_rotational_velocity = game_state.control_module.rot_accel();
    }

    game_state.roti_rotational_velocity = game_state.roti_rotational_velocity - FRICTIONAL_FORCE;
    game_state.roti_linear_velocity = game_state.roti_linear_velocity - FRICTIONAL_FORCE;

    if game_state.roti_rotational_velocity < 0.0 {
        game_state.roti_rotational_velocity = 0.0
    };

    if game_state.roti_rotational_velocity > 180.0 {
        game_state.roti_rotational_velocity = 180.0
    };

    if game_state.roti_linear_velocity > 500.0 {
        game_state.roti_linear_velocity = 500.0
    };

    if game_state.roti_linear_velocity < 0.0 {
        game_state.roti_linear_velocity = 0.0
    };

    roti.rotation += std::f32::consts::PI * engine.delta_f32 * game_state.roti_rotational_velocity;

    let x_vector_vel = game_state.roti_linear_velocity * roti.rotation.cos() + FLOW_CONSTANT;
    let y_vector_vel =
        game_state.roti_linear_velocity * roti.rotation.sin() + GRAVITATIONAL_CONSTANT;

    let mut translation_x = roti.translation.x + engine.delta_f32 * x_vector_vel;
    let mut translation_y = roti.translation.y + engine.delta_f32 * y_vector_vel;

    if translation_x < -550.0 {
        translation_x = 550.0;
    } else if translation_x > 550.0 {
        translation_x = -550.0;
    }

    if translation_y < -250.0 {
        translation_y = -250.0;
    } else if translation_y > 250.0 {
        translation_y = 250.0;
    }

    roti.translation.x = translation_x;
    roti.translation.y = translation_y;

    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("roti") || event.state.is_end() {
            continue;
        }

        game_state.food_colllected += 1;

        let fb = if event.pair.1 == "roti" {
            event.pair.0
        } else {
            event.pair.1
        };

        let food_b = engine.sprites.get_mut(&fb).unwrap();

        food_b.translation.x = thread_rng().gen_range(-550.00..550.0);
        food_b.translation.y = thread_rng().gen_range(-250.0..250.0);
    }

    let velocity_param_display = engine.texts.get_mut("velocity_param_display").unwrap();
    velocity_param_display.value = format!(
        "Velocity: {:.0}     Rotation: {:.0}     Food: {}",
        game_state.roti_linear_velocity,
        game_state.roti_rotational_velocity,
        game_state.food_colllected
    );
}
