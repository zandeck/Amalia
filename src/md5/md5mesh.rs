#![allow(dead_code)]
use cgmath::{Vector3, Vector2, Quaternion};

#[derive(Clone, PartialEq, Debug)]
pub struct Joint {
    pub name: String,
    pub parent_index: i32,
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vertex {
    pub index: u32,
    pub tex_coords: Vector2<f32>,
    pub start_weight: u32,
    pub weight_count: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Triangle {
    pub index: u32,
    pub vertex_indices: (u32, u32, u32),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Weight {
    pub index: u32,
    pub joint_index: u32,
    pub bias: f32,
    pub position: Vector3<f32>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Mesh {
    pub shader: String,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub weights: Vec<Weight>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Md5Mesh {
    pub version: u8,
    pub command_line: String,
    pub joints: Vec<Joint>,
    pub meshes: Vec<Mesh>,
}
