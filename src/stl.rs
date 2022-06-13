use once_cell::sync::Lazy;
use rust_3d::{io::StlFormat, Mesh3D, Point3D, PointCloud3D};
use std::path::Path;
type DefaultMesh = Mesh3D<Point3D, PointCloud3D<Point3D>, Vec<usize>>;

include!(concat!(env!("OUT_DIR"), "/stl.rs"));

pub struct Stl {
    mesh: DefaultMesh,
}

impl Stl {
    fn from_bytes<P>(name: P, bytes: &[u8]) -> Stl
    where
        P: AsRef<Path>,
    {
        let mut normals = vec![];
        let mut mesh = DefaultMesh::default();
        let mut cursor = std::io::Cursor::new(bytes);
        rust_3d::io::load_stl_mesh_duped(&mut cursor, StlFormat::Ascii, &mut mesh, &mut normals)
            .unwrap_or_else(|e| panic!("valid embedded stl {} : {:?}", name.as_ref().display(), e));
        Stl { mesh }
    }
    fn into_inner(self) -> DefaultMesh {
        self.mesh
    }
}
