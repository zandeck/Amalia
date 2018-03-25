#![allow(dead_code)]
extern crate cgmath;

use md5::md5common_parser::*;
use md5::md5anim;
use cgmath::{Vector3, Quaternion};

named!(pub parse_header<&[u8], (i32, String, i32, i32, i32, i32)>,
    do_parse!(
        tag!("MD5Version ") >>
        version: parse_i32 >>
        take_until_and_consume!("commandline ") >>
        command_line: escaped_string >>
        take_until_and_consume!("numFrames ") >>
        num_frame: parse_i32 >> 
        take_until_and_consume!("numJoints ") >>
        num_joints: parse_i32 >> 
        take_until_and_consume!("frameRate ") >>
        frame_rate: parse_i32 >> 
        take_until_and_consume!("numAnimatedComponents ") >>
        num_animated_components: parse_i32 >> 
    (version, command_line, num_frame, num_joints, frame_rate, num_animated_components))
);

named!(pub parse_joint<&[u8], md5anim::Joint>,
    do_parse!(
        joint_name: escaped_string >>
        parent_index: ws!(parse_i32) >>
        flags: ws!(parse_i32) >>
        start_index: ws!(parse_i32) >>
        opt!(ws!(comments)) >>
        (
            md5anim::Joint {
                name: joint_name,
                index: parent_index,
                flag: flags,
                start_index: start_index
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
        b1: parse_vector3f32 >>
        b2: parse_vector3f32 >>
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
        p: parse_vector3f32 >>
        o: parse_quaternionf32 >>
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
    extern crate cgmath;
    use nom::IResult::Done; 
    use md5::md5anim;
    use cgmath::{Vector3, Quaternion};

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
        let b = md5anim::Bound { bound_min: Vector3::new(-1.634066, -1.634066, -1.634066), bound_max: Vector3::new(-1.634066, 6.444685, 5.410537) };
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
        bounds.push(md5anim::Bound { bound_min: Vector3::new(-1.634066, -1.634066, -1.634066 ), bound_max: Vector3::new(-1.634066, 6.444685, 5.410537) });
        bounds.push(md5anim::Bound { bound_min: Vector3::new(-1.634381, -1.634381, -1.634381 ), bound_max: Vector3::new(-1.634381, 6.444589, 5.410597) });
        bounds.push(md5anim::Bound { bound_min: Vector3::new(-1.63419, -1.63419, -1.63419 ), bound_max: Vector3::new(-1.63419, 6.444603, 5.410734) });
        assert_eq!(super::parse_bounds(string), Done(&b""[..], bounds));
    }

    #[test]
    fn pos_and_orientation() {
        let string = b"
	    ( 3.122890 0.625194 0.923663 ) ( -0.022398 0.133633 0.852234 )";

        let l = (Vector3::new(3.12289, 0.625194, 0.923663), Quaternion::new(0.25533772, -0.022398, 0.133633, 0.852234 ));
        assert_eq!(super::pos_and_orientation(string), Done(&b""[..], l));
    }

    #[test]
    fn parse_baseframe() {
        let string = b"baseframe {
        ( 3.122890 0.625194 0.923663 ) ( -0.022398 0.133633 0.852234 )
        ( 0.000386 -1.102681 0.010090 ) ( 0.001203 -0.000819 0.001678 )
        }";
        let mut position_vector = Vec::new();
        let mut orientation_vector = Vec::new();

        position_vector.push(Vector3::new(3.12289, 0.625194, 0.923663));
        position_vector.push(Vector3::new(0.000386, -1.102681, 0.01009));

        orientation_vector.push(Quaternion::new(0.25533772, -0.022398, 0.133633, 0.852234 ));
        orientation_vector.push(Quaternion::new(0.9999951, 0.001203, -0.000819, 0.001678));

        let r = md5anim::BaseFrame { position: position_vector, orientation: orientation_vector};

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
        let mut position_vector = Vec::new();
        let mut orientation_vector = Vec::new();

        hierarchy.push(md5anim::Joint { name: String::from("origin"), index: -1, flag: 63, start_index: 0 });
        hierarchy.push(md5anim::Joint { name: String::from("sheath"), index: 0, flag: 63, start_index: 6 });
        hierarchy.push(md5anim::Joint { name: String::from("sword"), index: 1, flag: 63, start_index: 12 });
        
        bounds.push(md5anim::Bound { bound_min: Vector3::new(-1.634066, -1.634066, -1.634066), bound_max: Vector3::new(-1.634066, 6.444685, 5.410537)});
        bounds.push(md5anim::Bound { bound_min: Vector3::new(-1.634381, -1.634381, -1.634381), bound_max: Vector3::new(-1.634381, 6.444589, 5.410597)});
        bounds.push(md5anim::Bound { bound_min: Vector3::new(-1.63419, -1.63419, -1.63419), bound_max: Vector3::new(-1.63419, 6.444603, 5.410734)});

        position_vector.push(Vector3::new(3.12289, 0.625194, 0.923663));
        position_vector.push(Vector3::new(0.000386, -1.102681, 0.01009));

        orientation_vector.push(Quaternion::new(0.25533772, -0.022398, 0.133633, 0.852234 ));
        orientation_vector.push(Quaternion::new(0.9999951, 0.001203, -0.000819, 0.001678 ));
        let baseframe = md5anim::BaseFrame { position: position_vector, orientation: orientation_vector};

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