use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    math::{Matrix, Point3, Vec3},
    query::{Ray, World},
    rendering::Canvas,
};

/// Default recursion depth when computing reflections.
pub const DEFAULT_RECURSION_DEPTH: u32 = 5;

/// A perspective 3D camera.
#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    size: (u32, u32),
    fov: f32,
    transform: Matrix,
    recursion_limit: u32,

    // Derived parameters
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
}

/// Prefab for a [`Camera`].
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraPrefab {
    /// The width of this camera's canvas.
    pub width: u32,
    /// The height of this camera's canvas.
    pub height: u32,
    /// The field of view espressed in degrees.
    pub fov: f32,
    /// The location of the observer's eye.
    pub from: Point3,
    /// The observed point.
    pub to: Point3,
    /// The up vector of the camera.
    pub up: Vec3,
}

impl From<CameraPrefab> for Camera {
    fn from(prefab: CameraPrefab) -> Self {
        prefab.build()
    }
}

impl Camera {
    /// Creates a perspective camera with the given screen dimensions and field of view.
    ///
    /// The view transformation will be multiplicative identity.
    pub fn new(hsize: u32, vsize: u32, fov: f32) -> Self {
        Self::new_with_transform(hsize, vsize, fov, Matrix::identity(4))
    }

    /// Creates a new perspective camera with a view transform matrix.
    pub fn new_with_transform(hsize: u32, vsize: u32, fov: f32, transform: Matrix) -> Self {
        let mut camera = Camera {
            size: (hsize, vsize),
            fov,
            transform,
            recursion_limit: DEFAULT_RECURSION_DEPTH,
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
        };

        camera.update();
        camera
    }

    /// Returns the horizontal screen size of this camera.
    pub fn horizontal_size(&self) -> u32 {
        self.size.0
    }

    /// Returns the vertical screen size of this camera.
    pub fn vertical_size(&self) -> u32 {
        self.size.1
    }

    /// Updates this camera's screen size.
    pub fn set_size(&mut self, hsize: u32, vsize: u32) {
        self.size = (hsize, vsize);
        self.update();
    }

    /// Returns the camera's field of view.
    pub fn fov(&self) -> f32 {
        self.fov
    }

    /// Updates this camera's field of view.
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.update();
    }

    /// Updates this camera's view transform.
    pub fn view_transform(&self) -> &Matrix {
        &self.transform
    }

    /// Sets this camera's view transform.
    pub fn set_view_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.update();
    }

    /// Returns the size in world-space units of a pixel on the canvas.
    pub fn pixel_size(&self) -> f32 {
        self.pixel_size
    }

    /// Constructs a ray originating at the camera position and directed towards point `(x,y)`
    /// in the canvas.
    pub fn ray_to(&self, x: u32, y: u32) -> Ray {
        // offset from the edge of the canvas to the pixel's center
        let xoffset = (x as f32 + 0.5) * self.pixel_size;
        let yoffset = (y as f32 + 0.5) * self.pixel_size;

        // untransformed coordinates of the pixel in world space
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // transform the canvas point and the origin, then compute the ray's direction vector
        let t_inv = self.transform.inverse().unwrap();
        let pixel = &t_inv * Point3::new(world_x, world_y, -1.0);
        let origin = &t_inv * Point3::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    /// Renders `world` to a canvas through this camera.
    pub fn render(&self, world: &World) -> Canvas {
        Stream::new(self, world).finalize()
    }

    /// Renders `world` through this camera line-by-line.
    pub fn stream<'a, 'b>(&'a self, world: &'b World) -> Stream<'a, 'b> {
        Stream::new(self, world)
    }

    fn update(&mut self) {
        let half_view = (self.fov / 2.0).tan();
        let aspect_ratio = self.horizontal_size() as f32 / self.vertical_size() as f32;

        if aspect_ratio >= 1.0 {
            self.half_width = half_view;
            self.half_height = half_view / aspect_ratio;
        } else {
            self.half_width = half_view * aspect_ratio;
            self.half_height = half_view;
        };

        self.pixel_size = self.half_width * 2.0 / self.horizontal_size() as f32;
    }
}

