/*!
Shaders

Be sure to set uniform names (orelse, it fails or the order of uniforms would be borken).
*/

#![allow(unused)]

use crate::gfx::Shader;
use rokol::gfx::{self as rg, BakedResource};

/// Shorthand for specifying shader files
macro_rules! def_shd {
    ($file:expr) => {
        embed_shd!(
            concat!("glsl/", $file, ".vs"),
            concat!("glsl/", $file, ".fs"),
        )
    };
}

/// Loads shader files, dynamically on debug build, staticlly on release build
macro_rules! embed_shd {
    ($vs:expr, $fs:expr,) => {
        if cfg!(debug_assertions) {
            // debug: dynamically load the shader files
            let dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("src/shaders");
            let mut v = std::fs::read_to_string(dir.join($vs)).unwrap();
            v.push('\0');
            let mut f = std::fs::read_to_string(dir.join($fs)).unwrap();
            f.push('\0');
            [v, f]
        } else {
            // release: statically load the shader files
            [
                concat!(include_str!($vs), "\0").to_string(),
                concat!(include_str!($fs), "\0").to_string(),
            ]
        }
    };
}

/// Generates [`rg::Shader`]
fn gen(
    vs_fs: &[impl AsRef<str>; 2],
    mut_shd_desc: impl Fn(&mut rg::ShaderDesc),
    pip_desc: &mut rg::PipelineDesc,
) -> Shader {
    let mut shd_desc = unsafe { rokol::gfx::shader_desc(vs_fs[0].as_ref(), vs_fs[1].as_ref()) };
    mut_shd_desc(&mut shd_desc);

    let shd = rg::Shader::create(&shd_desc);

    pip_desc.shader = shd;
    let pip = rg::Pipeline::create(&pip_desc);

    Shader::new(shd, pip)
}

/// Sets image type
macro_rules! img_type {
    ($name:expr,$ty:expr) => {
        rg::ShaderImageDesc {
            name: concat!($name, "\0").as_ptr() as *const _,
            image_type: $ty as u32,
            ..Default::default()
        }
    };
}

/// Single-value uniform block
macro_rules! ub {
    ($name:expr, $uniform_ty:expr, $size_ty:ty) => {{
        let mut block = rg::ShaderUniformBlockDesc::default();

        block.uniforms[0] = rg::ShaderUniformDesc {
            name: concat!($name, "\0").as_ptr() as *const _,
            type_: $uniform_ty as u32,
            ..Default::default()
        };
        block.size += std::mem::size_of::<$size_ty>() as u64;

        block
    }};
}

/// (position, color) vertex
#[derive(Debug, Clone)]
#[repr(C)]
pub struct TriangleVertex {
    /// X, Y, Z
    pos: [f32; 3],
    /// R, G, B, A
    color: [f32; 4],
}

impl TriangleVertex {
    pub fn layout_desc() -> rg::LayoutDesc {
        let mut desc = rg::LayoutDesc::default();
        desc.attrs[0].format = rg::VertexFormat::Float3 as u32;
        desc.attrs[1].format = rg::VertexFormat::UByte4 as u32;
        desc
    }
}

impl<T, U> From<(T, U)> for TriangleVertex
where
    T: Into<[f32; 3]>,
    U: Into<[f32; 4]>,
{
    fn from(data: (T, U)) -> Self {
        Self {
            pos: data.0.into(),
            color: data.1.into(),
        }
    }
}

pub fn triangle() -> Shader {
    gen(
        &def_shd!("triangle"),
        |_shd| {},
        &mut rg::PipelineDesc {
            index_type: rg::IndexType::UInt16 as u32,
            layout: TriangleVertex::layout_desc(),
            cull_mode: rg::CullMode::None as u32,
            ..Default::default()
        },
    )
}
