/*!
Higher-level objects on [`rokol::gfx`]

They clear resource on `drop` and wrap `rokol` functions in methods.
*/

mod mesh;
mod shader;
mod tex;

pub use mesh::{DynamicMesh, StaticMesh};
pub use shader::Shader;
pub use tex::{RenderTexture2d, Texture2dDrop};
