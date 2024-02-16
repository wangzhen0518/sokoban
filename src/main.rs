use std::path;

use ggez::{conf, event};
use ggez::{ContextBuilder, GameResult};
use rust_sokoban::aduio::initialize_sounds;
use specs::{World, WorldExt};

use rust_sokoban::initialize_level;
use rust_sokoban::Game;
use rust_sokoban::{components, resources};

fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);
    resources::register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ContextBuilder::new("rust_sokoban", "Wang Zhen")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));
    let (mut context, event_loop) = context_builder.build()?;
    initialize_sounds(&mut world, &mut context);

    let game = Game { world };
    event::run(context, event_loop, game);
}
