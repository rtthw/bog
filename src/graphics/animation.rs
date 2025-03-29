//! Animation types & functions



/*

Example #1:
    Want: "clockwise, half rotation about the z-axis, repeat every 2 seconds"
    Get: |time| {
        Mat4::from_angle_z(Deg((time % 2.0) * (-180.0 / 2.0)))
    }

*/



pub fn rotate_z_degrees(time: f32, deg: f32, speed: f32) -> three_d::Mat4 {
    three_d::Mat4::from_angle_z(three_d::Deg(time * (deg / speed)))
}

pub fn rotate_z_degrees_repeat(time: f32, deg: f32, speed: f32) -> three_d::Mat4 {
    three_d::Mat4::from_angle_z(three_d::Deg((time % speed) * (deg / speed)))
}
