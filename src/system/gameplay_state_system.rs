use std::collections::HashMap;

use specs::{Join, ReadStorage, System, Write};

use crate::{
    components::{Box, BoxSpot, Position},
    resources::{GamePlay, GamePlayState},
};

pub struct GameplaySystem {}

impl<'a> System<'a> for GameplaySystem {
    type SystemData = (
        Write<'a, GamePlay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay, positions, boxes, box_spots) = data;
        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|(pos_i, box_i)| (pos_i.into(), box_i))
            .collect();
        for (_box_spot, pos) in (&box_spots, &positions).join() {
            if let Some(box_i) = boxes_by_position.get(&pos.into()) {
                if box_i.color != _box_spot.color {
                    gameplay.state = GamePlayState::Playing;

                    return;
                }
            } else {
                gameplay.state = GamePlayState::Playing;
                return;
            }
        }

        gameplay.state = GamePlayState::Won;
    }
}
