/*!
Learn OpenGL Rokol examples
*/

// use glue code for `sokol_app.h` (`sapp`) + `sokol_gfx.h`:
use rokol::glue::sapp::{run_delayed, Result, Rokol};

// select other `AppData` to swtich application
use rokol_learn_opengl::apps::CubeApp as AppData;

fn main() -> Result {
    // give implementation to log crate
    env_logger::init();

    let desc = Rokol {
        w: 1280,
        h: 720,
        title: "Rokol Learn OpenGL example".to_string(),
        ..Default::default()
    };

    run_delayed(desc, |_rokol| AppData::new())
}
