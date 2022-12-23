use std::fs;

#[derive(Builder)]
pub struct Map {
    #[builder(default = "32")]
    pub tile_size: u32,
    #[builder(default = "1.0")]
    pub tile_scale: f32,
    pub tiles_per_row: u32,
    #[builder(default = "10")]
    pub tiles_per_file_row: u32,
    pub tiles: Vec<u32>,
}

pub fn load_map(map_file: &str) -> Map {
    let data = fs::read_to_string(map_file).unwrap(); 
    let rows = data.lines()
        .collect::<Vec<&str>>();

    let columns_count = rows.first().unwrap().split(',').count() as u32;

    let tiles = rows
        .iter()
        .flat_map(|line| line.split(',').map(|s| s.parse::<u32>().unwrap())).collect();

    MapBuilder::default()
        .tiles_per_row(columns_count)
        .tiles(tiles)
        .tile_scale(2.0)
        .build()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::load_map;

    #[test]
    fn parse_map() {
        let tiles = load_map("./assets/tilemaps/jungle.map");

    }
}
