extern crate serde_json;
extern crate tokio;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct EdgeTagList {
    tag_1: String,
    tag_2: String,
    tag_3: String
}

#[derive(Serialize, Deserialize)]
struct Edge {
    is_open: bool,
    tag: EdgeTagList
}

#[derive(Serialize, Deserialize)]
struct EdgeList {
    edge_a: Edge,
    edge_b: Edge,
    edge_c: Edge,
    edge_d: Edge,
    edge_e: Edge,
    edge_f: Edge
}

#[derive(Serialize, Deserialize)]
struct Tile {
    tile_center: String,
    tile_edges: EdgeList
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let raw_map_path = "tile.json";
    let map_r: String = std::fs::read_to_string(raw_map_path).unwrap();
    let map_r: &str = &map_r;

    let struct_tile: Tile = serde_json::from_str(map_r)?;

    println!("{:#?}",struct_tile.tile_edges.edge_a.tag.tag_1);

    Ok(())
}
