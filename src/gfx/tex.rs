use {
    image::GenericImageView,
    rokol::gfx::{self as rg, BakedResource},
    std::{borrow::Cow, path::Path},
};

/// Image loading result
pub type Result<T> = image::ImageResult<T>;

#[derive(Debug)]
pub struct TextureBuilder<'a> {
    pixels: Cow<'a, [u8]>,
    size: [u32; 2],
    filter: rg::Filter,
    wrap: rg::Wrap,
}

impl TextureBuilder<'static> {
    pub fn from_path(path: &Path) -> Result<Self> {
        Ok(Self::from_dyn_img(image::open(path)?))
    }

    pub fn from_encoded_bytes(mem: &[u8]) -> Result<Self> {
        Ok(Self::from_dyn_img(image::load_from_memory(mem)?))
    }

    fn from_dyn_img(img: image::DynamicImage) -> Self {
        let size = [img.width(), img.height()];

        // [OpenGL] invert vertically
        let img = img.flipv();

        // NOTE: It can be, for example, RGB8 image
        let img: Vec<u8> = img.into_rgba8().into_raw();

        Self {
            pixels: Cow::from(img),
            size,
            filter: rg::Filter::Linear,
            wrap: rg::Wrap::ClampToEdge,
        }
    }
}

impl<'a> TextureBuilder<'a> {
    pub fn from_pixels(pixels: &'a [u8], w: u32, h: u32) -> Self {
        Self {
            pixels: Cow::from(pixels),
            size: [w, h],
            filter: rg::Filter::Linear,
            wrap: rg::Wrap::ClampToEdge,
        }
    }

    pub fn filter(&mut self, filter: rg::Filter) -> &mut Self {
        self.filter = filter;
        self
    }

    pub fn wrap(&mut self, wrap: rg::Wrap) -> &mut Self {
        self.wrap = wrap;
        self
    }

    pub fn build_texture(&self) -> Texture2dDrop {
        Texture2dDrop {
            img: rg::Image::create(&{
                let mut desc = self::img_desc(self.size[0], self.size[1], self.filter, self.wrap);
                desc.render_target = false;
                desc.usage = rg::ResourceUsage::Immutable as u32;
                desc.data.subimage[0][0] = self.pixels.as_ref().into();
                desc
            }),
            w: self.size[0],
            h: self.size[1],
        }
    }
}

fn img_desc(w: u32, h: u32, filter: rg::Filter, wrap: rg::Wrap) -> rg::ImageDesc {
    rg::ImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        width: w as i32,
        height: h as i32,
        min_filter: filter as u32,
        mag_filter: filter as u32,
        wrap_u: wrap as u32,
        wrap_v: wrap as u32,
        wrap_w: wrap as u32,
        ..Default::default()
    }
}

// fn target_desc(w: u32, h: u32, filter: rg::Filter, wrap: rg::Wrap) -> rg::ImageDesc {
//     let mut desc = image_desc_2d(w, h, filter, wrap);
//     desc.render_target = true;
//     // TODO:
//     // desc.usage = rg::ResourceUsage::Dynamic as u32;
//     desc
// }

// fn stencil_desc(w: u32, h: u32, filter: rg::Filter, wrap: rg::Wrap) -> rg::ImageDesc {
//     let mut desc = image_desc_2d(w, h, filter, wrap);
//     desc.render_target = true;
//     desc.pixel_format = rg::PixelFormat::DepthStencil as u32;
//     // TODO:
//     // desc.usage = rg::ResourceUsage::Dynamic as u32;
//     // desc.sample_count = 1;
//     desc
// }

/// Owned 2D texture
#[derive(Debug, Default)]
pub struct Texture2dDrop {
    img: rg::Image,
    w: u32,
    h: u32,
}

impl Drop for Texture2dDrop {
    fn drop(&mut self) {
        rg::Image::destroy(self.img);
    }
}

impl Texture2dDrop {
    /// Prefer to use [`TextureBuilder`]
    pub fn new(img: rg::Image, w: u32, h: u32) -> Self {
        Self { img, w, h }
    }

    pub fn w(&self) -> u32 {
        self.w
    }

    pub fn h(&self) -> u32 {
        self.h
    }

    pub fn size(&self) -> [u32; 2] {
        [self.w, self.h]
    }

    pub fn img(&self) -> rg::Image {
        self.img
    }
}

/// Off-screen 2D rendering target
#[derive(Debug, Default)]
pub struct RenderTexture2d {
    /// Render target texture binded to the rendering pass
    tex: Texture2dDrop,
    /// Off-screen rendering pass
    pass: rg::Pass,
}

impl Drop for RenderTexture2d {
    fn drop(&mut self) {
        rg::Pass::destroy(self.pass);
    }
}

impl RenderTexture2d {
    /// [`rokol::gfx::Pass`] for off-screen rendering
    pub fn pass(&self) -> rg::Pass {
        self.pass
    }

    pub fn tex(&self) -> &Texture2dDrop {
        &self.tex
    }

    pub fn w(&self) -> u32 {
        self.tex.w
    }

    pub fn h(&self) -> u32 {
        self.tex.h
    }

    pub fn size(&self) -> [u32; 2] {
        [self.tex.w, self.tex.h]
    }

    pub fn img(&self) -> rg::Image {
        self.tex.img
    }
}
