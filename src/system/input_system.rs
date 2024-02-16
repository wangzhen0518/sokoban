use std::collections::HashMap;

use ggez::event::KeyCode;
use specs::{world::Index, Entities, Join, ReadStorage, System, Write, WriteStorage};

use crate::{
    components::{Immovable, Movable, Player, Position},
    constants,
    events::{EntityMoved, Event},
    resources::{EventQueue, GamePlay, InputQueue},
};

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, InputQueue>,
        Write<'a, GamePlay>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut event_queue,
            mut input_queue,
            mut gameplay,
            entities,
            mut positions,
            players,
            movables,
            immovables,
        ) = data;

        let mut to_move = Vec::new();

        for (pos, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.key_pressed.pop() {
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|(ent, _mov, pos)| (pos.into(), ent.id()))
                    .collect();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|(ent, _mov, pos)| (pos.into(), ent.id()))
                    .collect();
                let (start, end, is_x) = match key {
                    KeyCode::Up | KeyCode::W => (pos.y, 0, false),
                    KeyCode::Down | KeyCode::S => (pos.y, constants::MAP_HEIGHT, false),
                    KeyCode::Left | KeyCode::A => (pos.x, 0, true),
                    KeyCode::Right | KeyCode::D => (pos.x, constants::MAP_WIDTH, true),
                    _ => continue,
                };

                let range: Vec<u8> = if start < end {
                    (start..=end).collect()
                } else {
                    (end..=start).rev().collect()
                };

                for x_or_y in range {
                    let pos_to_check = if is_x {
                        (x_or_y, pos.y)
                    } else {
                        (pos.x, x_or_y)
                    };

                    match mov.get(&pos_to_check) {
                        Some(id) => to_move.push((key, *id)),
                        None => match immov.get(&pos_to_check) {
                            Some(_) => {
                                to_move.clear();
                                event_queue.events.push(Event::PlayerHitObstacle)
                            }
                            None => break,
                        },
                    }
                }
            }
        }

        if !to_move.is_empty() {
            gameplay.moves_count += 1;
        }

        for (key, id) in to_move {
            if let Some(pos) = positions.get_mut(entities.entity(id)) {
                match key {
                    KeyCode::Up | KeyCode::W => pos.y -= 1,
                    KeyCode::Down | KeyCode::S => pos.y += 1,
                    KeyCode::Left | KeyCode::A => pos.x -= 1,
                    KeyCode::Right | KeyCode::D => pos.x += 1,
                    _ => continue,
                }
            }
            event_queue
                .events
                .push(Event::EntityMoved(EntityMoved { id }));
        }
    }
}
