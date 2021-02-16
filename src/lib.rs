/*!
Rokol Learn OpenGL examples
*/

pub mod gfx;
pub mod shaders;

use rokol::{app as ra, gfx as rg};

/// Runs a rokol application
///
/// It will postpone generation of our application until we setup `rokol::gfx`.
pub fn run<A: ra::RApp, G: FnOnce(&rokol::Rokol) -> A>(
    desc: rokol::Rokol,
    gen: G,
) -> rokol::Result {
    let mut runner = DelayedApp {
        desc: desc.clone(),
        app: None,
        gen: Some(gen),
    };
    desc.run(&mut runner)
}

#[derive(Debug)]
struct DelayedApp<A: ra::RApp, G: FnOnce(&rokol::Rokol) -> A> {
    desc: rokol::Rokol,
    app: Option<A>,
    gen: Option<G>,
}

impl<A: ra::RApp, G: FnOnce(&rokol::Rokol) -> A> ra::RApp for DelayedApp<A, G> {
    fn init(&mut self) {
        rg::setup(&mut rokol::glue::app_desc());
        self.app = Some(self.gen.take().unwrap()(&self.desc));
    }

    fn frame(&mut self) {
        self.app.as_mut().unwrap().frame();
    }
}
