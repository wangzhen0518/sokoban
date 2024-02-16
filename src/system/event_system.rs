use std::collections::HashMap;

use specs::{Entities, Join, ReadStorage, System, Write};

use crate::{
    aduio::{AudioStore, Sound},
    components::{Box, BoxSpot, Position},
    events::{BoxPlacedOnSpot, EntityMoved, Event},
    resources::EventQueue,
};

pub struct EventSystem<'a> {
    pub context: &'a mut ggez::Context,
}

impl<'a> System<'a> for EventSystem<'a> {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, AudioStore>,
        Entities<'a>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, mut audio_store, entities, boxes, box_spots, positions) = data;

        let mut new_events = Vec::new();

        for event in event_queue.events.drain(..) {
            println!("New event: {:?}", event);
            match event {
                Event::PlayerHitObstacle => audio_store.play_sound(self.context, Sound::Wall),
                Event::EntityMoved(EntityMoved { id }) => {
                    if let Some(box_i) = boxes.get(entities.entity(id)) {
                        let box_spots_with_position: HashMap<(u8, u8), &BoxSpot> =
                            (&box_spots, &positions)
                                .join()
                                .map(|(box_spot, pos)| (pos.into(), box_spot))
                                .collect();
                        if let Some(box_pos) = positions.get(entities.entity(id)) {
                            if let Some(box_spot) = box_spots_with_position.get(&box_pos.into()) {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: (box_i.color == box_spot.color),
                                }))
                            }
                        }
                    }
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    if is_correct_spot {
                        audio_store.play_sound(self.context, Sound::Correct)
                    } else {
                        audio_store.play_sound(self.context, Sound::Incorrect)
                    }
                }
            }
        }

        event_queue.events.append(&mut new_events);
    }
}
