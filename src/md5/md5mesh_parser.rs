#![allow(dead_code)]
use std::str;
use std::str::FromStr;
use nom::{digit, multispace};
use md5::md5mesh::{Md5Mesh, Joint, Vertex, Mesh, Triangle, Weight};
use md5::md5common_parser::*;


named!(pub parse_header<&[u8], (u8, String)>,
    do_parse!(
        version:
            map_res!(
                map_res!(
                    terminated!(
                        preceded!(tag!("MD5Version "), digit),
                        opt!(multispace)
                    ),
                    str::from_utf8
                ),
                FromStr::from_str ) >>

    command_line:
        preceded!(
            tag!("commandline "),
            escaped_string
        ) >>
        (version, command_line)));



named!(pub parse_joints<&[u8], Vec<Joint>>,
    preceded!(
        tag!("joints"),
        delimited!(
            ws!(tag!("{")),
            many0!(
                do_parse!(
                    name: ws!(escaped_string) >>
                    parent_index: ws!(parse_i32) >>
                    position: ws!(parse_vector3f32) >>
                    orientation: ws!(parse_quaternionf32) >>
                    opt!(comments) >>
                    (Joint {
                        name: name,
                        parent_index: parent_index,
                        position: position,
                        orientation: orientation
                    })
                )
            ),
            ws!(tag!("}"))
        )
    )
);

named!(pub parse_vertex<&[u8], Vertex>,
    do_parse!(
        ws!(tag!("vert")) >>
        index: ws!(parse_u32) >>
        tex_coords: ws!(parse_vector2f32) >>
        start_weight: ws!(parse_u32) >>
        weight_count: ws!(parse_u32) >>
        (Vertex {
            index: index,
            tex_coords: tex_coords,
            start_weight: start_weight,
            weight_count: weight_count
        })
    )
);

named!(pub parse_vertices<&[u8], Vec<Vertex>>,
    map!(
        do_parse!(
            ws!(tag!("numverts")) >>
            ws!(parse_u32) >>
            vertices: many0!(parse_vertex) >>
            (vertices)
        ),
        |mut vertices : Vec<Vertex>| {
            vertices.sort_by_key(|v| v.index);
            vertices
        }
    )
);

named!(pub parse_triangle<&[u8], Triangle>,
    do_parse!(
        ws!(tag!("tri")) >>
        index: ws!(parse_u32) >>
        vertex_indices: ws!(parse_tuple3u32) >>
        (Triangle {index: index, vertex_indices: vertex_indices})
    )
);

named!(pub parse_triangles<&[u8], Vec<Triangle>>,
    map!(
        do_parse!(
            ws!(tag!("numtris")) >>
            ws!(parse_u32) >>
            triangles: many0!(parse_triangle) >>
            (triangles)
        ),
        |mut triangles : Vec<Triangle>| {
            triangles.sort_by_key(|t| t.index);
            triangles
        }
    )
);

named!(pub parse_bias<&[u8], f32>,
    ws!(
        map_res!(
            parse_f32,
            |v: f32| {
                if v.abs() <= 1.0 {
                    Ok(v)
                } else {
                    Err("Invalid bias")
                }
            }
        )
    )
);

named!(pub parse_weight<&[u8], Weight>,
    do_parse!(
        ws!(tag!("weight")) >>
        index: ws!(parse_u32) >>
        joint_index: ws!(parse_u32) >>
        bias: ws!(parse_bias) >>
        position: ws!(parse_vector3f32) >>
        (Weight {index: index, joint_index: joint_index, bias: bias, position: position})
    )
);

named!(pub parse_weights<&[u8], Vec<Weight>>,
    map!(
        do_parse!(
            ws!(tag!("numweights")) >>
            ws!(parse_u32) >>
            weights: many0!(parse_weight) >>
            (weights)
        ),
        |mut weights : Vec<Weight>| {
            weights.sort_by_key(|w| w.index);
            weights
        }
    )
);

named!(pub parse_mesh<&[u8], Mesh>,
    preceded!(
        tag!("mesh"),
        delimited!(
            ws!(tag!("{")),
            do_parse!(
                shader:
                    preceded!(
                        tag!("shader"),
                        ws!(escaped_string)
                    ) >>
                verts: ws!(parse_vertices) >>
                tris: ws!(parse_triangles) >>
                weights: ws!(parse_weights) >>
                (Mesh {
                    shader: shader,
                    vertices: verts,
                    triangles: tris,
                    weights: weights
                })
            ),
            ws!(tag!("}"))
        )
    )
);

named!(pub parse_meshes<&[u8], Vec<Mesh>>,
    ws!(many0!(ws!(parse_mesh)))
);

named!(pub parse_md5mesh<&[u8], Md5Mesh>,
    do_parse!(
        header: ws!(parse_header) >>
        ws!(tag!("numJoints")) >>
        ws!(parse_u32) >>
        ws!(tag!("numMeshes")) >>
        ws!(parse_u32) >>
        joints: ws!(parse_joints) >>
        meshes: ws!(parse_meshes) >>
        (Md5Mesh {
            version: header.0,
            command_line: header.1,
            joints: joints,
            meshes: meshes
        })
    )
);

