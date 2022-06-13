use crate::data::{Elevation, Grid, Tile, TileType};
use crate::mesh;
use noise::{NoiseFn, Seedable};
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;
use rust_3d::*;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::PathBuf;

type DefaultMesh = Mesh3D<Point3D, PointCloud3D<Point3D>, Vec<usize>>;

fn hex(
    noise: &noise::Perlin,
    grid: &Grid,
    (x, y): (i32, i32),
    tile: &Tile,
    tiles: &[((i32, i32), &Tile)],
    slice: (Range<i32>, Range<i32>),
) -> DefaultMesh {
    let c_pt = grid.center_pt(x, y);
    #[derive(Hash, Eq, PartialEq)]
    enum HexFace {
        North = 0,
        NorthWest = 60,
        SouthWest = 120,
        South = 180,
        SouthEast = 240,
        NorthEast = 300,
    }
    impl HexFace {
        fn from_angle(angle: u32) -> HexFace {
            if angle == HexFace::North as u32 {
                HexFace::North
            } else if angle == HexFace::NorthWest as u32 {
                HexFace::NorthWest
            } else if angle == HexFace::NorthEast as u32 {
                HexFace::NorthEast
            } else if angle == HexFace::South as u32 {
                HexFace::South
            } else if angle == HexFace::SouthWest as u32 {
                HexFace::SouthWest
            } else {
                HexFace::SouthEast
            }
        }
    }
    fn hex_cyl(keep_faces: HashSet<HexFace>) -> DefaultMesh {
        let mut mesh = DefaultMesh::default();
        let angles: Vec<_> = (0..360).step_by(360 / 6).collect();

        let edges: Vec<_> = [0.0, 1.0]
            .into_iter()
            .map(|z| {
                let center = Point3D::new(0.0, 0.0, z);
                let center = mesh.add_vertex(center);
                let points: Vec<_> = angles
                    .iter()
                    .map(|angle| (angle as f64).to_radians())
                    .map(|angle| {
                        let new_x = angle.sin();
                        let new_y = angle.cos();
                        mesh.add_vertex(Point3D::new(new_x, new_y, z))
                    })
                    .collect();
                points.repeat(2).windows(2).take(6).for_each(|win| {
                    if z == 1.0 {
                        mesh.try_add_connection(win[1], win[0], center).unwrap();
                    } else {
                        mesh.try_add_connection(win[0], win[1], center).unwrap();
                    }
                });

                points
            })
            .collect();

        for (angle, (bottom, top)) in angles.iter().zip(
            edges[0]
                .repeat(2)
                .windows(2)
                .take(6)
                .zip(edges[1].repeat(2).windows(2)),
        ) {
            if keep_faces.contains(&HexFace::from_angle(angle as u32)) {
                mesh.try_add_connection(bottom[1], bottom[0], top[0])
                    .unwrap();
                mesh.try_add_connection(bottom[1], top[0], top[1]).unwrap();
            }
        }
        //Rotate so that north face is up.
        let m = Matrix4::rotation_axis(
            &Norm3D::norm_z(),
            Rad {
                val: (30.0f64 + 60.0 * -3.0).to_radians(),
            },
        );
        mesh.transform(&m);

        mesh
    }

    let mut faces = HashSet::default();
    if y == slice.1.start {
        if x % 2 == 0 {
            faces.insert(HexFace::NorthWest);
            faces.insert(HexFace::NorthEast);
        }

        faces.insert(HexFace::North);
    }
    if y == slice.1.end - 1 {
        if x % 2 == 1 {
            faces.insert(HexFace::SouthWest);
            faces.insert(HexFace::SouthEast);
        }
        faces.insert(HexFace::South);
    }
    if x == slice.0.start {
        faces.insert(HexFace::NorthWest);
        faces.insert(HexFace::SouthWest);
    }
    if x == slice.0.end - 1 {
        faces.insert(HexFace::NorthEast);
        faces.insert(HexFace::SouthEast);
    }
    let mut mesh = hex_cyl(faces);

    let long_tiles = tiles.repeat(2);
    let adj: Vec<_> = long_tiles.windows(2).take(6).collect();

    let move_down = -1.0;
    let scale_up = 2.0;

    let top_layer = 1.0 * scale_up + move_down;
    let m = Matrix4::translation(c_pt.x, c_pt.y, move_down) * Matrix4::scale(1.0, 1.0, scale_up);
    mesh.transform(&m);
    fn barycentric(p: Point2D, barys: [Point2D; 3]) -> [f64; 3] {
        let mut idx = [0, 1, 2];
        idx.sort_by(|&a, &b| barys[a].partial_cmp(&barys[b]).unwrap());
        let [a, b, c] = [
            barys[idx[0]].clone(),
            barys[idx[1]].clone(),
            barys[idx[2]].clone(),
        ];

        // https://computergraphics.stackexchange.com/questions/1866/how-to-map-square-texture-to-triangle
        let bary_a = ((b.y - c.y) * (p.x - c.x) + (c.x - b.x) * (p.y - c.y))
            / ((b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y));
        let bary_b = ((c.y - a.y) * (p.x - c.x) + (a.x - c.x) * (p.y - c.y))
            / ((b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y));
        let bary_c = 1.0 - bary_a - bary_b;

        let mut bary_res = [0.0; 3];
        bary_res[idx[0]] = bary_a;
        bary_res[idx[1]] = bary_b;
        bary_res[idx[2]] = bary_c;
        bary_res
    }

    mesh = rust_3d::subdivide::linear(&mesh).unwrap();
    mesh = rust_3d::subdivide::linear(&mesh).unwrap();
    //    mesh = rust_3d::subdivide::linear(&mesh).unwrap();
    //mesh = rust_3d::subdivide::linear(&mesh).unwrap();

    assert_eq!(adj.len(), 6);
    for v in (0..mesh.num_vertices()).map(|val| VId { val }) {
        let mut p = mesh.vertex(v).unwrap();
        let fudge = 0.5;

        //Fudging the exact point helps obscure obvious lines between hex cells.
        let p_mod = Point2D::new(
            p.x + noise.get([p.x, p.y]) * fudge,
            p.y + noise.get([p.y, p.x]) * fudge,
        );

        if p.z == top_layer {
            //println!("{:?}", p);

            let mut bary_tiles = if p_mod.x == c_pt.x && p_mod.y == c_pt.y {
                //Center of tile cannot participate in barycentric coordinates as it is one of the
                // points.
                p.z = tile.elevation.to_z() / 1000.0;
                vec![(1.0 / 3.0, tile), (1.0 / 3.0, tile), (1.0 / 3.0, tile)]
            } else {
                // Find the angle of the current point, to determine which tiles
                // particpate in the elevations of that tile.
                let rel_pt = Point2D::new(p_mod.x - c_pt.x, p_mod.y - c_pt.y);
                let angle = rel_pt.y.atan2(rel_pt.x);
                let adj_index = ((((angle - std::f64::consts::FRAC_PI_6).to_degrees() + 360.0)
                    % 360.0)
                    / 60.0) as usize;

                // Use center points from participating tiles as barycentric triangle region.
                let adj = adj[adj_index];
                let a = grid.center_pt(adj[0].0 .0, adj[0].0 .1);
                let b = grid.center_pt(adj[1].0 .0, adj[1].0 .1);
                let c = c_pt.clone();

                let pts = [a, b, c];
                let tiles = [adj[0].1, adj[1].1, tile];

                let barys = barycentric(p_mod.clone(), pts.clone());

                let mut bary_tiles = barys
                    .into_iter()
                    .zip(tiles.iter().cloned())
                    .collect::<Vec<_>>();

                let b_sum: f64 = bary_tiles
                    .iter_mut()
                    .map(|(b, t)| {
                        if matches!(
                            t.terrain_type,
                            TileType::City
                                | TileType::SeaCity
                                | TileType::Pagoda
                                | TileType::SmallCity
                                | TileType::Temple
                                | TileType::Castle
                                | TileType::Mushroom
                                | TileType::Village
                                | TileType::FairyTown
                                | TileType::FairyCity
                                | TileType::Tower
                                | TileType::Lair
                                | TileType::Ruins
                                | TileType::Fortress
                        ) {
                            *b *= 10.0;
                        }
                        *b
                    })
                    .sum();

                bary_tiles.iter_mut().for_each(|(b, _)| *b /= b_sum);

                bary_tiles
            };

            bary_tiles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            // Compute new default elevation
            let p_uv: f64 = bary_tiles
                .iter()
                .map(|&(b, t)| b * t.elevation.to_z() / 1000.0)
                .sum();

            assert!(!p_uv.is_nan());

            p.z = p_uv;
            // Add some elevation variablity to each region based on the region type.
            p.z += bary_tiles
                .iter()
                .map(|&(b, t)| {
                    b * match t.elevation {
                        Elevation::Water(depth) => {
                            let var = depth as f64 * 0.05;
                            noise.get([p.x * var, p.y * var]) * var
                        }

                        Elevation::Land(_height) => {
                            noise.get([p.x, p.y]) * (t.terrain_type.height_variance() * 3.0)
                        }
                    }
                })
                .sum::<f64>();
            p.z += top_layer;
            mesh.change_vertex(v, p).unwrap();
        }
    }

    mesh
}

