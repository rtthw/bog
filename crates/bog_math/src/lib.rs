//! Bog Math



pub extern crate cgmath;



pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec4 = cgmath::Vector4<f32>;



pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}



pub type Mat2 = cgmath::Matrix2<f32>;
pub type Mat3 = cgmath::Matrix3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;



pub fn mat3_translation(translation: Vec2) -> Mat3 {
    Mat3::from_translation(translation)
}

pub fn mat4_translation(translation: Vec3) -> Mat4 {
    Mat4::from_translation(translation)
}

pub fn mat3_scale(scale: f32) -> Mat3 {
    Mat3::from_scale(scale)
}

pub fn mat4_scale(scale: f32) -> Mat4 {
    Mat4::from_scale(scale)
}

pub fn mat3_scale_nonuniform(scale: Vec2) -> Mat3 {
    Mat3::from_nonuniform_scale(scale.x, scale.y)
}

pub fn mat4_scale_nonuniform(scale: Vec3) -> Mat4 {
    Mat4::from_nonuniform_scale(scale.x, scale.y, scale.z)
}
