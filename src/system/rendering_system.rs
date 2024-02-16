use std::{collections::HashMap, time::Duration};

use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam, Image},
    timer, Context,
};
use itertools::Itertools;
use specs::{Join, Read, ReadStorage, System};

use crate::{
    components::{Position, Renderable, RenderableKind},
    constants,
    resources::{GamePlay, Time},
};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, GamePlay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;

        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let rendering_data: Vec<(&Position, &Renderable)> =
            (&positions, &renderables).join().collect();
        // rendering_data.sort_by_key(|&k| k.0.z);
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        for (pos, item) in rendering_data.iter() {
            // let image = self.get_image(item, time.delta);
            let image_path = self.get_image(item, time.delta);

            let x = pos.x as f32 * constants::TILE_WIDTH;
            let y = pos.y as f32 * constants::TILE_WIDTH;
            let z = pos.z;

            let draw_param = DrawParam::new().dest([x, y]);
            // graphics::draw(self.context, &image, draw_param).expect("expected render");
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_param);
        }

        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|(zi, _), (zj, _)| Ord::cmp(zi, zj))
        {
            for (image_path, draw_param_batch) in group.iter() {
                let image = Image::new(self.context, image_path).expect("expected image");
                let mut sprite_batch = SpriteBatch::new(image);
                for &draw_param in draw_param_batch.iter() {
                    sprite_batch.add(draw_param);
                }
                graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                    .expect("expected render");
            }
        }

        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);
        let fps = format!("FPS: {:.0}", timer::fps(self.context));
        self.draw_text(&fps, 525.0, 120.0);

        graphics::present(self.context).expect("expect to present");
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = [x, y];
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = [0.0, 20.0];
        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }
    // pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> Image {
    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> String {
        let path_index = match renderable.kind() {
            RenderableKind::Static => 0,
            RenderableKind::Animated => ((delta.as_millis() % 1000) / 250) as usize,
        };
        // let image_path = renderable.path(path_index);
        // Image::new(self.context, image_path).expect("expected image")
        renderable.path(path_index)
    }
}
