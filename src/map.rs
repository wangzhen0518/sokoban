use specs::World;

use crate::{
    components::{BoxColor, Position},
    entities,
};

pub fn load_map(world: &mut World, map_string: &str) {
    let rows: Vec<&str> = map_string.trim().lines().map(|x| x.trim()).collect();
    for (y, &row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        for (x, &column) in columns.iter().enumerate() {
            let pos = Position::new(x as u8, y as u8, 0);
            match column {
                "." => entities::create_floor(world, pos),
                "W" => {
                    entities::create_floor(world, pos);
                    entities::create_wall(world, pos);
                }
                "P" => {
                    entities::create_floor(world, pos);
                    entities::create_player(world, pos);
                }
                "BB" => {
                    entities::create_floor(world, pos);
                    entities::create_box(world, pos, BoxColor::Blue);
                }
                "RB" => {
                    entities::create_floor(world, pos);
                    entities::create_box(world, pos, BoxColor::Red);
                }
                "BS" => {
                    entities::create_floor(world, pos);
                    entities::create_box_spot(world, pos, BoxColor::Blue);
                }
                "RS" => {
                    entities::create_floor(world, pos);
                    entities::create_box_spot(world, pos, BoxColor::Red);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
