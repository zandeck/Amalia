use std::{str, cmp};
use std::str::FromStr;
use maths::vector3::Vector3;
use maths::vector2::Vector2;
use maths::quaternion::Quaternion;
use nom::{digit, multispace};
use md5::md5mesh::{Md5Mesh, Joint, Vertex, Mesh, Triangle, Weight};

named!(pub escaped_string<&[u8], String>,
    map_res!(
        delimited!(
            tag!("\""),
            fold_many0!(
                is_not!("\""),
                Vec::new(),
                |mut acc: Vec<u8>, bytes: &[u8]| {
                    acc.extend(bytes);
                    acc
                }
            ),
            tag!("\"")
        ),
        String::from_utf8
    )
);

named!(pub comments<&[u8]>,
    preceded!(
        tag!("//"),
        take_until_and_consume!("\n")
    )
);

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

named!(pub parse_u32<&[u8], u32>,
    map_opt!(
        map_res!(
            digit,
            str::from_utf8
        ),
        |str| u32::from_str(str).ok()
    )
);

named!(pub parse_i<&[u8], (bool, &str)>,
    ws!(
        do_parse!(
            neg: opt!(ws!(tag!("-"))) >>
            int: map_res!(digit, str::from_utf8) >>
            (neg.is_some(), int)
        )
    )
);

named!(pub parse_i32<&[u8], i32>,
    map_opt!(
        parse_i,
        | (neg, int) : (bool, &str) |
            i32::from_str(int)
            .ok()
            .map(|v| if neg {-v} else {v})
    )
);

named!(pub parse_f<&[u8], (bool, &str, Option<&str>)>,
    ws!(
        do_parse!(
            neg: opt!(ws!(tag!("-"))) >>
            int: map_res!(digit, str::from_utf8) >>
            dec_opt:
                opt!(
                    do_parse!(
                        tag!(".") >>
                        dec: map_res!(digit, str::from_utf8) >>
                        (dec)
                    )
                ) >> (neg.is_some(), int, dec_opt)
        )
    )
);

named!(pub parse_f32<&[u8], f32>,
    map_opt!(
        parse_f,
        | (neg, int, dec_opt) : (bool, &str, Option<&str>) |
            match dec_opt {
                Some(dec) => f32::from_str(&(String::from(int) + "." +  dec)),
                None => f32::from_str(int)
            }
            .ok()
            .map(|v| if neg {-v} else {v})
    )
);

named!(pub parse_vector2f32<&[u8], Vector2<f32>>,
    ws!(
        do_parse!(
            x: ws!(parse_f32) >>
            y: ws!(parse_f32) >>
            (Vector2::<f32> {x: x, y: y})
        )
    )
);

named!(pub parse_tuple3u32<&[u8], (u32, u32, u32)>,
    ws!(
        do_parse!(
            a: ws!(parse_u32) >>
            b: ws!(parse_u32) >>
            c: ws!(parse_u32) >>
            (a, b, c)
        )
    )
);

named!(pub parse_tuple3f32<&[u8], (f32, f32, f32)>,
    ws!(
        do_parse!(
            a: ws!(parse_f32) >>
            b: ws!(parse_f32) >>
            c: ws!(parse_f32) >>
            (a, b, c)
        )
    )
);

named!(pub parse_vector3f32<&[u8], Vector3<f32>>,
    ws!(
        map!(
            parse_tuple3f32,
            |(a, b, c)| {
                Vector3::<f32> {x: a, y: b, z: c}
            }
        )
    )
);

named!(pub parse_quaternionf32<&[u8], Quaternion<f32>>,
    ws!(
        map!(
            parse_tuple3f32,
            |(a, b, c)| {
                let mut d = 1.0 - a * a - b * b - c * c;
                if d < 0.0 { d = 0.0 };
                Quaternion::<f32> {
                    scal: a,
                    vec:
                        Vector3::<f32> {
                            x: b,
                            y: c,
                            z: d
                        }
                    }
            }
        )
    )
);

named!(pub parse_joints<&[u8], Vec<Joint>>,
    preceded!(
        tag!("joints"),
        delimited!(
            ws!(tag!("{")),
            many0!(
                do_parse!(
                    name: ws!(escaped_string) >>
                    parent_index: ws!(parse_i32) >>
                    ws!(tag!("(")) >>
                    position: ws!(parse_vector3f32) >>
                    ws!(tag!(")")) >>
                    ws!(tag!("(")) >>
                    orientation: ws!(parse_quaternionf32) >>
                    ws!(tag!(")")) >>
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
        ws!(tag!("(")) >>
        tex_coords: ws!(parse_vector2f32) >>
        ws!(tag!(")")) >>
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
        ws!(tag!("(")) >>
        position: ws!(parse_vector3f32) >>
        ws!(tag!(")")) >>
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
    use nom::IResult::Done;
    use std::str;
    use std::str::FromStr;
    use maths::vector3::Vector3;
    use maths::vector2::Vector2;
    use maths::quaternion::Quaternion;
    use nom::{digit, alphanumeric, multispace, anychar, be_u8, line_ending, be_f32};
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
                position: Vector3::<f32> { x: -0.000000, y: 0.001643, z: -0.000604 },
                orientation: Quaternion::<f32> {
                    scal: -0.707107,
                    vec: Vector3::<f32> {
                        x: -0.000242,
                        y: -0.707107,
                        z: 0.0
                    }
                }
            };

        let joint2 =
            Joint {
                name: String::from("sheath"),
                parent_index: 0,
                position: Vector3::<f32> { x: 1.100481, y: -0.317714, z: 3.170247 },
                orientation: Quaternion::<f32> {
                    scal: 0.307041,
                    vec: Vector3::<f32> {
                        x: -0.578615,
                        y: 0.354181,
                        z: 0.4454863
                    }
                }
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
                tex_coords: Vector2::<f32> {x: 0.683594, y: 0.455078},
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
                tex_coords: Vector2::<f32> {x: 0.683594, y: 0.455078},
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
                position: Vector3::<f32> { x: -0.194917, y: 0.111128, z: -0.362937 }
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
                orientation: Quaternion::<f32> {
                    scal: -0.707107,
                    vec: Vector3::<f32> {
                        x: -0.000242,
                        y: -0.707107,
                        z: 0.0
                    }
                }
            };

        let vertex =
            Vertex {
                index: 0,
                tex_coords: Vector2::<f32> {x: 0.683594, y: 0.455078},
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
                position: Vector3::<f32> { x: -0.194917, y: 0.111128, z: -0.362937 }
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
