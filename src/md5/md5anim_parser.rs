use md5::md5common_parser::*;
use md5::md5anim;
use maths::vector3::Vector3;
use maths::vector2::Vector2;
use maths::quaternion::Quaternion;

named!(pub parse_header<&[u8], (i32, String, i32, i32, i32, i32)>,
    do_parse!(
        tag!("MD5Version ") >>
        version: parse_i32 >>
        take_until_and_consume!("commandline ") >>
        command_line: escaped_string >>
        take_until_and_consume!("numFrames ") >>
        numFrame: parse_i32 >> 
        take_until_and_consume!("numJoints ") >>
        numJoints: parse_i32 >> 
        take_until_and_consume!("frameRate ") >>
        frameRate: parse_i32 >> 
        take_until_and_consume!("numAnimatedComponents ") >>
        numAnimatedComponents: parse_i32 >> 
    (version, command_line, numFrame, numJoints, frameRate, numAnimatedComponents))
);

named!(pub parse_joint<&[u8], md5anim::Joint>,
    do_parse!(
        jointName: escaped_string >>
        parentIndex: ws!(parse_i32) >>
        flags: ws!(parse_i32) >>
        startIndex: ws!(parse_i32) >>
        opt!(ws!(comments)) >>
        (
            md5anim::Joint {
                name: jointName,
                index: parentIndex,
                flag: flags,
                start_index: startIndex
            }
        )
    )
);

named!(pub parse_hierarchy<&[u8], Vec<md5anim::Joint> >,
    do_parse!(
        ws!(tag!("hierarchy")) >>
        ws!(tag!("{")) >>
        joints: many1!(parse_joint) >>
        ws!(tag!("}")) >>
        (joints)
    )
);

named!(pub parse_bound<&[u8], md5anim::Bound>, 
    do_parse!(
        ws!(tag!("(")) >>
        b1: parse_vector3f32 >>
        ws!(tag!(")")) >>
        ws!(tag!("(")) >>
        b2: parse_vector3f32 >>
        ws!(tag!(")")) >>
        (
            md5anim::Bound {
                bound_min: b1,
                bound_max: b2
            }
        )
    )
);

named!(pub parse_bounds<&[u8], Vec<md5anim::Bound> >,
    do_parse!(
        ws!(tag!("bounds")) >>
        ws!(tag!("{")) >>
        bounds: many1!(parse_bound) >>
        ws!(tag!("}")) >>
        (bounds)
    )
);

named!(pub pos_and_orientation<&[u8], (Vector3<f32>, Quaternion<f32>) >,
    do_parse!(
        ws!(tag!("(")) >>
        p: parse_vector3f32 >>
        ws!(tag!(")")) >>
        ws!(tag!("(")) >>
        o: parse_quaternionf32 >>
        ws!(tag!(")")) >>
        (p, o)
    )
 );

 named!(pub parse_baseframe<&[u8], md5anim::BaseFrame>,
    do_parse!(
        ws!(tag!("baseframe")) >>
        ws!(tag!("{")) >>
        r: fold_many1!(pos_and_orientation, (Vec::new(), Vec::new()), | mut acc: (Vec<Vector3<f32> >, Vec<Quaternion<f32> >), item: (Vector3<f32>, Quaternion<f32>) | {
            acc.0.push(item.0);
            acc.1.push(item.1);
            acc
        }) >>
        ws!(tag!("}")) >>
        (
            md5anim::BaseFrame {
                position: r.0,
                orientation: r.1
            }
        )
    )
 );

named!(pub parse_frame<&[u8], md5anim::Frame>, 
    do_parse!(
        ws!(tag!("frame")) >>
        frame_number: ws!(parse_u32) >>
        ws!(tag!("{")) >>
        frame_data: ws!(many0!(parse_f32)) >> 
        ws!(tag!("}")) >>
        (
            md5anim::Frame {
                frame_number: frame_number,
                frame_data: frame_data
            }
        )
    )
);

named!(pub parse_frames<&[u8], Vec<md5anim::Frame> >, 
    do_parse!(
        frames: many1!(parse_frame) >>
        (frames)
    )
);

named!(pub parse_anim<&[u8], md5anim::Md5Anim>, 
    do_parse!(
        header: parse_header >>
        hierarchy: parse_hierarchy >>
        bounds: parse_bounds >>
        baseframe: parse_baseframe >>
        frames: parse_frames >>
        (
            md5anim::Md5Anim {
                version: header.0,
                command_line: header.1,
                num_frames: header.2,
                num_joints: header.3,
                frame_rate: header.4,
                num_animated_components: header.5,
                hierarchies: hierarchy,
                bounds: bounds,
                base_frame: baseframe,
                frames: frames
            }
        )

    )
);

