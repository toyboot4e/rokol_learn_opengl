/*!
Shaders

Be sure to set uniform names (orelse, it fails or the order of uniforms would be borken).
*/

#![allow(unused)]

use rokol::gfx::{self as rg, BakedResource, Shader};

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
