use std::fmt::{Display, Formatter};
use bevy_math::{AspectRatio, UVec2, Vec2};

/// Represents abstract resolutions for a given aspect ratio.
///
///
/// Custom provides support for resolutions that aren't listed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolution {
    R360p(AspectRatio),
    R480p(AspectRatio),
    R720p(AspectRatio),
    R1080p(AspectRatio),
    R1440p(AspectRatio),

    Custom {
        height: f32,
        aspect_ratio: AspectRatio,
    }
}

impl From<Resolution> for Vec2 {
    fn from(value: Resolution) -> Self {
        match value {
            Resolution::R360p(ar) => Vec2::new(360. * ar.ratio(), 360.),
            Resolution::R480p(ar) => Vec2::new(480. * ar.ratio(), 480.),
            Resolution::R720p(ar) => Vec2::new(720. * ar.ratio(), 720.),
            Resolution::R1080p(ar) => Vec2::new(1080. * ar.ratio(), 1080.),
            Resolution::R1440p(ar) => Vec2::new(1440. * ar.ratio(), 1440.),
            Resolution::Custom { height, aspect_ratio } => Vec2::new(height * aspect_ratio.ratio(), height)
        }
    }
}

#[cfg(feature="bevy_window")]
impl From<Resolution> for bevy_window::WindowResolution {
    fn from(value: Resolution) -> Self {
        bevy_window::WindowResolution::from(Vec2::from(value))
    }
}

impl Resolution {

    /// Iterates through the pre-defined [`Resolution`] variants
    pub fn iter(aspect_ratio: AspectRatio) -> impl Iterator<Item = Resolution> {
        [
            Resolution::R360p(aspect_ratio),
            Resolution::R480p(aspect_ratio),
            Resolution::R720p(aspect_ratio),
            Resolution::R1080p(aspect_ratio),
            Resolution::R1440p(aspect_ratio),
        ].into_iter()
    }

    /// Get the Aspect Ratio of a given resolution
    pub fn aspect_ratio(&self) -> AspectRatio {
        use Resolution::*;
        match self {
            Custom { aspect_ratio, .. } => aspect_ratio.clone(),
            R360p(a) | R480p(a) | R720p(a) | R1080p(a) | R1440p(a) => a.clone(),
        }
    }

    pub fn height(&self) -> f32 {
        use Resolution::*;
        match self {
            Custom { height, .. } => height.clone(),
            R360p(_)  => 360.,
            R480p(_) => 480.,
            R720p(_) => 720.,
            R1080p(_) => 1080.,
            R1440p(_) => 1440.,
        }
    }

    pub fn width(&self) -> f32 {
        use Resolution::*;
        match self {
            Custom { height, aspect_ratio } => aspect_ratio.ratio() * height.clone(),
            R360p(a)  => a.ratio() * 360.,
            R480p(a) => a.ratio() * 480.,
            R720p(a) => a.ratio() * 720.,
            R1080p(a) => a.ratio() * 1080.,
            R1440p(a) => a.ratio() * 1440.,
        }

    }

    pub fn can_fit_aspect_ratio(height: f32, aspect_ratio: AspectRatio) -> bool {
        ((aspect_ratio.ratio() * height) % 1.) == 0.
    }

    pub fn fits_aspect_ratio(&self, aspect_ratio: AspectRatio) -> bool {
        Self::can_fit_aspect_ratio(self.height(), aspect_ratio)
    }

    pub fn has_integer_scale(from: Resolution, to:Resolution) -> bool {
        if from.aspect_ratio().ratio() != to.aspect_ratio().ratio() { return false }

        if from.height() > to.height()  {
            (from.height() / to.height()) % 1. == 0.
        } else {
            (to.height() / from.height()) % 1. == 0.
        }
    }

    pub fn can_integer_scale(self, resolution_b: Resolution) -> bool {
        Self::has_integer_scale(self, resolution_b)
    }

    pub fn get_scale(from: Resolution, to: Resolution) -> Vec2 {
        if from.aspect_ratio().ratio() != to.aspect_ratio().ratio() {
            Vec2::new(to.width() / from.width(), to.height() / from.height())
        } else {
            Vec2::splat(to.height() / from.height())
        }
    }

