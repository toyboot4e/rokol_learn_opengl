use rokol::{app as ra, gfx as rg};

use crate::{
    gfx::{Shader, StaticMesh},
    shaders,
};

/// Draws a triangle
#[derive(Debug)]
pub struct TriangleApp {
    /// Clears the frame color buffer on starting screen rendering pass
    pa: rg::PassAction,
    /// Triangle shader
    shd: Shader,
    /// Buffer for the triangle shader
    mesh: StaticMesh<shaders::TriangleVertex>,
}

impl TriangleApp {
    pub fn new() -> Self {
        // set up a triangle
        let verts: &[shaders::TriangleVertex] = &[
            // (vertex, color)
            ([0.0, 0.5, 0.5], [1.0, 0.0, 0.0, 1.0]).into(), // top
            ([0.5, -0.5, 0.5], [0.0, 1.0, 0.0, 1.0]).into(), // bottom right
            ([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0, 1.0]).into(), // bottom left
        ];
        let indices: &[u16] = &[0, 1, 2];

        Self {
            pa: rg::PassAction::clear([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0]),
            shd: shaders::triangle(),
            mesh: StaticMesh::new_16(verts, indices),
        }
    }
}

impl rokol::app::RApp for TriangleApp {
    // 60 FPS game loop provided by `rokol::app`
    fn frame(&mut self) {
        self.update();
        self.render();
        rg::commit();
    }
}

impl TriangleApp {
    fn update(&mut self) {
        //
    }

    fn render(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        self.shd.apply_pip();
        self.mesh.draw_all();
        rg::end_pass();
    }
}
