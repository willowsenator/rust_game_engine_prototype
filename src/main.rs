use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    score: u32,
    block_index: i32,
    //spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            block_index: 0,
            //spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();

    add_player_sprite(&mut game);
    add_ui(&mut game);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    handle_collision_events(engine, game_state);
    handle_player_movement(engine);
    handle_block_mouse_input(engine, game_state);
}

fn add_ui(game: &mut Game<GameState>) {
    let score_text = game.add_text("score", "Score: 0");
    score_text.translation = Vec2::new(520.0, 320.0);
    let high_score_text = game.add_text("high_score", "High Score: 0");
    high_score_text.translation = Vec2::new(-520.0, 320.0);
}

fn add_player_sprite(game: &mut Game<GameState>) {
    let player = game.add_sprite("player", SpritePreset::RacingCarGreen);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = NORTH_EAST;
    player.scale = 1.0;
    player.layer = 1.0;
    player.collision = true;
}

fn handle_collision_events(engine: &mut Engine, game_state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        if event.state.is_begin() && event.pair.one_starts_with("player") {
            // remove the sprite the player collided with
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }

            game_state.score += 1;
            println!("Current Score: {}", game_state.score);
        }
    }
}

fn handle_player_movement(engine: &mut Engine) {
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
}

fn handle_block_mouse_input(engine: &mut Engine, game_state: &mut GameState) {
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("block_{}", game_state.block_index);
            game_state.block_index += 1;
            let block = engine.add_sprite(&label, SpritePreset::RollingBlockSmall);
            block.translation = mouse_location;
            block.collision = true;
        }
    }
}
