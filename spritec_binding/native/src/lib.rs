use neon::prelude::*;
use spritec::config::{Camera, PresetCamera};
use spritec::math::{Vec3, Rgba};
use spritec::query3d::{File, GeometryFilter, GeometryQuery};
use spritec::renderer::{
    FileQuery, Outline, RenderCamera, RenderJob, RenderNode, RenderedImage, Size,
    ThreadRenderContext,
};
use spritec::tasks::config_to_camera;
use std::num::NonZeroU32;
use std::path::Path;
use std::sync::{Arc, Mutex};

fn version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(spritec::meta::version()))
}

fn render_sprite(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let mut ctx = ThreadRenderContext::new().expect("Unable to create ThreadRenderContext");

    let camera = PresetCamera::Custom(
        Camera {
            eye: Vec3 {x: 8.0, y: 8.0, z: 8.0},
            ..Default::default()
        }
    );

    let file = File::open(Path::new(
        // "/Users/bc/school/fydp/models/adventurers_camp_fairy_lantern/scene.gltf",
        // "/Users/bc/school/fydp/models/animation_v2/scene.gltf",
        "/Users/bc/school/fydp/spritec/samples/bigboi/gltf/bigboi.glb",
    ))
    .expect("Unable to open file");

    let job = RenderJob {
        scale: unsafe { NonZeroU32::new_unchecked(1) },
        root: RenderNode::RenderedImage(RenderedImage {
            size: Size {
                width: unsafe { NonZeroU32::new_unchecked(64) },
                height: unsafe { NonZeroU32::new_unchecked(64) },
            },
            background: Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            camera: RenderCamera::Camera(Arc::new(config_to_camera(camera))),
            lights: Vec::new(),
            geometry: FileQuery {
                query: GeometryQuery {
                    models: GeometryFilter::all_in_default_scene(),
                    animation: None,
                },
                file: Arc::new(Mutex::new(file)),
            },
            outline: Outline {
                thickness: 0.0,
                color: Rgba::black(),
            },
        }),
    };
    let image = job.execute(&mut ctx).expect("Sprite creation failed");

    let mut array_buffer = cx.array_buffer(image.width() * image.height() * 4)?;
    cx.borrow_mut(&mut array_buffer, |data| {
        let slice = data.as_mut_slice::<u8>();
        for (i, pixel) in image.pixels().enumerate() {
            slice[i * 4 + 0] = pixel[0];
            slice[i * 4 + 1] = pixel[1];
            slice[i * 4 + 2] = pixel[2];
            slice[i * 4 + 3] = pixel[3];
        }
    });
    Ok(array_buffer)
}

register_module!(mut cx, {
    cx.export_function("version", version)?;
    cx.export_function("render_sprite", render_sprite)?;
    Ok(())
});
