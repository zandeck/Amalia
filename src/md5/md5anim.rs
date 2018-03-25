#![allow(dead_code)]
extern crate cgmath;

use cgmath::{Vector3, Quaternion};

#[derive(Clone, PartialEq, Debug)]
pub struct Joint {
    pub name: String,
    pub index: i32,
    pub flag: i32,
    pub start_index: i32
}

#[derive(Clone, PartialEq, Debug)]
pub struct Bound {
    pub bound_min: Vector3<f32>,
    pub bound_max: Vector3<f32>
}

#[derive(Clone, PartialEq, Debug)]
pub struct BaseFrame {
    pub position: Vec<Vector3<f32>>,
    pub orientation: Vec<Quaternion<f32>>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Frame {
    pub frame_number: u32,
    pub frame_data: Vec<f32>
}


#[derive(Clone, PartialEq, Debug)]
pub struct Md5Anim {
    pub version: i32,
    pub command_line: String,
    pub num_frames: i32,
    pub num_joints: i32,
    pub frame_rate: i32,
    pub num_animated_components: i32,
    pub hierarchies: Vec<Joint>,
    pub bounds: Vec<Bound>,
    pub base_frame: BaseFrame,
    pub frames: Vec<Frame>
}
