use bevy_math::{AspectRatio, UVec2, Vec2};
use std::fmt::{Display, Formatter};

/// Represents a specific resolution
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Resolution {
    width: f32,
    height: f32,
    aspect_ratio: AspectRatioMode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum AspectRatioMode {
    Dynamic,
    Set(AspectRatio),
}

impl AspectRatioMode {
    fn is_dynamic(&self) -> bool {
        matches!(self, AspectRatioMode::Dynamic)
    }
}

impl Resolution {
    pub fn new(width: f32, height: f32) -> Self {
        Resolution {
            width,
            height,
            aspect_ratio: AspectRatioMode::Dynamic,
        }
    }

    pub fn from_height(height: f32, aspect_ratio: AspectRatio) -> Self {
        Resolution {
            height,
            width: height * aspect_ratio.ratio(),
            aspect_ratio: AspectRatioMode::Set(aspect_ratio),
        }
    }

    pub fn from_width(width: f32, aspect_ratio: AspectRatio) -> Self {
        Resolution {
            width,
            height: width / aspect_ratio.ratio(),
            aspect_ratio: AspectRatioMode::Set(aspect_ratio),
        }
    }

    pub fn aspect_ratio(&self) -> AspectRatio {
        match self.aspect_ratio {
            AspectRatioMode::Dynamic => AspectRatio::try_new(self.width, self.height).unwrap(),
            AspectRatioMode::Set(ar) => ar,
        }
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn has_integer_scale(&self, target_resolution: &Resolution) -> bool {
        has_integer_scale(self, target_resolution)
    }

    pub fn scale_factor(&self, target_resolution: &Resolution) -> Vec2 {
        get_scale_factor(self, target_resolution)
    }

    pub fn change_height(mut self, height: f32, maintain_aspect_ratio: bool) -> Self {
        if maintain_aspect_ratio {
            if (self.width / height) != self.aspect_ratio().ratio() {
                self.width = height * self.aspect_ratio().ratio()
            }
        } else {
            self.aspect_ratio = AspectRatioMode::Dynamic;
        }
        self.height = height;

        self
    }

    pub fn change_width(mut self, width: f32, maintain_aspect_ratio: bool) -> Self {
        if maintain_aspect_ratio {
            if (width / self.height) != self.aspect_ratio().ratio() {
                self.height = self.width / self.aspect_ratio().ratio()
            }
        } else {
            self.aspect_ratio = AspectRatioMode::Dynamic;
        }

        self.width = width;

        self
    }

    pub fn change_ratio(mut self, ratio: AspectRatio) -> Self {
        self.aspect_ratio = AspectRatioMode::Set(ratio);
        self.width = self.height * ratio.ratio();

        self
    }

    pub fn can_fit(&self, aspect_ratio: &AspectRatio) -> bool {
        resolution_fits_aspect_ratio(self, aspect_ratio)
    }

    pub fn scale_and_keep_aspect_ratio(self, scalar: Vec2) -> Option<Self> {
        if (self.width * scalar.x) / (self.height * scalar.y) != self.aspect_ratio().ratio() {
            return None;
        }

        Some(Self {
            width: self.width * scalar.x,
            height: self.height * scalar.y,
            aspect_ratio: self.aspect_ratio,
        })
    }

    pub fn scale(self, scalar: Vec2) -> Self {
        let ratio = (self.width * scalar.x) / (self.height * scalar.y);

        if self.aspect_ratio.is_dynamic() || self.aspect_ratio().ratio() == ratio {
            Self {
                width: self.width * scalar.x,
                height: self.height * scalar.y,
                ..self
            }
        } else {
            Self {
                width: self.width * scalar.x,
                height: self.height * scalar.y,
                aspect_ratio: AspectRatioMode::Dynamic,
            }
        }
    }
}

pub fn has_integer_scale(to: &Resolution, from: &Resolution) -> bool {
    if from.aspect_ratio().ratio() != to.aspect_ratio().ratio() {
        return false;
    }

    if from.height < to.height {
        (to.height / from.height) % 1. == 0.
    } else {
        (from.height / to.height) % 1. == 0.
    }
}
pub fn get_scale_factor(to: &Resolution, from: &Resolution) -> Vec2 {
    if from.aspect_ratio().ratio() != to.aspect_ratio().ratio() {
        Vec2::new(from.width / to.width, from.height / to.height)
    } else {
        Vec2::splat(from.height / to.height)
    }
}
pub fn resolution_fits_aspect_ratio(resolution: &Resolution, aspect_ratio: &AspectRatio) -> bool {
    fits_aspect_ratio(resolution.height, aspect_ratio)
}

pub fn fits_aspect_ratio(height: f32, aspect_ratio: &AspectRatio) -> bool {
    (height * aspect_ratio.ratio()) % 1. == 0.
}

pub fn r360p(aspect_ratio: AspectRatio) -> Resolution {
    Resolution::from_height(360., aspect_ratio)
}
pub fn r480p(aspect_ratio: AspectRatio) -> Resolution {
    Resolution::from_height(480., aspect_ratio)
}
pub fn r720p(aspect_ratio: AspectRatio) -> Resolution {
    Resolution::from_height(720., aspect_ratio)
}
pub fn r1080p(aspect_ratio: AspectRatio) -> Resolution {
    Resolution::from_height(1080., aspect_ratio)
}
pub fn r1440p(aspect_ratio: AspectRatio) -> Resolution {
    Resolution::from_height(1440., aspect_ratio)
}

impl From<Resolution> for Vec2 {
    fn from(value: Resolution) -> Self {
        Vec2::new(value.width, value.height)
    }
}
impl From<Resolution> for UVec2 {
    fn from(value: Resolution) -> Self {
        UVec2::new(value.width.ceil() as u32, value.height.ceil() as u32)
    }
}
impl From<Resolution> for AspectRatio {
    fn from(value: Resolution) -> Self {
        value.aspect_ratio()
    }
}

#[cfg(feature = "bevy_window")]
impl From<Resolution> for bevy_window::WindowResolution {
    fn from(value: Resolution) -> Self {
        bevy_window::WindowResolution::from(Vec2::from(value))
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = UVec2::from(*self);
        write!(f, "{} x {}", res.x, res.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_fit() {
        assert!(fits_aspect_ratio(360., &AspectRatio::FOUR_THREE));
        assert!(fits_aspect_ratio(360., &AspectRatio::SIXTEEN_NINE));
        assert!(!fits_aspect_ratio(480., &AspectRatio::SIXTEEN_NINE));
        assert!(fits_aspect_ratio(480., &AspectRatio::FOUR_THREE));
    }

    #[test]
    fn can_integer_scale() {
        assert!(!has_integer_scale(
            &r360p(AspectRatio::FOUR_THREE),
            &r480p(AspectRatio::FOUR_THREE)
        ));
        assert!(has_integer_scale(
            &r360p(AspectRatio::SIXTEEN_NINE),
            &r720p(AspectRatio::SIXTEEN_NINE)
        ));
        assert!(!has_integer_scale(
            &r360p(AspectRatio::SIXTEEN_NINE),
            &r720p(AspectRatio::FOUR_THREE)
        ));
    }

    #[test]
    fn get_scale() {
        assert_eq!(
            get_scale_factor(
                &r360p(AspectRatio::SIXTEEN_NINE),
                &r720p(AspectRatio::SIXTEEN_NINE)
            ),
            Vec2::splat(2.)
        );
        assert_eq!(
            get_scale_factor(
                &r720p(AspectRatio::SIXTEEN_NINE),
                &r360p(AspectRatio::SIXTEEN_NINE)
            ),
            Vec2::splat(0.5)
        );
    }

    #[test]
    fn resolution_uvec2() {
        let r360_1 = r360p(AspectRatio::SIXTEEN_NINE).into();
        let r360_2 = r360p(AspectRatio::FOUR_THREE).into();
        let r480_1 = r480p(AspectRatio::SIXTEEN_NINE).into();
        let r480_2 = r480p(AspectRatio::FOUR_THREE).into();
        let r720_1 = r720p(AspectRatio::SIXTEEN_NINE).into();
        let r720_2 = r720p(AspectRatio::FOUR_THREE).into();
        let r1080_1 = r1080p(AspectRatio::SIXTEEN_NINE).into();
        let r1080_2 = r1080p(AspectRatio::FOUR_THREE).into();
        let r1440_1 = r1440p(AspectRatio::SIXTEEN_NINE).into();
        let r1440_2 = r1440p(AspectRatio::FOUR_THREE).into();
        let r_custom = Resolution::from_height(240., AspectRatio::SIXTEEN_NINE).into();

        assert_eq!(UVec2::new(427, 240), r_custom);

        assert_eq!(UVec2::new(1920, 1440), r1440_2);
        assert_eq!(UVec2::new(2560, 1440), r1440_1);

        assert_eq!(UVec2::new(1920, 1080), r1080_1);
        assert_eq!(UVec2::new(1440, 1080), r1080_2);

        assert_eq!(UVec2::new(1280, 720), r720_1);
        assert_eq!(UVec2::new(960, 720), r720_2);

        assert_eq!(UVec2::new(854, 480), r480_1);
        assert_eq!(UVec2::new(640, 480), r480_2);

        assert_eq!(UVec2::new(640, 360), r360_1);
        assert_eq!(UVec2::new(480, 360), r360_2);
    }

    #[test]
    fn resolution_vec2() {
        let r360_1 = r360p(AspectRatio::SIXTEEN_NINE).into();
        let r360_2 = r360p(AspectRatio::FOUR_THREE).into();

        let r480_1 = r480p(AspectRatio::SIXTEEN_NINE).into();
        let r480_2 = r480p(AspectRatio::FOUR_THREE).into();

        let r720_1 = r720p(AspectRatio::SIXTEEN_NINE).into();
        let r720_2 = r720p(AspectRatio::FOUR_THREE).into();

        let r1080_1 = r1080p(AspectRatio::SIXTEEN_NINE).into();
        let r1080_2 = r1080p(AspectRatio::FOUR_THREE).into();

        let r1440_1 = r1440p(AspectRatio::SIXTEEN_NINE).into();
        let r1440_2 = r1440p(AspectRatio::FOUR_THREE).into();

        let r_custom = Resolution::from_height(240., AspectRatio::SIXTEEN_NINE).into();

        assert_eq!(Vec2::new(426.66666, 240.), r_custom);

        assert_eq!(Vec2::new(1920., 1440.), r1440_2);
        assert_eq!(Vec2::new(2560., 1440.), r1440_1);

        assert_eq!(Vec2::new(1920., 1080.), r1080_1);
        assert_eq!(Vec2::new(1440., 1080.), r1080_2);

        assert_eq!(Vec2::new(1280., 720.), r720_1);
        assert_eq!(Vec2::new(960., 720.), r720_2);

        assert_eq!(Vec2::new(853.3333, 480.), r480_1);
        assert_eq!(Vec2::new(640., 480.), r480_2);

        assert_eq!(Vec2::new(640., 360.), r360_1);
        assert_eq!(Vec2::new(480., 360.), r360_2);
    }

    #[test]
    fn aspect_ratio() {
        let r360 = r360p(AspectRatio::SIXTEEN_NINE).into();

        let r480 = r480p(AspectRatio::FOUR_THREE).into();

        let r720 = r720p(AspectRatio::SIXTEEN_NINE).into();

        let r1080 = r1080p(AspectRatio::SIXTEEN_NINE).into();

        let r1440 = r1440p(AspectRatio::SIXTEEN_NINE).into();

        let r_custom = Resolution::from_height(240., AspectRatio::SIXTEEN_NINE).aspect_ratio();

        assert_eq!(AspectRatio::SIXTEEN_NINE, r_custom);

        assert_eq!(AspectRatio::SIXTEEN_NINE, r1440);

        assert_eq!(AspectRatio::SIXTEEN_NINE, r1080);

        assert_eq!(AspectRatio::SIXTEEN_NINE, r720);

        assert_eq!(AspectRatio::FOUR_THREE, r480);

        assert_eq!(AspectRatio::SIXTEEN_NINE, r360);
    }

    #[test]
    fn scale() {
        let r360 = r360p(AspectRatio::SIXTEEN_NINE);

        assert_eq!(
            Vec2::from(r360.scale(Vec2::new(1., 2.))),
            Vec2::new(640., 720.)
        );
        assert_eq!(
            r360.scale(Vec2::splat(2.)),
            r720p(AspectRatio::SIXTEEN_NINE)
        );

        assert_eq!(r360.scale_and_keep_aspect_ratio(Vec2::new(1., 2.)), None);
        assert_eq!(
            r360.scale_and_keep_aspect_ratio(Vec2::splat(2.)),
            Some(r720p(AspectRatio::SIXTEEN_NINE))
        );
    }
    
    #[test]
    fn changes() {
        let r360 = r360p(AspectRatio::SIXTEEN_NINE);
        
        assert_eq!(r360.change_height(720., false).width(), 640.);
        assert_eq!(r360.change_height(720., true).width(), 1280.);
        assert_eq!(r360.change_width(1280., true).height(), 720.);
        assert_eq!(r360.change_width(1280., false).height(), 360.);
        assert_eq!(UVec2::from(r360.change_ratio(AspectRatio::FOUR_THREE)), UVec2::new(480, 360));
        assert_eq!(UVec2::from(r360.change_ratio(AspectRatio::ULTRAWIDE)), UVec2::new(840, 360));
    }

    #[cfg(feature = "bevy_window")]
    #[test]
    fn resolution_to_window() {
        let r360_1 = r360p(AspectRatio::SIXTEEN_NINE).into();
        let r360_2 = r360p(AspectRatio::FOUR_THREE).into();

        assert_eq!(bevy_window::WindowResolution::new(640., 360.), r360_1);
        assert_eq!(bevy_window::WindowResolution::new(480., 360.), r360_2);
    }
}
