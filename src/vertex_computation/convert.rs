use vulkano;
use cgmath::{Vector3, InnerSpace};
use md5::md5mesh::{Mesh};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: (f32, f32, f32)
}

impl_vertex!(Vertex, position);

pub fn posvec3_to_posvulkano( v_p: &Vec<Vector3<f32>> ) -> Vec<Vertex> {
    let mut res : Vec<Vertex> = Vec::new();
    //res.push(Vertex { position: (0., 0., 0.) });

    for e in v_p {
        res.push(Vertex { position: (e.x, e.y, e.z) } );
    }
    res
}


#[derive(Copy, Clone, Debug)]
pub struct Normal {
    normal: (f32, f32, f32)
}

impl_vertex!(Normal, normal);

pub fn normvec3_to_normvulkano (v_n: &Vec<Vector3<f32>> ) -> Vec<Normal> {
    let mut res : Vec<Normal> = Vec::new();
    //res.push(Normal { normal: (0., 0., 0.) });

    for e in v_n {
        res.push(Normal { normal: (e.x, e.y, e.z) } );
    }
    res
}

pub fn generate_indices(m: &Mesh) -> Vec<u16> {
    let mut res: Vec<u16> = Vec::new();

    for t in &m.triangles {
        let (a, b, c) = t.vertex_indices;

        res.push(a as u16);
        res.push(b as u16);
        res.push(c as u16);
    }

    res
}