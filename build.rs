use cmd_lib::run_cmd;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let dest_path = Path::new(&out_dir).join("stl.rs");

    let scads = [
        "city", "fort", "temple", "fairy", "tower", "tree", "castle", "ruins", "pagoda", "village",
        "mushroom",
    ];

    let mut stl = File::create(dest_path).unwrap();

    for s in scads {
        let stl_tmp = out_dir.join(format!("{s}.stl"));
        let scad = PathBuf::from(format!("scad/{s}.scad"));
        println!("cargo:rerun-if-changed={scad}", scad = scad.display());
        run_cmd!(openscad --export-format asciistl ${scad} -o ${stl_tmp}).unwrap();
        let s_upper = s.to_ascii_uppercase();
        writeln!(
            stl,
            "pub static {s_upper} : Lazy<DefaultMesh> = Lazy::new(|| Stl::from_bytes(\"{stl_tmp}\", include_bytes!(\"{stl_tmp}\")).into_inner());",
                stl_tmp=stl_tmp.display()
        ).unwrap();
    }
}
