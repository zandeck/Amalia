#[macro_use]
extern crate nom;
extern crate cgmath;

extern crate winit;
extern crate time;

#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;

mod md5;
use md5::md5mesh_parser::parse_md5mesh;
use nom::FileProducer;
use std::fs::File;
use std::io::Read;

mod renderer;
use renderer::render::render_model;

mod vertex_computation;
use vertex_computation::compute::{prepare_mesh, prepare_normals, prepare_full_mesh};
use vertex_computation::convert::posvec3_to_posvulkano;
use vertex_computation::convert::normvec3_to_normvulkano;
use vertex_computation::convert::generate_indices;

fn main() {

    let path = "./Resources/bob_lamp_update/bob_lamp_update_export.md5mesh";
    let mut f = File::open(path).unwrap();
    let mut buff = vec![];

    f.read_to_end(&mut buff).unwrap();
    let (_, res) = parse_md5mesh(&buff).unwrap();

    // let model_idx: usize = 0;

    // let s = prepare_mesh(&res.meshes[model_idx], &res.joints);
    // let n = prepare_normals(&res.meshes[model_idx], &s);
    // let idx = generate_indices(&res.meshes[model_idx]);

    let (s, n, idx) = prepare_full_mesh(&res);

    let vertices = posvec3_to_posvulkano(&s);
    let normales = normvec3_to_normvulkano(&n);

    //println!("{:?}", vertices);
    //println!("{:?}", normales);
    //println!("{:?}", idx);
    render_model(vertices.as_slice(), normales.as_slice(), idx.as_slice());
}