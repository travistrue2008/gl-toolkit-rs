use crate::Result;
use crate::Error;
use crate::Color;

use flagset::{FlagSet, flags};
use gl::types::*;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::sync::Mutex;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FrontFace {
    Clockwise,
    CounterClockwise,
}

impl FrontFace {
    fn get_native(&self) -> GLenum {
        match self {
            FrontFace::Clockwise => gl::CW,
            FrontFace::CounterClockwise => gl::CCW,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

struct State {
    initialized: bool,
    front: FrontFace,
    blend_src: BlendComponent,
    blend_dst: BlendComponent,
    clear_color: Color,
    viewport: Viewport,
    features: HashSet<Feature>,
}

lazy_static! {
    static ref INTERNAL_STATE: Mutex<State> = {
        Mutex::new(State {
            initialized: false,
            front: FrontFace::CounterClockwise,
            blend_src: BlendComponent::SrcAlpha,
            blend_dst: BlendComponent::OneMinusSrcAlpha,
            clear_color: Color::make(0, 0, 0, 0),
            viewport: Viewport::new(),
            features: HashSet::new(),
        })
    };
}

pub fn init() -> Result<()> {
    let mut st = INTERNAL_STATE.lock().unwrap();

    if st.initialized == false {
        st.initialized = true;

        unsafe {
            gl::FrontFace(gl::CCW);
            gl::Viewport(0, 0, 0, 0);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

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

pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
    let mut st = INTERNAL_STATE.lock().unwrap();
    let sr = st.clear_color.r as f32 / 255.0;
    let sg = st.clear_color.g as f32 / 255.0;
    let sb = st.clear_color.b as f32 / 255.0;
    let sa = st.clear_color.a as f32 / 255.0;

    if sr != r || sg != g || sb != b || sa != a {
        unsafe { gl::ClearColor(r, g, b, a) };

        st.clear_color = Color::make(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (a * 255.0) as u8,
        );
    }
}

pub fn set_front_face(target: FrontFace) {
    let mut st = INTERNAL_STATE.lock().unwrap();

    if st.front != target {
        unsafe { gl::FrontFace(target.get_native()) };

        st.front = target;
    }
}

pub fn set_blend_func(src: BlendComponent, dst: BlendComponent) {
    let mut st = INTERNAL_STATE.lock().unwrap();

    if st.blend_src != src || st.blend_dst != dst {
        unsafe { gl::BlendFunc(src.get_native(), dst.get_native()) };

        st.blend_src = src;
        st.blend_dst = dst;
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
