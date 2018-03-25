#![allow(dead_code)]
extern crate nom;
extern crate cgmath;

use cgmath::{Vector2, Vector3, Quaternion};
use std::str;
use std::str::FromStr;
use nom::digit;
use std::f32;

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
            opt!(tag!("(")) >>
            x: ws!(parse_f32) >>
            y: ws!(parse_f32) >>
            opt!(tag!(")")) >>
            (Vector2::new(x, y))
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
            opt!(tag!("(")) >>
            a: ws!(parse_f32) >>
            b: ws!(parse_f32) >>
            c: ws!(parse_f32) >>
            opt!(tag!(")")) >>
            (a, b, c)
        )
    )
);

named!(pub parse_vector3f32<&[u8], Vector3<f32>>,
    ws!(
        map!(
            parse_tuple3f32,
            |(a, b, c)| {
                Vector3::new(a, b, c)
            }
        )
    )
);

named!(pub parse_quaternionf32<&[u8], Quaternion<f32>>,
    ws!(
        map!(
            parse_tuple3f32,
            |(x, y, z)| {
                let mut scal : f32= 1.0 - x * x - y * y - z * z;
                if scal < 0.0 { scal = 0.0 };
                Quaternion::new(-scal.sqrt(), x, y, z) 
            }
        )
    )
);
