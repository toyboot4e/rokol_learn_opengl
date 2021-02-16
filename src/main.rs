/*!
Learn OpenGL Rokol examples
*/

use rokol::{app as ra, gfx as rg};

fn main() -> rokol::Result {
    // give implementation to log crate
    env_logger::init();

    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Clear".to_string(),
        ..Default::default()
    };

    let mut app = AppData::new();

    rokol.run(&mut app)
}

#[derive(Debug)]
struct AppData {
    /// Clears the frame color buffer on starting screen rendering pass
    pa: rg::PassAction,
}

impl AppData {
    pub fn new() -> Self {
        let pa = rg::PassAction::clear([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0]);

        Self { pa }
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
        // start rendering pass to the screen (the frame buffer)
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::end_pass();
    }
}
