use crate::data::{Elevation, Grid, OverrideTile, Resources, Tile, TileType};
use roxmltree;
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

pub fn read_tiles(path: PathBuf, overrides: Option<PathBuf>) -> std::io::Result<Grid> {
    let s = std::fs::read_to_string(&path)?;
    let doc = roxmltree::Document::parse(&s).unwrap();
    let overrides = if let Some(overrides) = overrides {
        let overrides: HashMap<String, OverrideTile> =
            serde_json::from_slice(&std::fs::read(overrides).unwrap()).unwrap();
        Some(overrides)
    } else {
        None
    };

    let terrain_map = if let Some(map) = doc
        .descendants()
        .find(|n| n.tag_name().name() == "terrainmap")
    {
        let entries = map.text().unwrap().split("\t").collect::<Vec<_>>();
        entries
            .chunks(2)
            .map(|win| {
                let nospace = win[0].replace(" ", "");
                let tt = TileType::from_str(&nospace).unwrap();
                (win[1].parse::<usize>().unwrap(), tt)
            })
            .collect::<HashMap<_, _>>()
    } else {
        HashMap::new()
    };

    let mut tiles = doc
        .descendants()
        .filter(|n| n.tag_name().name() == "tilerow")
        .map(|row| {
            use std::io::BufRead;
            let buf = std::io::BufReader::new(row.text().unwrap().as_bytes());
            buf.lines()
                .map(|line| line.unwrap())
                .filter(|line| line != "")
                .map(|line| {
                    let mut iter = line.split("\t");
                    Tile {
                        terrain_type: *terrain_map
                            .get(&iter.next().unwrap().parse().unwrap())
                            .unwrap(),
                        elevation: {
                            let el = iter.next().unwrap().parse().unwrap();
                            if el < 0 && el > -10 {
                                Elevation::Water(el)
                            } else {
                                Elevation::Land(el)
                            }
                        },
                        something: iter.next().unwrap().parse().unwrap(),
                        icy: iter.next().unwrap() == "1",
                        gm_only: iter.next().unwrap() == "1",
                        something_else: iter.next().unwrap().parse().unwrap(),
                        resources: {
                            let next = iter.next().unwrap();
                            if next == "Z" {
                                None
                            } else {
                                Some(Resources {
                                    animals: next.parse().unwrap(),
                                    brick: iter.next().unwrap().parse().unwrap(),
                                    crops: iter.next().unwrap().parse().unwrap(),
                                    gems: iter.next().unwrap().parse().unwrap(),
                                    lumber: iter.next().unwrap().parse().unwrap(),
                                    metals: iter.next().unwrap().parse().unwrap(),
                                    //rock: iter.next().unwrap().parse().unwrap(),
                                })
                            }
                        },
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if let Some(overrides) = overrides {
        for (_, o) in overrides {
            let tile = &mut tiles[o.x as usize][o.y as usize];
            if let Some(typ) = o.typ {
                tile.terrain_type = typ;
            }

            if let Some(elevation) = o.elevation {
                tile.elevation = Elevation::Land(elevation);
            }
        }
    }
    let grid_width = tiles.len();
    let grid_height = tiles[0].len();

    let grid = Grid {
        radius: 1,
        height: grid_height,
        width: grid_width,
        tiles: tiles.into_iter().flatten().collect(),
    };

    Ok(grid)
}