/// Streaming iterator over the scanlines produced by [`Camera::render`].
#[derive(Debug)]
pub struct Stream<'a, 'b> {
    camera: &'a Camera,
    world: &'b World,
    canvas: Canvas,
    threads: usize,
    current_line: u32,
}

impl<'a, 'b> Stream<'a, 'b> {
    /// Creates a new stream that will render `world` as seen by `camera`.
    pub fn new(camera: &'a Camera, world: &'b World) -> Self {
        Self {
            camera,
            world,
            canvas: Canvas::new(camera.horizontal_size(), camera.vertical_size()),
            threads: num_cpus::get(),
            current_line: 0,
        }
    }

    /// Returns the canvas associated to this stream.
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    /// Computes and return the next scanline, returning `true` if more processing is needed.
    pub fn advance(&mut self) -> bool {
        if self.current_line >= self.camera.vertical_size() {
            return false;
        }

        let Stream {
            ref camera,
            ref world,
            ..
        } = self;

        let y = self.current_line;

        self.canvas
            .scanlines_mut(self.current_line as usize, self.threads)
            .enumerate()
            .par_bridge()
            .for_each(|(i, line)| {
                for x in 0..camera.horizontal_size() {
                    let ray = camera.ray_to(x, y + i as u32);
                    let color = world
                        .color_at(&ray, camera.recursion_limit)
                        .unwrap_or_default();

                    line[x as usize] = color;
                }
            });

        self.current_line += self.threads as u32;
        true
    }

    /// Finish rendering this stream and return the final canvas.
    pub fn finalize(mut self) -> Canvas {
        while self.advance() {}
        self.canvas
    }
}

impl CameraPrefab {
    /// Builds a `Camera` from this prefab.
    pub fn build(self) -> Camera {
        Camera::new_with_transform(
            self.width,
            self.height,
            self.fov.to_radians(),
            Matrix::look_at(self.from, self.to, self.up),
        )
    }
}

#[cfg(all(feature = "serde-support", test))]
mod tests {
    use serde_test::{assert_de_tokens, Token};

    use super::*;

    #[test]
    fn prefab_to_camera() {
        let expected = Camera::new_with_transform(
            640,
            480,
            60.0_f32.to_radians(),
            Matrix::look_at(
                (1.0, 2.0, 3.0).into(),
                (4.0, 5.0, 6.0).into(),
                Vec3::unit_y(),
            ),
        );

        let result = CameraPrefab {
            width: 640,
            height: 480,
            fov: 60.0,
            from: (1.0, 2.0, 3.0).into(),
            to: (4.0, 5.0, 6.0).into(),
            up: (0.0, 1.0, 0.0).into(),
        }
        .build();

        assert_eq!(result, expected);
    }

    #[test]
    fn deserialize() {
        let prefab = CameraPrefab {
            width: 640,
            height: 480,
            fov: 60.0,
            from: (1.0, 2.0, 3.0).into(),
            to: (4.0, 5.0, 6.0).into(),
            up: (0.0, 1.0, 0.0).into(),
        };

        assert_de_tokens(
            &prefab,
            &[
                Token::Struct {
                    name: "CameraPrefab",
                    len: 6,
                },
                Token::Str("width"),
                Token::U32(640),
                Token::Str("height"),
                Token::U32(480),
                Token::Str("fov"),
                Token::F32(60.0),
                Token::Str("from"),
                Token::Seq { len: Some(3) },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::SeqEnd,
                Token::Str("to"),
                Token::Seq { len: Some(3) },
                Token::F32(4.0),
                Token::F32(5.0),
                Token::F32(6.0),
                Token::SeqEnd,
                Token::Str("up"),
                Token::Seq { len: Some(3) },
                Token::F32(0.0),
                Token::F32(1.0),
                Token::F32(0.0),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
