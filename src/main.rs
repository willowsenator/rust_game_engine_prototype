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
    player.translation = Vec2::new(400.0, 300.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    game_state.current_score += 1;
}
