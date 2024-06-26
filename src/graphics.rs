use bytemuck::{Pod, Zeroable};
use glam::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::geometry::{Ray, Aabb};



pub fn compact_color(mut color: Vec4) -> [u8; 4] {
    color = Vec4::clamp(255.0 * color, Vec4::ZERO, Vec4::splat(255.0));

    [
        color.x as u8,
        color.y as u8,
        color.z as u8,
        color.w as u8,
    ]
}



#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Pod, Zeroable)]
pub struct Color([u8; 4]);

impl Color {
    pub const DEFAULT: Self = Self([0, 0, 0, 255]);

    pub const fn new(hex: u32) -> Self {
        Self(hex.to_le_bytes())
    }

    pub fn from_vec4(value: Vec4) -> Self {
        Self(compact_color(value))
    }
}



#[repr(C)]
#[derive(Clone, Debug, PartialEq, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Pod, Zeroable)]
pub struct Camera {
    pub distance: f32,
    pub theta: f32,
    pub phi: f32,
    pub vfov: f32,
    pub target_pos: Vec3,
}

impl Camera {
    pub fn spherical_to_cartesian(radius: f32, theta: f32, phi: f32) -> Vec3 {
        radius * Vec3::new(
            phi.sin() * theta.sin(),
            phi.cos(),
            phi.sin() * theta.cos(),
        )
    }

    pub fn shoot_ray(&self, screen_coord: Vec2, aspect_ratio: f32) -> Ray {
        let camera_pos = self.target_pos + Self::spherical_to_cartesian(
            self.distance, self.theta, self.phi,
        );
        let camera_direction = Vec3::normalize(self.target_pos - camera_pos);
        let camera_tangent = -self.theta.sin() * Vec3::Z + self.theta.cos() * Vec3::X;
        let camera_bitangent = Vec3::cross(camera_direction, camera_tangent);

        let fov_tan = f32::tan(0.5 * self.vfov);
        let direction = Vec3::normalize(camera_direction
            + (screen_coord.x / aspect_ratio) * fov_tan * camera_tangent
            + screen_coord.y * fov_tan * camera_bitangent
        );
        let origin = camera_pos;

        Ray { direction, origin }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            distance: 1.3,
            theta: std::f32::consts::FRAC_PI_2,
            phi: std::f32::consts::FRAC_PI_2,
            target_pos: Vec3::ZERO,
            vfov: std::f32::consts::FRAC_PI_4,
        }
    }
}



pub const RENDER_TARGET_COLOR: u32 = 0;
pub const RENDER_TARGET_DENSITY: u32 = 1;



#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Default, Copy, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum RenderTarget {
    #[default]
    Color = 0,
    Density = 1,
}

impl TryFrom<u32> for RenderTarget {
    type Error = RenderTargetParseU32Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            RENDER_TARGET_COLOR => Self::Color,
            RENDER_TARGET_DENSITY => Self::Density,
            _ => return Err(RenderTargetParseU32Error(value)),
        })
    }
}

impl std::str::FromStr for RenderTarget {
    type Err = RenderTargetParseStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "color" => Self::Color,
            "density" => Self::Density,
            _ => return Err(RenderTargetParseStrError(s.to_owned())),
        })
    }
}

impl std::fmt::Display for RenderTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Color => "color",
            Self::Density => "density",
        })
    }
}



#[derive(Debug, Error)]
#[error("failed to parse RenderTarget, imvalid argument '{0}'")]
pub struct RenderTargetParseStrError(pub String);



#[derive(Debug, Error)]
#[error("failed to parse RenderTarget, invalid argument '{0}'")]
pub struct RenderTargetParseU32Error(pub u32);



#[repr(C)]
#[derive(Clone, Debug, PartialEq, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Pod, Zeroable)]
pub struct RenderConfiguration {
    pub camera: Camera,
    pub rm_settings: RaymarchSettings,
    pub bounding_box: Aabb,
    #[serde(skip)]
    pub render_target: u32,
}

impl Default for RenderConfiguration {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            rm_settings: RaymarchSettings::default(),
            bounding_box: Aabb::default().with_translation(Vec3::splat(-0.5)),
            render_target: RENDER_TARGET_COLOR,
        }
    }
}



#[repr(C)]
#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Pod, Zeroable)]
pub struct RaymarchSettings {
    pub n_steps: u32,
}

impl Default for RaymarchSettings {
    fn default() -> Self {
        Self { n_steps: 300 }
    }
}