//! Mesh type



pub use three_d::{CpuMesh, Mat2, Mat3, Mat4};

use three_d::{Geometry as _, Srgba};

use super::{Render, Renderer};



pub struct Mesh {
    pub(crate) inner: three_d::Mesh,
    // NOTE: The only reason this whole type exists (at the moment) is because `three-d` doesn't
    //       let you track animations.
    animating: bool,
}

// Builder.
impl Mesh {
    pub fn new(renderer: &Renderer, cpu_mesh: &CpuMesh) -> Self {
        Self {
            inner: three_d::Mesh::new(renderer, cpu_mesh),
            animating: false,
        }
    }

    pub fn with_animation<A>(mut self, animation: A) -> Self
    where A: Fn(f32) -> three_d::Mat4 + Send + Sync + 'static,
    {
        self.animate(animation);
        self
    }
}

impl Mesh {
    pub fn transform(&mut self, transformation: three_d::Mat4) {
        self.inner.set_transformation(transformation);
    }

    // See: `three_d::Mesh::set_transformation_2d`.
    pub fn transform_2d(&mut self, transformation: Mat3) {
        self.inner.set_transformation(Mat4::new(
            transformation.x.x,
            transformation.x.y,
            0.0,
            transformation.x.z,
            transformation.y.x,
            transformation.y.y,
            0.0,
            transformation.y.z,
            0.0,
            0.0,
            1.0,
            0.0,
            transformation.z.x,
            transformation.z.y,
            0.0,
            transformation.z.z,
        ));
    }

    pub fn animate<A>(&mut self, animation: A)
    where A: Fn(f32) -> three_d::Mat4 + Send + Sync + 'static,
    {
        self.inner.set_animation(animation);
        self.animating = true;
    }

    pub fn perform_animation(&mut self, seconds_since_start: f32) {
        self.inner.animate(seconds_since_start);
    }
}

impl Mesh {
    pub fn aabb(&self) -> three_d::AxisAlignedBoundingBox {
        self.inner.aabb()
    }
}



pub struct ColoredMesh {
    pub mesh: Mesh,
    pub color: Srgba,
}

impl Render for ColoredMesh {
    fn objects(&self) -> impl Iterator<Item = impl three_d::Object> {
        std::iter::once(three_d::Gm::new(
            &self.mesh.inner,
            three_d::ColorMaterial {
                color: self.color,
                ..Default::default()
            },
        ))
    }
}
