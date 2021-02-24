//! Textured cube

use std::path::PathBuf;

use {
    glam::Mat4,
    rokol::{app as ra, gfx as rg},
};

use crate::{
    gfx::{Shader, StaticMesh, Texture2dDrop, TextureBuilder},
    shaders::{self, CubeVertex},
};

fn gen_cube_mesh() -> StaticMesh<CubeVertex> {
    StaticMesh::new_16(
        // vertices
        &[
            // six rectangles
            ([-1.0, -1.0, -1.0], [255, 255, 255, 255], [0.0, 0.0]).into(),
            ([1.0, -1.0, -1.0], [255, 255, 255, 255], [1.0, 0.0]).into(),
            ([1.0, 1.0, -1.0], [255, 255, 255, 255], [1.0, 1.0]).into(),
            ([-1.0, 1.0, -1.0], [255, 255, 255, 255], [0.0, 1.0]).into(),
            //
            ([-1.0, -1.0, 1.0], [255, 255, 255, 255], [0.0, 0.0]).into(),
            ([1.0, -1.0, 1.0], [255, 255, 255, 255], [1.0, 0.0]).into(),
            ([1.0, 1.0, 1.0], [255, 255, 255, 255], [1.0, 1.0]).into(),
            ([-1.0, 1.0, 1.0], [255, 255, 255, 255], [0.0, 1.0]).into(),
            //
            ([-1.0, -1.0, -1.0], [255, 255, 255, 255], [0.0, 0.0]).into(),
            ([-1.0, 1.0, -1.0], [255, 255, 255, 255], [1.0, 0.0]).into(),
            ([-1.0, 1.0, 1.0], [255, 255, 255, 255], [1.0, 1.0]).into(),
            ([-1.0, -1.0, 1.0], [255, 255, 255, 255], [0.0, 1.0]).into(),
            //
            ([1.0, -1.0, -1.0], [255, 255, 255, 255], [0.0, 0.0]).into(),
            ([1.0, 1.0, -1.0], [255, 255, 255, 255], [1.0, 0.0]).into(),
            ([1.0, 1.0, 1.0], [255, 255, 255, 255], [1.0, 1.0]).into(),
            ([1.0, -1.0, 1.0], [255, 255, 255, 255], [0.0, 1.0]).into(),
            //
            ([-1.0, -1.0, -1.0], [255, 255, 255, 255], [0.0, 0.0]).into(),
            ([-1.0, -1.0, 1.0], [255, 255, 255, 255], [1.0, 0.0]).into(),
            ([1.0, -1.0, 1.0], [255, 255, 255, 255], [1.0, 1.0]).into(),
            ([1.0, -1.0, -1.0], [255, 255, 255, 255], [0.0, 1.0]).into(),
            //
            ([-1.0, 1.0, -1.0], [255, 255, 255, 255], [0.0, 0.0]).into(),
            ([-1.0, 1.0, 1.0], [255, 255, 255, 255], [1.0, 0.0]).into(),
            ([1.0, 1.0, 1.0], [255, 255, 255, 255], [1.0, 1.0]).into(),
            ([1.0, 1.0, -1.0], [255, 255, 255, 255], [0.0, 1.0]).into(),
        ],
        // indices
        &[
            0, 1, 2, 0, 2, 3, // one rectangle
            6, 5, 4, 7, 6, 4, //
            8, 9, 10, 8, 10, 11, //
            14, 13, 12, 15, 14, 12, //
            16, 17, 18, 16, 18, 19, //
            22, 21, 20, 23, 22, 20, //
        ],
    )
}

#[derive(Debug)]
pub struct CubeApp {
    pa: rg::PassAction,
    shd: Shader,
    tex: Texture2dDrop,
    mesh: StaticMesh<CubeVertex>,
}

impl CubeApp {
    pub fn new() -> Self {
        let color = [100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0];
        let shd = shaders::cube();

        let tex = {
            let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
            let path = root.join("assets/tex/container.jpg");
            TextureBuilder::from_path(&path).unwrap().build_texture()
        };

        let mut mesh = self::gen_cube_mesh();
        mesh.bind_img(tex.img(), 0);

        Self {
            pa: rg::PassAction::clear(color),
            shd,
            tex,
            mesh,
        }
    }
}

impl rokol::app::RApp for CubeApp {
    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());

        self.shd.apply_pip();

        {
            // TODO: rotate
            // let spd =
            // let rot_x = glam::Mat4::rotate(rx, [1.0f, 0.0f, 0.0f]);
            // let rot_y = glam::Mat4::rotate(ry, [0.0f, 1.0f, 0.0f]);
            // let model = rot_x * rot_y;

            // left-handed matrices
            let view = Mat4::look_at_rh(
                // camera position
                [2.0, 2.0, 4.0].into(),
                // focal point
                [0.0, 0.0, 0.0].into(),
                // up direction
                [0.0, 1.0, 0.0].into(),
            );

            let ratio = ra::width() as f32 / ra::height() as f32;
            let proj = Mat4::perspective_rh(
                3.14 / 3.0, // fov_y_radian
                ratio,      // aspect_ratio
                0.01,       // z_near
                100.0,      // z_far
            );

            // column-major matrix notation (v' = Mv)
            let vp = proj * view;

            let bytes: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    vp.as_ref() as *const _ as *const _,
                    std::mem::size_of::<Mat4>(),
                )
            };
            rg::apply_uniforms(rg::ShaderStage::Vs, 0, bytes);
        }

        self.mesh.draw_all();

        rg::end_pass();
        rg::commit();
    }
}
