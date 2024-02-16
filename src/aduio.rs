use std::{collections::HashMap, fmt::Display};

use ggez::{
    audio::{self, SoundSource},
    Context,
};
use specs::{World, WorldExt};

pub fn initialize_sounds(world: &mut World, context: &mut Context) {
    let mut audio_store = world.write_resource::<AudioStore>();
    let sounds = ["correct", "incorrect", "wall"];
    for sound in sounds {
        audio_store.sounds.insert(
            sound.into(),
            audio::Source::new(context, format!("/sounds/{}.wav", sound)).expect("expected sound"),
        );
    }
}

#[derive(Clone, Copy)]
pub enum Sound {
    Wall,
    Correct,
    Incorrect,
}

impl Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Sound::Wall => "wall",
            Sound::Correct => "correct",
            Sound::Incorrect => "incorrect",
        })
    }
}

impl From<Sound> for String {
    fn from(value: Sound) -> Self {
        format!("{}", value)
    }
}

impl From<&Sound> for String {
    fn from(value: &Sound) -> Self {
        format!("{}", value)
    }
}

#[derive(Debug, Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, audio::Source>,
}

impl AudioStore {
    pub fn play_sound(&mut self, context: &mut Context, sound: Sound) {
        let sound_name = String::from(sound);
        self.sounds
            .get_mut(&sound_name)
            .expect("expect sound play")
            .play_detached(context)
            .unwrap();
    }
}
