use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: vec![],
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarGreen);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = NORTH_EAST;
    player.scale = 1.0;
    player.layer = 1.0;
    player.collision = true;

    let car1 = game.add_sprite("car1", SpritePreset::RacingCarYellow);
    car1.translation = Vec2::new(300.0, 0.0);
    car1.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        if event.state.is_begin() && event.pair.one_starts_with("player") {
            // remove the sprite the player collided with
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }

            game_state.current_score += 1;
            println!("Current Score: {}", game_state.current_score);
        }
    }

    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.x += 100.0 * engine.delta_f32;
}