    pub fn scale(self, to: Resolution) -> Vec2 {
        Self::get_scale(self, to)
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = UVec2::from(*self);
        write!(f, "{} x {}", res.x, res.y)
    }
}

impl From<Resolution> for UVec2 {
    fn from(value: Resolution) -> Self {
        match value {
            Resolution::R360p(ar) => UVec2::new((360. * ar.ratio()).ceil() as u32, 360),
            Resolution::R480p(ar) => UVec2::new((480. * ar.ratio()).ceil() as u32, 480),
            Resolution::R720p(ar) => UVec2::new((720. * ar.ratio()).ceil() as u32, 720),
            Resolution::R1080p(ar) => UVec2::new((1080. * ar.ratio()).ceil() as u32, 1080),
            Resolution::R1440p(ar) => UVec2::new((1440. * ar.ratio()).ceil() as u32, 1440),
            Resolution::Custom { height, aspect_ratio } => UVec2::new((height * aspect_ratio.ratio()).ceil() as u32, height.ceil() as u32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_fit() {
        assert!(Resolution::can_fit_aspect_ratio(360., AspectRatio::SIXTEEN_NINE));
        assert!(Resolution::can_fit_aspect_ratio(360., AspectRatio::FOUR_THREE));
        assert!(!Resolution::can_fit_aspect_ratio(480., AspectRatio::SIXTEEN_NINE));
        assert!(Resolution::can_fit_aspect_ratio(480., AspectRatio::FOUR_THREE));
    }

    #[test]
    fn does_fits() {
        assert!(Resolution::R360p(AspectRatio::FOUR_THREE).fits_aspect_ratio(AspectRatio::FOUR_THREE));
        assert!(Resolution::R360p(AspectRatio::FOUR_THREE).fits_aspect_ratio(AspectRatio::SIXTEEN_NINE));
        assert!(!Resolution::R480p(AspectRatio::FOUR_THREE).fits_aspect_ratio(AspectRatio::SIXTEEN_NINE));
        assert!(Resolution::R480p(AspectRatio::FOUR_THREE).fits_aspect_ratio(AspectRatio::FOUR_THREE));
    }

    #[test]
    fn can_integer_scale() {
        assert!(!Resolution::has_integer_scale(Resolution::R360p(AspectRatio::FOUR_THREE), Resolution::R480p(AspectRatio::FOUR_THREE)));
        assert!(Resolution::has_integer_scale(Resolution::R360p(AspectRatio::SIXTEEN_NINE), Resolution::R720p(AspectRatio::SIXTEEN_NINE)));
        assert!(!Resolution::has_integer_scale(Resolution::R360p(AspectRatio::SIXTEEN_NINE), Resolution::R720p(AspectRatio::FOUR_THREE)));
    }

    #[test]
    fn get_scale() {
        assert_eq!(Resolution::get_scale(Resolution::R360p(AspectRatio::SIXTEEN_NINE), Resolution::R720p(AspectRatio::SIXTEEN_NINE)), Vec2::splat(2.));
        assert_eq!(Resolution::get_scale(Resolution::R720p(AspectRatio::SIXTEEN_NINE), Resolution::R360p(AspectRatio::SIXTEEN_NINE)), Vec2::splat(0.5));
    }

    #[test]
    fn scale() {
        assert_eq!(Resolution::R360p(AspectRatio::SIXTEEN_NINE).scale(Resolution::R720p(AspectRatio::SIXTEEN_NINE)), Vec2::splat(2.));
        assert_eq!(Resolution::R720p(AspectRatio::SIXTEEN_NINE).scale(Resolution::R360p(AspectRatio::SIXTEEN_NINE)), Vec2::splat(0.5));
    }


    #[test]
    fn resolution_uvec2() {
        let r360_1 = Resolution::R360p(AspectRatio::SIXTEEN_NINE).into();
        let r360_2 = Resolution::R360p(AspectRatio::FOUR_THREE).into();
        let r480_1 = Resolution::R480p(AspectRatio::SIXTEEN_NINE).into();
        let r480_2 = Resolution::R480p(AspectRatio::FOUR_THREE).into();
        let r720_1 = Resolution::R720p(AspectRatio::SIXTEEN_NINE).into();
        let r720_2 = Resolution::R720p(AspectRatio::FOUR_THREE).into();
        let r1080_1 = Resolution::R1080p(AspectRatio::SIXTEEN_NINE).into();
        let r1080_2 = Resolution::R1080p(AspectRatio::FOUR_THREE).into();
        let r1440_1 = Resolution::R1440p(AspectRatio::SIXTEEN_NINE).into();
        let r1440_2 = Resolution::R1440p(AspectRatio::FOUR_THREE).into();
        let r_custom = Resolution::Custom {
            height: 240.,
            aspect_ratio: AspectRatio::SIXTEEN_NINE,
        }.into();

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
        let r360_1 = Resolution::R360p(AspectRatio::SIXTEEN_NINE).into();
        let r360_2 = Resolution::R360p(AspectRatio::FOUR_THREE).into();

        let r480_1 = Resolution::R480p(AspectRatio::SIXTEEN_NINE).into();
        let r480_2 = Resolution::R480p(AspectRatio::FOUR_THREE).into();

        let r720_1 = Resolution::R720p(AspectRatio::SIXTEEN_NINE).into();
        let r720_2 = Resolution::R720p(AspectRatio::FOUR_THREE).into();

        let r1080_1 = Resolution::R1080p(AspectRatio::SIXTEEN_NINE).into();
        let r1080_2 = Resolution::R1080p(AspectRatio::FOUR_THREE).into();

        let r1440_1 = Resolution::R1440p(AspectRatio::SIXTEEN_NINE).into();
        let r1440_2 = Resolution::R1440p(AspectRatio::FOUR_THREE).into();

        let r_custom = Resolution::Custom {
            height: 240.,
            aspect_ratio: AspectRatio::SIXTEEN_NINE,
        }.into();

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
    fn iter() {
        let iter = Resolution::iter(AspectRatio::SIXTEEN_NINE).collect::<Vec<Resolution>>();

        assert_eq!(iter[0], Resolution::R360p(AspectRatio::SIXTEEN_NINE).into());
        assert_eq!(iter[1], Resolution::R480p(AspectRatio::SIXTEEN_NINE).into());
        assert_eq!(iter[2], Resolution::R720p(AspectRatio::SIXTEEN_NINE).into());
        assert_eq!(iter[3], Resolution::R1080p(AspectRatio::SIXTEEN_NINE).into());
        assert_eq!(iter[4], Resolution::R1440p(AspectRatio::SIXTEEN_NINE).into());
    }

    #[test]
    fn aspect_ratio() {

        let r360 = Resolution::R360p(AspectRatio::SIXTEEN_NINE).aspect_ratio();

        let r480 = Resolution::R480p(AspectRatio::FOUR_THREE).aspect_ratio();

        let r720 = Resolution::R720p(AspectRatio::SIXTEEN_NINE).aspect_ratio();

        let r1080 = Resolution::R1080p(AspectRatio::SIXTEEN_NINE).aspect_ratio();

        let r1440 = Resolution::R1440p(AspectRatio::SIXTEEN_NINE).aspect_ratio();

        let r_custom = Resolution::Custom {
            height: 240.,
            aspect_ratio: AspectRatio::SIXTEEN_NINE,
        }.aspect_ratio();

        assert_eq!(AspectRatio::SIXTEEN_NINE, r_custom);

        assert_eq!(AspectRatio::SIXTEEN_NINE, r1440);

        assert_eq!(AspectRatio::SIXTEEN_NINE, r1080);

        assert_eq!(AspectRatio::SIXTEEN_NINE, r720);

        assert_eq!(AspectRatio::FOUR_THREE, r480);

        assert_eq!(AspectRatio::SIXTEEN_NINE, r360);
    }

    #[cfg(feature="bevy_window")]
    #[test]
    fn resolution_to_window() {
        let r360_1 = Resolution::R360p(AspectRatio::SIXTEEN_NINE).into();
        let r360_2 = Resolution::R360p(AspectRatio::FOUR_THREE).into();

        assert_eq!(bevy_window::WindowResolution::new(640., 360.), r360_1);
        assert_eq!(bevy_window::WindowResolution::new(480., 360.), r360_2);
    }
}