#[cfg(test)]
mod test {
    use md5::md5anim_parser;
    use nom::IResult::Done; 
    use md5::md5anim;
    use maths::vector3::Vector3;
    use maths::vector2::Vector2;
    use maths::quaternion::Quaternion;

    #[test]
    fn parse_header() {
        let string = b"MD5Version 10
        commandline \"Exported from Blender by io_export_md5.py by Paul Zirkle\"

        numFrames 141
        numJoints 33
        frameRate 24
        numAnimatedComponents 198";
        let header = (10, String::from("Exported from Blender by io_export_md5.py by Paul Zirkle"), 141, 33, 24, 198);
        assert_eq!(super::parse_header(string), Done(&b""[..], header));
    }

    #[test]
    fn parse_joint() {
        let string = b"\"origin\"	-1 63 0	//\n";
        let header = md5anim::Joint { name: String::from("origin"), index: -1, flag: 63, start_index: 0 };
        assert_eq!(super::parse_joint(string), Done(&b""[..], header));
    }

    #[test]
    fn parse_hierarchy() {
        let string = b"hierarchy {
        \"origin\"	-1 63 0	//
        \"sheath\"	0 63 6	// origin
        \"sword\"	1 63 12	// sheath
        }";
        let mut res = Vec::new();
        res.push(md5anim::Joint { name: String::from("origin"), index: -1, flag: 63, start_index: 0 });
        res.push(md5anim::Joint { name: String::from("sheath"), index: 0, flag: 63, start_index: 6 });
        res.push(md5anim::Joint { name: String::from("sword"), index: 1, flag: 63, start_index: 12 });
        assert_eq!(super::parse_hierarchy(string), Done(&b""[..], res));
    }

    #[test]
    fn parse_bound() {
        let string = b"	( -1.634066 -1.634066 -1.634066 ) ( -1.634066 6.444685 5.410537 )";
        let b = md5anim::Bound { bound_min: Vector3 { x: -1.634066, y: -1.634066, z: -1.634066 }, bound_max: Vector3 { x: -1.634066, y: 6.444685, z: 5.410537 } };
        assert_eq!(super::parse_bound(string), Done(&b""[..], b));
    }

    #[test]
    fn parse_bounds() {
        let string = b"bounds {
        ( -1.634066 -1.634066 -1.634066 ) ( -1.634066 6.444685 5.410537 )
        ( -1.634381 -1.634381 -1.634381 ) ( -1.634381 6.444589 5.410597 )
        ( -1.634190 -1.634190 -1.634190 ) ( -1.634190 6.444603 5.410734 )
        }"; 
        let mut bounds = Vec::new();
        bounds.push(md5anim::Bound { bound_min: Vector3 { x: -1.634066, y: -1.634066, z: -1.634066 }, bound_max: Vector3 { x: -1.634066, y: 6.444685, z: 5.410537 } });
        bounds.push(md5anim::Bound { bound_min: Vector3 { x: -1.634381, y: -1.634381, z: -1.634381 }, bound_max: Vector3 { x: -1.634381, y: 6.444589, z: 5.410597 } });
        bounds.push(md5anim::Bound { bound_min: Vector3 { x: -1.63419, y: -1.63419, z: -1.63419 }, bound_max: Vector3 { x: -1.63419, y: 6.444603, z: 5.410734 } });
        assert_eq!(super::parse_bounds(string), Done(&b""[..], bounds));
    }

    #[test]
    fn pos_and_orientation() {
        let string = b"
	    ( 3.122890 0.625194 0.923663 ) ( -0.022398 0.133633 0.852234 )";

        let l = (Vector3::<f32> { x: 3.12289, y: 0.625194, z: 0.923663 }, Quaternion::<f32> { scal: 0.25533772, vec: Vector3 { x: -0.022398, y: 0.133633, z: 0.852234 } });
        assert_eq!(super::pos_and_orientation(string), Done(&b""[..], l));
    }

    #[test]
    fn parse_baseframe() {
        let string = b"baseframe {
        ( 3.122890 0.625194 0.923663 ) ( -0.022398 0.133633 0.852234 )
        ( 0.000386 -1.102681 0.010090 ) ( 0.001203 -0.000819 0.001678 )
        }";
        let mut positionVector = Vec::new();
        let mut orientationVector = Vec::new();

        positionVector.push(Vector3 { x: 3.12289, y: 0.625194, z: 0.923663 });
        positionVector.push( Vector3 { x: 0.000386, y: -1.102681, z: 0.01009 });

        orientationVector.push(Quaternion { scal: 0.25533772, vec: Vector3 { x: -0.022398, y: 0.133633, z: 0.852234 } });
        orientationVector.push(Quaternion { scal: 0.9999951, vec: Vector3 { x: 0.001203, y: -0.000819, z: 0.001678 } });

        let r = md5anim::BaseFrame { position: positionVector, orientation: orientationVector};

        assert_eq!(super::parse_baseframe(string), Done(&b""[..], r));
    }

    #[test]
    fn parse_frame() {
        let string = b"frame 0 {
        000000.001643 -0.000604 -0.707107 -0.000242 -0.707107
        3.122890 0.625194 0.923663 0.022398 -0.133633 -0.852234
        0.000386 -1.102681 0.010090 -0.001203 0.000819 -0.001678
        2.600285 -0.203020 0.001408 0.750740 
        }";
        let frame_data = vec![0.001643, -0.000604, -0.707107, -0.000242, -0.707107, 3.12289, 0.625194, 0.923663, 0.022398, -0.133633, -0.852234, 0.000386, -1.102681, 0.01009, -0.001203, 0.000819, -0.001678, 2.600285, -0.20302, 0.001408, 0.75074];
        let frame = md5anim::Frame {frame_number: 0, frame_data: frame_data};

        assert_eq!(super::parse_frame(string), Done(&b""[..], frame));
    }

    #[test]
    fn parse_frames() {
        let string = b"frame 0 {
        000000.001643 -0.000604 -0.707107 -0.000242 -0.707107
        3.122890 0.625194 0.923663 0.022398 -0.133633 -0.852234
        0.000386 -1.102681 0.010090 -0.001203 0.000819 -0.001678
        2.600285 -0.203020 0.001408 0.750740
        }
        
        frame 1 {
        000000.001643 -0.000604 -0.707107 -0.000242 -0.707107
        3.122890 0.625194 0.923663 0.022398 -0.133633 -0.852234
        0.000386 -1.102681 0.010090 -0.001203 0.000819 -0.001678
        2.600285 -0.203020 0.001408 0.750740
        }
        
        frame 2 {
        000000.001643 -0.000604 -0.707107 -0.000242 -0.707107
        3.122890 0.625194 0.923663 0.022398 -0.133633 -0.852234
        0.000386 -1.102681 0.010090 -0.001203 0.000819 -0.001678
        2.600285 -0.203020 0.001408 0.750740
        }";
        let r = vec![md5anim::Frame { frame_number: 0, frame_data: vec![0.001643, -0.000604, -0.707107, -0.000242, -0.707107, 3.12289, 0.625194, 0.923663, 0.022398, -0.133633, -0.852234, 0.000386, -1.102681, 0.01009, -0.001203, 0.000819, -0.001678, 2.600285, -0.20302, 0.001408, 0.75074] }, md5anim::Frame { frame_number: 1, frame_data: vec![0.001643, -0.000604, -0.707107, -0.000242, -0.707107, 3.12289, 0.625194, 0.923663, 0.022398, -0.133633, -0.852234, 0.000386, -1.102681, 0.01009, -0.001203, 0.000819, -0.001678, 2.600285, -0.20302, 0.001408, 0.75074] }, md5anim::Frame { frame_number: 2, frame_data: vec![0.001643, -0.000604, -0.707107, -0.000242, -0.707107, 3.12289, 0.625194, 0.923663, 0.022398, -0.133633, -0.852234, 0.000386, -1.102681, 0.01009, -0.001203, 0.000819, -0.001678, 2.600285, -0.20302, 0.001408, 0.75074] }];

        assert_eq!(super::parse_frames(string), Done(&b""[..], r));
    }

    #[test]
    fn parse_anim() {
        let string = b"MD5Version 10
        commandline \"Exported from Blender by io_export_md5.py by Paul Zirkle\"

        numFrames 141
        numJoints 33
        frameRate 24
        numAnimatedComponents 198
        
        hierarchy {
        \"origin\"	-1 63 0	//
        \"sheath\"	0 63 6	// origin
        \"sword\"	1 63 12	// sheath
        }

        bounds {
        ( -1.634066 -1.634066 -1.634066 ) ( -1.634066 6.444685 5.410537 )
        ( -1.634381 -1.634381 -1.634381 ) ( -1.634381 6.444589 5.410597 )
        ( -1.634190 -1.634190 -1.634190 ) ( -1.634190 6.444603 5.410734 )
        }

        baseframe {
        ( 3.122890 0.625194 0.923663 ) ( -0.022398 0.133633 0.852234 )
        ( 0.000386 -1.102681 0.010090 ) ( 0.001203 -0.000819 0.001678 )
        }

        frame 0 {
        000000.001643 -0.000604 -0.707107 -0.000242 -0.707107
        3.122890 0.625194 0.923663 0.022398 -0.133633 -0.852234
        0.000386 -1.102681 0.010090 -0.001203 0.000819 -0.001678
        2.600285 -0.203020 0.001408 0.750740
        }
        
        frame 1 {
        000000.001643 -0.000604 -0.707107 -0.000242 -0.707107
        3.122890 0.625194 0.923663 0.022398 -0.133633 -0.852234
        0.000386 -1.102681 0.010090 -0.001203 0.000819 -0.001678
        2.600285 -0.203020 0.001408 0.750740
        }
        
        frame 2 {
        000000.001643 -0.000604 -0.707107 -0.000242 -0.707107
        3.122890 0.625194 0.923663 0.022398 -0.133633 -0.852234
        0.000386 -1.102681 0.010090 -0.001203 0.000819 -0.001678
        2.600285 -0.203020 0.001408 0.750740
        }
        ";

        let header = (10, String::from("Exported from Blender by io_export_md5.py by Paul Zirkle"), 141, 33, 24, 198);
        let mut hierarchy = Vec::new();
        let mut bounds = Vec::new();
        let mut positionVector = Vec::new();
        let mut orientationVector = Vec::new();

        hierarchy.push(md5anim::Joint { name: String::from("origin"), index: -1, flag: 63, start_index: 0 });
        hierarchy.push(md5anim::Joint { name: String::from("sheath"), index: 0, flag: 63, start_index: 6 });
        hierarchy.push(md5anim::Joint { name: String::from("sword"), index: 1, flag: 63, start_index: 12 });
        
        bounds.push(md5anim::Bound { bound_min: Vector3 { x: -1.634066, y: -1.634066, z: -1.634066 }, bound_max: Vector3 { x: -1.634066, y: 6.444685, z: 5.410537 } });
        bounds.push(md5anim::Bound { bound_min: Vector3 { x: -1.634381, y: -1.634381, z: -1.634381 }, bound_max: Vector3 { x: -1.634381, y: 6.444589, z: 5.410597 } });
        bounds.push(md5anim::Bound { bound_min: Vector3 { x: -1.63419, y: -1.63419, z: -1.63419 }, bound_max: Vector3 { x: -1.63419, y: 6.444603, z: 5.410734 } });

        positionVector.push(Vector3 { x: 3.12289, y: 0.625194, z: 0.923663 });
        positionVector.push( Vector3 { x: 0.000386, y: -1.102681, z: 0.01009 });

        orientationVector.push(Quaternion { scal: 0.25533772, vec: Vector3 { x: -0.022398, y: 0.133633, z: 0.852234 } });
        orientationVector.push(Quaternion { scal: 0.9999951, vec: Vector3 { x: 0.001203, y: -0.000819, z: 0.001678 } });
        let baseframe = md5anim::BaseFrame { position: positionVector, orientation: orientationVector};

        let frames = vec![md5anim::Frame { frame_number: 0, frame_data: vec![0.001643, -0.000604, -0.707107, -0.000242, -0.707107, 3.12289, 0.625194, 0.923663, 0.022398, -0.133633, -0.852234, 0.000386, -1.102681, 0.01009, -0.001203, 0.000819, -0.001678, 2.600285, -0.20302, 0.001408, 0.75074] }, md5anim::Frame { frame_number: 1, frame_data: vec![0.001643, -0.000604, -0.707107, -0.000242, -0.707107, 3.12289, 0.625194, 0.923663, 0.022398, -0.133633, -0.852234, 0.000386, -1.102681, 0.01009, -0.001203, 0.000819, -0.001678, 2.600285, -0.20302, 0.001408, 0.75074] }, md5anim::Frame { frame_number: 2, frame_data: vec![0.001643, -0.000604, -0.707107, -0.000242, -0.707107, 3.12289, 0.625194, 0.923663, 0.022398, -0.133633, -0.852234, 0.000386, -1.102681, 0.01009, -0.001203, 0.000819, -0.001678, 2.600285, -0.20302, 0.001408, 0.75074] }];

        let res = md5anim::Md5Anim {
            version: header.0,
            command_line: header.1,
            num_frames: header.2,
            num_joints: header.3,
            frame_rate: header.4,
            num_animated_components: header.5,
            hierarchies: hierarchy,
            bounds: bounds,
            base_frame: baseframe,
            frames: frames
        };

        assert_eq!(super::parse_anim(string), Done(&b""[..], res));

    }
}