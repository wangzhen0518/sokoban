use std::{fmt::Display, time::Duration};

use ggez::event::KeyCode;
use specs::World;

use crate::{aduio::AudioStore, events::Event};

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(GamePlay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}

#[derive(Default)]
pub struct InputQueue {
    pub key_pressed: Vec<KeyCode>,
}

#[derive(Debug)]
pub enum GamePlayState {
    Playing,
    Won,
}

impl Display for GamePlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GamePlayState::Playing => "Playing",
            GamePlayState::Won => "Won",
        })
    }
}

impl Default for GamePlayState {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Default, Debug)]
pub struct GamePlay {
    pub state: GamePlayState,
    pub moves_count: u32,
}

#[derive(Default, Debug)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default, Debug)]
pub struct EventQueue {
    pub events: Vec<Event>,
}
