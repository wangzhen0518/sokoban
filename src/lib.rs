use ggez::{event, timer, Context, GameError};
use resources::Time;
use specs::{RunNow, World, WorldExt};
use system::EventSystem;

pub mod aduio;
pub mod components;
pub mod constants;
pub mod entities;
pub mod events;
pub mod map;
pub mod resources;
pub mod system;

use crate::{
    resources::InputQueue,
    system::{GameplaySystem, InputSystem, RenderingSystem},
};

pub struct Game {
    pub world: World,
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }
        {
            let mut gss = GameplaySystem {};
            gss.run_now(&self.world);
        }
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(_ctx);
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        {
            let mut rs = RenderingSystem { context: _ctx };
            rs.run_now(&self.world);
        }
        {
            let mut es = EventSystem { context: _ctx };
            es.run_now(&self.world);
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", _keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.key_pressed.push(_keycode);
    }
}

pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
        N N W W  W  W  W W
        W W W .  .  .  . W
        W . . .  BB .  . W
        W . . RB .  .  . W 
        W . P .  .  .  . W
        W . . .  .  RS . W
        W . . BS .  .  . W
        W . . .  .  .  . W
        W W W W  W  W  W W
        ";
    map::load_map(world, MAP);
}
