/*!
Learn OpenGL Rokol examples
*/

use rokol_learn_opengl::apps::texture::TextureApp as AppData;

fn main() -> rokol::Result {
    // give implementation to log crate
    env_logger::init();

    let desc = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol Learn OpenGL example".to_string(),
        ..Default::default()
    };

    rokol::run(desc, |_rokol| AppData::new())
}
