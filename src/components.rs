use std::fmt::Display;

use specs::{Component, NullStorage, VecStorage, World, WorldExt};

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

#[derive(Component, Default, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Position {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }
}

impl From<&Position> for (u8, u8) {
    fn from(value: &Position) -> Self {
        (value.x, value.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RenderableKind {
    Static,
    Animated,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    paths: Vec<String>,
    kind: RenderableKind,
}

impl Renderable {
    pub fn new_static(path: String) -> Renderable {
        Renderable {
            paths: vec![path],
            kind: RenderableKind::Static,
        }
    }

    pub fn new_animated(paths: Vec<String>) -> Renderable {
        Renderable {
            paths,
            kind: RenderableKind::Animated,
        }
    }

    pub fn path(&self, mut path_index: usize) -> String {
        // path_index %= 2 * self.paths.len() * 2 - 1;
        // if path_index > self.paths.len() - 1 {
        //     path_index = 2 * self.paths.len() - path_index - 2;
        // }

        path_index %= self.paths.len();
        self.paths[path_index].clone()
    }

    pub fn kind(&self) -> RenderableKind {
        self.kind
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Debug, PartialEq, Eq)]
pub enum BoxColor {
    Red,
    Blue,
}

impl Display for BoxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BoxColor::Red => "red",
            BoxColor::Blue => "blue",
        })
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub color: BoxColor,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub color: BoxColor,
}
