use rust_3d::Point2D;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;
use strum::EnumString;

#[derive(Debug, Deserialize)]
pub struct OverrideTile {
    pub x: i32,
    pub y: i32,
    #[serde(rename = "type")]
    pub typ: Option<TileType>,
    pub elevation: Option<i32>,
}

struct TileTypeVisitor;

impl<'de> Visitor<'de> for TileTypeVisitor {
    type Value = TileType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid terrain type string")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(de::Error::custom)
    }

    fn visit_string<E>(self, value: String) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for TileType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<TileType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TileTypeVisitor)
    }
}

#[derive(Debug)]
pub struct Resources {
    pub animals: u32,
    pub brick: u32,
    pub crops: u32,
    pub gems: u32,
    pub lumber: u32,
    pub metals: u32,
    //    rock: u32,
}

#[derive(Debug, EnumString, Copy, Clone)]
pub enum TileType {
    WaterSea,
    WaterSeaDeep,
    WaterOcean,
    WaterOceanDeep,
    WaterShoals,
    MountainsSnowcapped,
    Mountain,
    Hills,
    UnderdarkForestFungalHeavy,
    FlatForestMixedHeavy,
    FlatForestJungle,
    HillsGrassy,
    FlatDeadForestWetlands,
    OtherBrokenLands,
    HillsShrubland,
    HillsGrassland,
    FlatGrazingLand,
    FlatForestJungleHeavy,
    FlatDesertDunes,
    FlatShrubland,
    MountainSnowcapped,
    FlatSwamp,
    FlatMoss,
    FlatMarsh,
    HillsForestMixed,
    FlatGrassland,
    FlatForestDeciduous,
    FlatWetlandsEvergreen,
    FlatDesertCactusHeavy,
    Mountains,
    FlatSteppe,
    MountainVolcano,
    FlatForestWetlands,
    MountainsForestDeciduous,
    FlatSavanna,
    //Custom
    City,
    Pagoda,
    SeaCity,
    SmallCity,
    Temple,
    FairyTown,
    Tower,
    Lair,
    Ruins,
    FairyCity,
    Fortress,
    Castle,
    Village,
    Mushroom,
}

impl TileType {
    pub fn tree_density(&self) -> f64 {
        use TileType as TT;
        match self {
            TT::Hills => 0.1,
            TT::UnderdarkForestFungalHeavy => 0.6,
            TT::FlatForestMixedHeavy => 0.8,
            TT::FlatForestJungle => 0.8,
            TT::HillsGrassy => 0.1,
            TT::FlatDeadForestWetlands => 0.3,
            TT::FlatWetlandsEvergreen => 0.5,
            TT::FlatMarsh => 0.3,
            TT::FlatDesertCactusHeavy => 0.0,
            TT::FlatForestJungleHeavy => 0.8,
            TT::HillsForestMixed => 0.5,
            TT::FlatForestDeciduous => 0.8,
            TT::FlatForestWetlands => 0.8,
            TT::MountainsForestDeciduous => 0.5,
            _ => 0.0,
        }
    }
    pub fn height_variance(&self) -> f64 {
        use TileType as TT;
        match self {
            TT::WaterSea => 0.05,
            TT::WaterSeaDeep => 0.1,
            TT::WaterOcean => 0.05,
            TT::WaterOceanDeep => 0.1,
            TT::WaterShoals => 0.0,
            TT::MountainsSnowcapped => 0.5,
            TT::Mountain => 0.5,
            TT::Hills => 0.1,
            TT::UnderdarkForestFungalHeavy => 0.05,
            TT::FlatForestMixedHeavy => 0.05,
            TT::FlatForestJungle => 0.05,
            TT::HillsGrassy => 0.05,
            TT::FlatDeadForestWetlands => 0.0,
            TT::FlatWetlandsEvergreen => 0.0,
            TT::FlatMarsh => 0.0,
            TT::FlatDesertCactusHeavy => 0.0,
            TT::OtherBrokenLands => 0.1,
            TT::HillsShrubland => 0.2,
            TT::HillsGrassland => 0.2,
            TT::FlatGrazingLand => 0.0,
            TT::FlatForestJungleHeavy => 0.0,
            TT::FlatDesertDunes => 0.01,
            TT::FlatShrubland => 0.0,
            TT::MountainSnowcapped => 0.5,
            TT::FlatSwamp => 0.0,
            TT::FlatMoss => 0.0,
            TT::HillsForestMixed => 0.2,
            TT::FlatGrassland => 0.1,
            TT::FlatForestDeciduous => 0.0,
            TT::Mountains => 0.5,
            TT::FlatSteppe => 0.0,
            TT::MountainVolcano => 0.5,
            TT::FlatForestWetlands => 0.01,
            TT::MountainsForestDeciduous => 0.5,
            TT::FlatSavanna => 0.01,
            TT::City
            | TT::Pagoda
            | TT::SeaCity
            | TT::SmallCity
            | TT::Temple
            | TT::Castle
            | TT::Mushroom
            | TT::Village
            | TT::FairyTown
            | TT::Tower
            | TT::Lair
            | TT::Ruins
            | TT::FairyCity
            | TT::Fortress => 0.0,
        }
    }
    // fn color(&self) -> image::Rgb<u8> {
    //     use TileType as TT;
    //     let rgb = match self {
    //         TT::WaterSea => [140, 178, 216],
    //         TT::WaterSeaDeep => [102, 153, 204],
    //         TT::WaterOcean => [0, 102, 153],
    //         TT::WaterOceanDeep => [0, 51, 102],
    //         TT::WaterShoals => [153, 204, 255],
    //         TT::MountainsSnowcapped => [198, 142, 0],
    //         TT::Mountain => [198, 142, 0],
    //         TT::Hills => [232, 206, 89],
    //         TT::UnderdarkForestFungalHeavy => [132, 198, 130],
    //         TT::FlatForestMixedHeavy => [79, 137, 58],
    //         TT::FlatForestJungle => [84, 158, 102],
    //         TT::HillsGrassy => [216, 209, 99],
    //         TT::FlatDeadForestWetlands => [127, 127, 127],
    //         TT::FlatWetlandsEvergreen => [71, 137, 63],
    //         TT::FlatMarsh => [132, 206, 147],
    //         TT::FlatDesertCactusHeavy => [183, 183, 0],
    //         TT::OtherBrokenLands => [205, 155, 0],
    //         TT::HillsShrubland => [216, 219, 112],
    //         TT::HillsGrassland => [216, 242, 150],
    //         TT::FlatGrazingLand => [204, 242, 150],
    //         TT::FlatForestJungleHeavy => [76, 142, 89],
    //         TT::FlatDesertDunes => [255, 255, 153], // beaches
    //         TT::FlatShrubland => [216, 219, 127],
    //         TT::MountainSnowcapped => [198, 142, 0],
    //         TT::FlatSwamp => [145, 203, 148],
    //         TT::FlatMoss => [156, 186, 111],
    //         TT::HillsForestMixed => [114, 163, 58],
    //         TT::FlatGrassland => [229, 242, 155],
    //         TT::FlatForestDeciduous => [147, 198, 99],
    //         TT::Mountains => [178, 127, 0],
    //         TT::FlatSteppe => [186, 201, 127],
    //         TT::MountainVolcano => [216, 142, 0],
    //         TT::FlatForestWetlands => [135, 145, 84],
    //         TT::MountainsForestDeciduous => [135, 163, 51],
    //         TT::FlatSavanna => [198, 234, 141],
    //         TT::City
    //         | TT::Pagoda
    //         | TT::SeaCity
    //         | TT::SmallCity
    //         | TT::Temple
    //         | TT::Castle
    //         | TT::Mushroom
    //         | TT::Village
    //         | TT::FairyTown
    //         | TT::FairyCity
    //         | TT::Tower
    //         | TT::Lair
    //         | TT::Ruins
    //         | TT::Fortress => [0, 0, 0],
    //     };
    //     image::Rgb(rgb)
    // }
}

