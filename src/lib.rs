//! The main spritec executable

// TOOL POLICY:
// - We add tools in order to help *us* improve our code
// - If they are not doing that, we can configure them or even elect to remove them
// - No tool is perfect and we are allowed to disagree with its results
// - If the tool warns about something that isn't actually an issue worth caring about, add it to
//   the list below and explain your change in your PR
// - We don't want to litter our code with #[allow] attributes unnecessarily, so try to either
//   globally disable that aspect of the tool or live with it and do what the tool says
// - If we make a mistake and find that one of these lints shouldn't have been added here, we can
//   always remove it later
#![deny(clippy::all)] // Deny clippy warnings when running clippy (used for CI)
#![allow(
    clippy::identity_op,
    clippy::let_and_return,
    clippy::cast_lossless,
)]

pub mod color;
pub mod geometry;
pub mod light;
pub mod loaders;
pub mod material;
pub mod scale;
pub mod shaders;

use std::f32::consts::PI;

use euc::{Pipeline, rasterizer, buffer::Buffer2d, Target};
use vek::{Mat4, Vec2, Vec3, Vec4, Rgba};

use crate::color::{rgba_to_bgra_u32, bgra_u32_to_rgba, vek_rgba_to_image_rgba};
use crate::geometry::Mesh;
use crate::light::DiffuseLight;
use crate::shaders::{CelShader, OutlineShader};
use crate::scale::scale_buffer;

pub fn render(
    color: &mut Buffer2d<Rgba<f32>>,
    depth: &mut Buffer2d<f32>,
    model: Mat4<f32>,
    view: Mat4<f32>,
    projection: Mat4<f32>,
    meshes: &[Mesh],
    outline_thickness: f32,
) {
    // Must be multiplied backwards since each point to be multiplied will be on the right
    let mvp = projection * view * model;

    for mesh in meshes {
        OutlineShader {
            mvp,

            mesh,

            outline_color: Rgba {r: 0.0, g: 0.0, b: 0.0, a: 0.0},
            outline_thickness,
        }.draw::<rasterizer::Triangles<_>, _>(mesh.indices(), color, depth);

        CelShader {
            mvp,
            model_inverse_transpose: model.inverted().transposed(),

            mesh,

            light: DiffuseLight {
                direction: Vec3::from(view * Vec4::forward_lh()),
                color: Rgba::white(),
                intensity: 1.0,
            },

            ambient_intensity: 0.5,
        }.draw::<rasterizer::Triangles<_>, _>(mesh.indices(), color, depth);
    }
}
