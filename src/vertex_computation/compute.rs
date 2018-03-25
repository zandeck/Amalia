 use md5::md5mesh::*;
 use cgmath::{Vector3, InnerSpace};
 use vertex_computation::convert::generate_indices;

 pub fn prepare_mesh(m: &Mesh, v_joints: &Vec<Joint>) -> Vec<Vector3<f32>> {
     let mut position_buffer : Vec<Vector3<f32>> = Vec::new();

     for vertice in &m.vertices {
         let mut new_vertice : Vector3<f32> = Vector3::new(0., 0., 0.);

         for i in 0..vertice.weight_count {
             
             let w : &Weight = &m.weights[ (vertice.start_weight + i) as usize];
             let j : &Joint = &v_joints[ w.joint_index as usize] ;

             let rot_position = j.orientation * w.position;

             new_vertice += ( j.position + rot_position ) * w.bias;
         }
         // println!("{:?}", new_vertice);

         position_buffer.push(new_vertice);

     }
    // println!("{:?}", position_buffer);
    position_buffer
 }

 pub fn prepare_normals(m: &Mesh, vertices_position: &Vec<Vector3<f32>>) -> Vec<Vector3<f32>> {
    let mut normal_buffer : Vec<Vector3<f32>> = vec![ Vector3::new(0., 0., 0.); vertices_position.len() ];

    for t in &m.triangles {
        let (i0, i1, i2) = t.vertex_indices;

        let v0 = &vertices_position[ i0 as usize ];
        let v1 = &vertices_position[ i1 as usize ];
        let v2 = &vertices_position[ i2 as usize ];

        //println!("v0 : {:?}", v0);
        //println!("v1 : {:?}", v1);
        //println!("v2 : {:?}", v2);
        let cross_product = (v2 - v0).cross(v1 - v0);
        //println!("Cross_product : {:?}", cross_product);

        normal_buffer[ i0 as usize ] += cross_product;
        normal_buffer[ i1 as usize ] += cross_product;
        normal_buffer[ i2 as usize ] += cross_product;
    }

    // println!("{:?}", normal_buffer);
    for i in 0..normal_buffer.len() {
       normal_buffer[ i ] = normal_buffer[ i ].normalize();
    }

    normal_buffer
 }

 pub fn prepare_full_mesh(ms: &Md5Mesh) -> (Vec<Vector3<f32>>, Vec<Vector3<f32>>, Vec<u16>) {
     let mut res_v : Vec<Vector3<f32>> = Vec::new();
     let mut res_n : Vec<Vector3<f32>> = Vec::new();
     let mut res_i : Vec<u16> = Vec::new();

    for m in &ms.meshes {
        println!("{:?} : {:?}", m.shader, m.vertices.len());
        let mut tmp = prepare_mesh(m, &ms.joints);
        let mut tmp_normals = prepare_normals(m, &tmp);
        let mut tmp_i = generate_indices(&m);

        for i in 0..tmp_i.len() {
            tmp_i[ i ] += res_v.len() as u16;
        }

        res_v.append(&mut tmp);
        res_n.append(&mut tmp_normals);
        res_i.append(&mut tmp_i);
    }
    (res_v, res_n, res_i)
 }