#[cfg(test)]
mod tests {
    extern crate cgmath;

    use nom::IResult::Done;
    use std::str;
    use cgmath::{Vector3, Vector2, Quaternion};
    use md5::md5mesh::{Md5Mesh, Joint, Vertex, Mesh, Triangle, Weight};

    #[test]
    fn parse_header() {
        let string =
            b"MD5Version 10
            commandline \"Exported from Blender by io_export_md5.py by Paul Zirkle\"";

        let header = (10, String::from("Exported from Blender by io_export_md5.py by Paul Zirkle"));
        assert_eq!(super::parse_header(string), Done(&b""[..], header));
    }

    #[test]
    fn parse_joints() {
        let string =
            b"joints {
            	\"origin\"	-1 ( -0.000000 0.001643 -0.000604 ) ( -0.707107 -0.000242 -0.707107 )		// comment
            	\"sheath\"	0 ( 1.100481 -0.317714 3.170247 ) ( 0.307041 -0.578615 0.354181 )		// comment
              }";

          let joint1 =
              Joint {
                  name: String::from("origin"),
                  parent_index: -1,
                  position: Vector3::new(-0.000000, 0.001643, -0.000604),
                  orientation: Quaternion::new(0.0, -0.707107, -0.000242, -0.707107)
              };

        let joint2 =
            Joint {
                name: String::from("sheath"),
                parent_index: 0,
                position: Vector3::new(1.100481, -0.317714, 3.170247),
                orientation: Quaternion::new(0.4454863, 0.307041, -0.578615, 0.354181)
            };

        let joints = vec![joint1, joint2];

        assert_eq!(super::parse_joints(string), Done(&b""[..], joints));
    }

    #[test]
    fn parse_vertex() {
        let string = b"vert 0 ( 0.683594 0.455078 ) 0 3";

        let vertex =
            Vertex {
                index: 0,
                tex_coords: Vector2::new(0.683594, 0.455078),
                start_weight: 0,
                weight_count: 3
            };

        assert_eq!(super::parse_vertex(string), Done(&b""[..], vertex));
    }

    #[test]
    fn parse_mesh() {
        let string =
            b"mesh {
                shader \"bob_body\"

                numverts 1
                vert 0 ( 0.683594 0.455078 ) 0 3

                numtris 628
            	tri 0 0 2 1

                numweights 859
                weight 0 16 0.333333 ( -0.194917 0.111128 -0.362937 )
            }";

        let vertex =
            Vertex {
                index: 0,
                tex_coords: Vector2::new(0.683594, 0.455078),
                start_weight: 0,
                weight_count: 3
            };

        let triangle =
            Triangle {
                index: 0,
                vertex_indices: (0, 2, 1)
            };

        let weight =
            Weight {
                index: 0,
                joint_index: 16,
                bias: 0.333333,
                position: Vector3::new(-0.194917, 0.111128, -0.362937)
            };

        let mesh =
            Mesh {
                shader: String::from("bob_body"),
                vertices: vec![vertex],
                triangles: vec![triangle],
                weights: vec![weight]
            };

        assert_eq!(super::parse_mesh(string), Done(&b""[..], mesh));
    }

    #[test]
    fn parse_md5mesh() {
        let string =
            b"MD5Version 10
            commandline \"Exported from Blender by io_export_md5.py by Paul Zirkle\"

            numJoints 33
            numMeshes 6

            joints {
            	\"origin\"	-1 ( -0.000000 0.001643 -0.000604 ) ( -0.707107 -0.000242 -0.707107 )		//comment
            }

            mesh {
                shader \"bob_body\"

                numverts 1
                vert 0 ( 0.683594 0.455078 ) 0 3

                numtris 1
                tri 0 0 2 1

                numweights 1
                weight 0 16 0.333333 ( -0.194917 0.111128 -0.362937 )
            }

            ";

        let joint =
            Joint {
                name: String::from("origin"),
                parent_index: -1,
                position: Vector3::<f32> { x: -0.000000, y: 0.001643, z: -0.000604 },
                orientation: Quaternion::new(0.0, -0.707107, -0.000242, -0.707107)
            };

        let vertex =
            Vertex {
                index: 0,
                tex_coords: Vector2::new(0.683594, 0.455078),
                start_weight: 0,
                weight_count: 3
            };

        let triangle =
            Triangle {
                index: 0,
                vertex_indices: (0, 2, 1)
            };

        let weight =
            Weight {
                index: 0,
                joint_index: 16,
                bias: 0.333333,
                position: Vector3::new(-0.194917, 0.111128, -0.362937)
            };

        let mesh =
            Mesh {
                shader: String::from("bob_body"),
                vertices: vec![vertex],
                triangles: vec![triangle],
                weights: vec![weight]
            };

        let md5mesh =
            Md5Mesh {
                version: 10,
                command_line: String::from("Exported from Blender by io_export_md5.py by Paul Zirkle"),
                joints: vec![joint],
                meshes: vec![mesh]
            };

        assert_eq!(super::parse_md5mesh(string), Done(&b""[..], md5mesh));
    }
}