const MAX_TREES_PER_TILE: f64 = 50.0;

fn mesh_add<M1, M2>(left: &mut M1, right: M2)
where
    M1: IsFaceEditableMesh<Point3D, Face3> + IsVertexEditableMesh<Point3D, Face3>,
    M2: IsMesh<Point3D, Face3>,
{
    let verts: HashMap<_, _> = (0..right.num_vertices())
        .map(|val| (VId { val }, right.vertex(VId { val }).unwrap()))
        .map(|(id, vert)| {
            let new_id = left.add_vertex(vert);
            (id, new_id)
        })
        .collect();

    (0..right.num_faces())
        .map(|val| right.face_vertex_ids(FId { val }).unwrap())
        .for_each(|face| {
            left.try_add_connection(verts[&face.a], verts[&face.b], verts[&face.c])
                .unwrap();
        });
}

pub fn gen(
    grid: &Grid,
    output: PathBuf,
    seed: u32,
    slice: (Option<Range<i32>>, Option<Range<i32>>),
) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let noise = noise::Perlin::new().set_seed(seed);

    let r = grid.radius as f64;

    let x_range = if let Some(s) = slice.0 {
        s
    } else {
        0..grid.width as i32
    };

    let y_range = if let Some(s) = slice.1 {
        s
    } else {
        0..grid.height as i32
    };
    let slice = (x_range, y_range);

    let mut rngs: Vec<_> = grid
        .iter()
        .map(|_| rand::rngs::StdRng::seed_from_u64(rng.gen()))
        .collect();
    let meshes: Vec<_> = grid
        .iter()
        .filter(|((x_idx, y_idx), _tile)| slice.0.contains(x_idx) && slice.1.contains(y_idx))
        .zip(rngs.iter_mut())
        .par_bridge()
        .map(|(((x_idx, y_idx), tile), rng)| {
            let adjs = grid.adj(x_idx, y_idx);
            let mesh = hex(&noise, &grid, (x_idx, y_idx), tile, &adjs, slice.clone());
            ((mesh, ((x_idx, y_idx), tile)), rng)
        })
        .map(|((mut mesh, ((x_idx, y_idx), tile)), rng)| {
            let (x, y) = grid.center(x_idx, y_idx);

            let (scale, foliage) = match tile.terrain_type {
                TileType::UnderdarkForestFungalHeavy => {
                    (Point3D::new(0.2, 0.2, 0.5), &*mesh::MUSHROOM)
                }
                _ => (Point3D::new(1.0, 1.0, 1.0), &*mesh::TREE),
            };
            (0..(tile.terrain_type.tree_density() * MAX_TREES_PER_TILE) as usize)
                .map(|_| {
                    (
                        Point3D::new(rng.gen_range((-r)..r), rng.gen_range((-r)..r), 0.0),
                        rng.gen_range(0.0..360.0f64).to_radians(),
                    )
                })
                .for_each(|(off, angle)| {
                    let center = Point3D::new(x as f64, y as f64, 100000.0);
                    let ray = Ray3D::new(Line3D::new(center + off, Norm3D::norm_z_neg()));
                    let mut v = vec![];
                    collect_intersections_ray_mesh(&ray, &mut mesh, &mut v);
                    if let Some(p) = v.iter().filter(|p| p.z > 0.0).next() {
                        let m = Matrix4::translation(p.x, p.y, p.z)
                            * Matrix4::scale(scale.x, scale.y, scale.z)
                            * Matrix4::rotation_axis(&Norm3D::norm_z(), Rad { val: angle })
                            * Matrix4::identity();
                        let tree = foliage.transformed(&m);
                        mesh_add(&mut mesh, tree);
                    }
                });
            ((mesh, ((x_idx, y_idx), tile)), rng)
        })
        .map(|((mut mesh, ((x_idx, y_idx), tile)), rng)| {
            use TileType as TT;
            let center_pt = grid.center_pt(x_idx, y_idx);
            let mut scale = 0.25;
            let new_mesh = match tile.terrain_type {
                TT::Lair | TT::FairyTown | TT::FairyCity => Some(&*mesh::FAIRY),
                TT::Pagoda => Some(&*mesh::PAGODA),
                TT::Ruins => Some(&*mesh::RUINS),
                TT::Tower => Some(&*mesh::TOWER),
                TT::Temple => Some(&*mesh::TEMPLE),
                TT::Castle => Some(&*mesh::CASTLE),
                TT::Mushroom => Some(&*mesh::MUSHROOM),
                TT::Village => Some(&*mesh::VILLAGE),
                TT::Fortress => Some(&*mesh::FORT),
                TT::SeaCity | TT::City => Some(&*mesh::CITY),
                TT::SmallCity => {
                    scale *= 0.5;
                    Some(&*mesh::CITY)
                }
                _ => None,
            };

            if let Some(new_mesh) = new_mesh {
                let ray = Ray3D::new(Line3D::new(
                    Point3D::new(center_pt.x + 0.1, center_pt.y + 0.1, 10000.0),
                    Norm3D::norm_z_neg(),
                ));
                let mut v = vec![];
                collect_intersections_ray_mesh(&ray, &mut mesh, &mut v);
                let angle = rng.gen_range(0.0..360.0f64);
                if let Some(p) = v.iter().filter(|p| p.z > 0.0).next() {
                    let m = Matrix4::translation(p.x, p.y, p.z)
                        * Matrix4::rotation_axis(
                            &Norm3D::norm_z(),
                            Rad {
                                val: angle.to_radians(),
                            },
                        )
                        * Matrix4::scale(scale, scale, scale);
                    let new_mesh = new_mesh.transformed(&m);
                    mesh_add(&mut mesh, new_mesh);
                }
            }
            ((mesh, ((x_idx, y_idx), tile)), rng)
        })
        .map(|((mut mesh, ((x_idx, y_idx), tile)), rng)| {
            //Invert y coordinate system
            let m = Matrix4::scale(1.0, -1.0, 1.0);
            mesh.transform(&m);
            let mut new_mesh = DefaultMesh::default();
            let translate = (0..mesh.num_vertices())
                .map(|val| VId { val })
                .map(|val| {
                    let pt = mesh.vertex(val).unwrap();
                    (val, new_mesh.add_vertex(pt))
                })
                .collect::<HashMap<_, _>>();

            // Invert faces
            for i in (0..mesh.num_faces()).map(|val| FId { val }) {
                let vert = mesh.face_vertex_ids(i).unwrap();
                new_mesh
                    .try_add_connection(translate[&vert.b], translate[&vert.a], translate[&vert.c])
                    .unwrap();
            }

            ((new_mesh, ((x_idx, y_idx), tile)), rng)
        })
        .map(|((mesh, ((_x_idx, _y_idx), _tile)), _rng)| mesh)
        .collect();

    let meshes = if false {
        let mut mesh = meshes
            .into_iter()
            .par_bridge()
            .fold(
                || DefaultMesh::default(),
                |mut out_mesh, mesh| {
                    mesh_add(&mut out_mesh, mesh);
                    out_mesh
                },
            )
            .reduce(
                || Mesh3D::default(),
                |mut m1, m2| {
                    mesh_add(&mut m1, m2);
                    m1
                },
            );
        let (x, y) = grid.center(grid.width as i32, grid.height as i32);
        let (x, y) = (x - grid.radius as f64, y - grid.radius as f64);
        let x_radius = x as f64 / (2.0 * std::f64::consts::PI);
        let x_circumference = x as f64;

        let y_radius = y as f64 / (2.0 * std::f64::consts::PI);
        let y_circumference = y as f64;

        let m =
        // move map to be purely in y coordinate space
        Matrix4::translation(0.0, -y as f64/2.0, 0.0)
        //flip map x coordinates
            * Matrix4::scale(-1.0, 1.0, 1.0)
            * Matrix4::identity();

        mesh.transform(&m);

        //wrap into tube
        for i in (0..mesh.num_vertices()).map(|val| VId { val }) {
            let mut v = mesh.vertex(i).unwrap();
            let theta = 180.0 * (v.y / (y_circumference / 2.0));
            //println!("{}", theta);
            let theta = Rad {
                val: theta.to_radians(),
            };
            let m = Matrix4::rotation_axis(
                &Norm3D::norm_x(),
                Rad { val: 0.0 }, //desired orientation
            ) * Matrix4::translation(-x as f64 / 2.0, 0.0, 0.0)
                * Matrix4::rotation_axis(&Norm3D::norm_x(), theta)
                * Matrix4::scale(1.0, 0.0, 1.0)
                * Matrix4::translation(0.0, 0.0, y_radius)
                * Matrix4::identity();

            v.transform(&m);
            //println!("{:?} {:?}", v, new);
            mesh.change_vertex(i, v).unwrap();
        }

        // wrap into donut
        for i in (0..mesh.num_vertices()).map(|val| VId { val }) {
            let mut v = mesh.vertex(i).unwrap();
            let theta = 180.0 * v.x / (x_circumference / 2.0);
            //println!("{}", theta);
            let theta = Rad {
                val: theta.to_radians(),
            };
            let m = Matrix4::rotation_axis(&Norm3D::norm_z(), theta)
                * Matrix4::scale(0.0, 1.0, 1.0)
                * Matrix4::translation(0.0, x_radius + y_radius, 0.0)
                * Matrix4::identity();
            v.transform(&m);
            mesh.change_vertex(i, v).unwrap();
        }
        vec![mesh]
    } else {
        meshes
    };

    println!("Writing mesh to {}...", output.display());
    let mut stl = std::fs::File::create(&output).unwrap();
    for m in meshes {
        rust_3d::io::save_stl_ascii(&mut stl, &m).unwrap();
        //rust_3d::io::save_ply_binary(&mut ply, &m, &rust_3d::Precision::P64).unwrap();
    }
}
