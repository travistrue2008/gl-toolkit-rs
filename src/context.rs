use crate::{Result, Error};

use flagset::{FlagSet, flags};
use gl::types::*;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::os::raw::c_void;
use std::sync::Mutex;
use std::vec::Vec;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Feature {
    Blend,
    ColorLogicOp,
    CullFace,
    DepthClamp,
    DepthTest,
    Dither,
    FramebufferSrgb,
    LineSmooth,
    Multisample,
    PolygonOffsetFill,
    PolygonOffsetLine,
    PolygonOffsetPoint,
    PolygonSmooth,
    RasterizerDiscard,
    SampleAlphaToCoverage,
    SampleAlphaToOne,
    SampleCoverage,
    SampleShading,
    SampleMask,
    ScissorTest,
    StencilTest,
    TextureCubeMapSeamless,
    ProgramPointSize,
}

impl Feature {
    pub fn get_native(&self) -> GLenum {
        match self {
            Feature::Blend => gl::BLEND,
            Feature::ColorLogicOp => gl::COLOR_LOGIC_OP,
            Feature::CullFace => gl::CULL_FACE,
            Feature::DepthClamp => gl::DEPTH_CLAMP,
            Feature::DepthTest => gl::DEPTH_TEST,
            Feature::Dither => gl::DITHER,
            Feature::FramebufferSrgb => gl::FRAMEBUFFER_SRGB,
            Feature::LineSmooth => gl::LINE_SMOOTH,
            Feature::Multisample => gl::MULTISAMPLE,
            Feature::PolygonOffsetFill => gl::POLYGON_OFFSET_FILL,
            Feature::PolygonOffsetLine => gl::POLYGON_OFFSET_LINE,
            Feature::PolygonOffsetPoint => gl::POLYGON_OFFSET_POINT,
            Feature::PolygonSmooth => gl::POLYGON_SMOOTH,
            Feature::RasterizerDiscard => gl::RASTERIZER_DISCARD,
            Feature::SampleAlphaToCoverage => gl::SAMPLE_ALPHA_TO_COVERAGE,
            Feature::SampleAlphaToOne => gl::SAMPLE_ALPHA_TO_ONE,
            Feature::SampleCoverage => gl::SAMPLE_COVERAGE,
            Feature::SampleShading => gl::SAMPLE_SHADING,
            Feature::SampleMask => gl::SAMPLE_MASK,
            Feature::ScissorTest => gl::SCISSOR_TEST,
            Feature::StencilTest => gl::STENCIL_TEST,
            Feature::TextureCubeMapSeamless => gl::TEXTURE_CUBE_MAP_SEAMLESS,
            Feature::ProgramPointSize => gl::PROGRAM_POINT_SIZE,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BlendComponent {
    Zero,
    One,
    SrcColor,
    DstColor,
    SrcAlpha,
    DstAlpha,
    ConstColor,
    ConstAlpha,
    SrcAlphaSaturate,
    OneMinusSrcColor,
    OneMinusDstColor,
    OneMinusSrcAlpha,
    OneMinusDstAlpha,
    OneMinusConstColor,
    OneMinusConstAlpha,
}

impl BlendComponent {
    fn get_native(&self) -> GLenum {
        match self {
            BlendComponent::Zero => gl::ZERO,
            BlendComponent::One => gl::ONE,
            BlendComponent::SrcColor => gl::SRC_COLOR,
            BlendComponent::DstColor => gl::DST_COLOR,
            BlendComponent::SrcAlpha => gl::SRC_ALPHA,
            BlendComponent::DstAlpha => gl::DST_ALPHA,
            BlendComponent::ConstColor => gl::CONSTANT_COLOR,
            BlendComponent::ConstAlpha => gl::CONSTANT_ALPHA,
            BlendComponent::SrcAlphaSaturate => gl::SRC_ALPHA_SATURATE,
            BlendComponent::OneMinusSrcColor => gl::ONE_MINUS_SRC_COLOR,
            BlendComponent::OneMinusDstColor => gl::ONE_MINUS_DST_COLOR,
            BlendComponent::OneMinusSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
            BlendComponent::OneMinusDstAlpha => gl::ONE_MINUS_DST_COLOR,
            BlendComponent::OneMinusConstColor => gl::ONE_MINUS_CONSTANT_COLOR,
            BlendComponent::OneMinusConstAlpha => gl::ONE_MINUS_CONSTANT_ALPHA,
        }
    }
}

flags! {
    pub enum ClearFlag: GLbitfield {
        Color = gl::COLOR_BUFFER_BIT,
        Depth = gl::DEPTH_BUFFER_BIT,
        Stencil = gl::STENCIL_BUFFER_BIT,
    }
}

impl Display for ClearFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Copy, Clone)]
struct TextureUnit {
    d1_handle: GLuint,
    d2_handle: GLuint,
    d3_handle: GLuint,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Viewport {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Viewport {
    fn new() -> Viewport {
        Viewport {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

impl TextureUnit {
    fn new() -> TextureUnit {

        TextureUnit {
            d1_handle: 0,
            d2_handle: 0,
            d3_handle: 0,
        }
    }
}

struct State {
    initialized: bool,
    target_unit: GLuint,
    viewport: Viewport,
    features: HashSet<Feature>,
    units: Vec::<TextureUnit>,
}

lazy_static! {
    static ref INTERNAL_STATE: Mutex<State> = {
        Mutex::new(State {
            initialized: false,
            target_unit: 0,
            viewport: Viewport::new(),
            features: HashSet::new(),
            units: Vec::new(),
        })
    };
}

fn init_texture_units() {
    let mut st = INTERNAL_STATE.lock().unwrap();
    let max_units = unsafe {
        let mut count: i32 = 0;

        gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut count);

        for i in 0..count {
            gl::ActiveTexture(gl::TEXTURE0 + i as GLuint);
            gl::BindTexture(gl::TEXTURE_1D, 0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindTexture(gl::TEXTURE_3D, 0);
        }

        gl::ActiveTexture(gl::TEXTURE0);

        count as usize
    };

    st.units = vec![TextureUnit::new(); max_units];
}

pub fn init<F: FnMut(&'static str) -> *const c_void>(loader: F) -> Result<()> {
    if INTERNAL_STATE.lock().unwrap().initialized == false {
        gl::load_with(loader);
        init_texture_units();
        INTERNAL_STATE.lock().unwrap().initialized = true;

        Ok(())
    } else {
        Err(Error::AlreadyInitialized)
    }
}

pub fn enable(feature: Feature) -> bool {
    let result = INTERNAL_STATE.lock().unwrap().features.insert(feature);

    if result {
        unsafe { gl::Enable(feature.get_native()) };
    }

    result
}

pub fn disable(feature: Feature) -> bool {
    let result = INTERNAL_STATE.lock().unwrap().features.remove(&feature);

    if result {
        unsafe { gl::Disable(feature.get_native()) };
    }

    result
}

pub fn clear(flags: FlagSet<ClearFlag>) {
    unsafe { gl::Clear(flags.bits()) };
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe { gl::ClearColor(r, g, b, a) };
}

pub fn blend_func(src: BlendComponent, dst: BlendComponent) {
    unsafe { gl::BlendFunc(src.get_native(), dst.get_native()) };
}

pub fn activate_unit(unit: GLuint) {
    let mut st = INTERNAL_STATE.lock().unwrap();

    if st.target_unit != unit {
        unsafe { gl::ActiveTexture(gl::TEXTURE0 + unit); };

        st.target_unit = unit;
    }
}

pub fn set_viewport(x: u32, y: u32, width: u32, height: u32) {
    let mut st = INTERNAL_STATE.lock().unwrap();
    let viewport = Viewport { x, y, width, height };

    if st.viewport != viewport {
        unsafe { gl::Viewport(x as i32, y as i32, width as i32, height as i32) };

        st.viewport = viewport;
    }
}
