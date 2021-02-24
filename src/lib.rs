/*!
Rokol Learn OpenGL examples
*/

pub mod apps;
pub mod gfx;
pub mod shaders;

use rokol::{app as ra, gfx as rg};
use std::path::PathBuf;

use crate::gfx::{Shader, StaticMesh, Texture2dDrop, TextureBuilder};

#[derive(Debug)]
pub struct TextureApp {
    /// Clears the frame color buffer on starting screen rendering pass
    pa: rg::PassAction,
    shd: Shader,
    tex: Texture2dDrop,
    mesh: StaticMesh<shaders::TextureVertex>,
}

impl TextureApp {
    pub fn new() -> Self {
        let color = [100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0];

        let shd = shaders::texture();

        let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let path = root.join("assets/tex/rpg.png");
        let path = root.join("assets/tex/container.jpg");
        let tex = TextureBuilder::from_path(&path)
            .unwrap()
            // .filter(rg::Filter::Nearest)
            .build_texture();

        let mut quad_mesh = {
            let verts: &[shaders::TextureVertex] = &[
                // pos, color, uv
                ([-0.5, -0.5, 0.0], [255, 255, 255, 255], [0.0, 0.0]).into(),
                ([0.5, -0.5, 0.0], [255, 255, 255, 255], [1.0, 0.0]).into(),
                ([0.5, 0.5, 0.0], [255, 255, 255, 255], [1.0, 1.0]).into(),
                ([-0.5, 0.5, 0.0], [255, 255, 255, 255], [0.0, 1.0]).into(),
            ];
            let indices: &[u16] = &[0, 1, 2, 0, 2, 3];

            StaticMesh::new_16(verts, indices)
        };

        quad_mesh.bind_img(tex.img(), 0);

        Self {
            pa: rg::PassAction::clear(color),
            shd,
            tex,
            mesh: quad_mesh,
        }
    }
}

impl rokol::app::RApp for TextureApp {
    fn frame(&mut self) {
        self.update();
        self.render();
        rg::commit();
    }
}

impl TextureApp {
    fn update(&mut self) {}

    fn render(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        self.shd.apply_pip();
        self.mesh.draw_all();
        rg::end_pass();
    }
}