#[derive(Debug)]
pub enum Elevation {
    Water(i32),
    Land(i32),
}

impl Elevation {
    pub fn to_z(&self) -> f64 {
        match self {
            Elevation::Water(_w) => 0.0,
            Elevation::Land(l) => {
                let min_land_elevation = 1000.0;
                *l as f64 + min_land_elevation
            }
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    pub terrain_type: TileType,
    pub elevation: Elevation,
    pub something: f32,
    pub icy: bool,
    pub gm_only: bool,
    pub something_else: i32,
    pub resources: Option<Resources>,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for GridPos {
    fn from(f: (i32, i32)) -> Self {
        Self { x: f.0, y: f.1 }
    }
}

pub struct Grid {
    pub radius: i32,
    pub height: usize,
    pub width: usize,
    pub tiles: Vec<Tile>,
}

const ADJ: [(i32, i32); 6] = [(-1, -1), (-1, 0), (0, 1), (1, 1), (1, 0), (0, -1)];

impl Grid {
    pub fn normalize(&self, x: i32, y: i32) -> (usize, usize) {
        let x = (x.rem_euclid(self.width as i32)) as usize;
        let y = (y.rem_euclid(self.height as i32)) as usize;
        (x, y)
    }
    pub fn grid_pos(&self, x: i32, y: i32) -> &Tile {
        let (x, y) = self.normalize(x, y);
        &self.tiles[x * self.height as usize + y]
    }
    pub fn center(&self, x: i32, y: i32) -> (f64, f64) {
        let pt = self.center_pt(x, y);
        (pt.x, pt.y)
    }
    pub fn hex_radius_min(&self) -> f64 {
        60.0f64.to_radians().sin() * self.radius as f64
    }

    pub fn center_pt(&self, x: i32, y: i32) -> Point2D {
        //let (x,y) = self.normalize(x, y);
        let hr = self.hex_radius_min();
        Point2D::new(
            (x + 1) as f64 * (self.radius as f64 * 1.5),
            ((y + 1) as f64 * hr * 2.0) + (x.rem_euclid(2) as f64 * hr),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = ((i32, i32), &Tile)> {
        let xs = (0..self.width)
            .flat_map(move |v| std::iter::repeat(v).take(self.height))
            .map(|x| x as i32);
        let ys = (0..self.height).cycle().map(|y| y as i32);

        xs.zip(ys).zip(&self.tiles)
    }

    pub fn adj(&self, x: i32, y: i32) -> Vec<((i32, i32), &Tile)> {
        let mut v: Vec<_> = ADJ
            .iter()
            .map(|(adj_x, adj_y)| {
                (
                    adj_x,
                    match (x.rem_euclid(2), adj_x) {
                        (0, 1) => adj_y - 1,
                        (1, -1) => adj_y + 1,
                        _ => adj_y + 0,
                    },
                )
            })
            .map(|(adj_x, adj_y)| (x + adj_x, y + adj_y))
            .map(|(x, y)| ((x, y), self.grid_pos(x, y)))
            .collect();

        v.sort_by_key(|(p, _t)| {
            let center = self.center_pt(x, y);
            let pt = self.center_pt(p.0, p.1) - center;
            ((pt.y.atan2(pt.x).to_degrees() + 360.0) % 360.0) as u32
        });
        v
    }
}
