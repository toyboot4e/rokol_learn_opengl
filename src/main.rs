/*!
Learn OpenGL Rokol examples
*/

use rokol::{app as ra, gfx as rg};

use rokol_learn_opengl::{
    gfx::{Shader, StaticMesh},
    shaders,
};

fn main() -> rokol::Result {
    // give implementation to log crate
    env_logger::init();

    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol Learn OpenGL examples".to_string(),
        ..Default::default()
    };

    rokol_learn_opengl::run(rokol, |_rokol| AppData::new())
}

#[derive(Debug)]
struct AppData {
    /// Clears the frame color buffer on starting screen rendering pass
    pa: rg::PassAction,
    /// Triangle shader
    shd: Shader,
    /// Buffer for the triangle shader
    mesh: StaticMesh<shaders::TriangleVertex>,
}

impl AppData {
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

impl rokol::app::RApp for AppData {
    fn init(&mut self) {
        rg::setup(&mut rokol::glue::app_desc());
    }

    // 60 FPS game loop provided by `rokol::app`
    fn frame(&mut self) {
        self.update();
        self.render();
        rg::commit();
    }
}

impl AppData {
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
