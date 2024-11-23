use std::fmt::{Display, Formatter};
use bevy_math::{AspectRatio, UVec2, Vec2};
use crate::resolutions::Resolution;

/// These represent Common Resolutions without needing to figure out the Aspect Ratio, these are all
/// assumed to be 16:9.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CommonResolutions {
    R360p,
    R720p,
    R1080p,
    R1440p,
}

impl CommonResolutions {
    /// Iterates through the pre-defined [`CommonResolutions`] variants
    pub fn iter() -> impl Iterator<Item = CommonResolutions> {
        [
            CommonResolutions::R360p,
            CommonResolutions::R720p,
            CommonResolutions::R1080p,
            CommonResolutions::R1440p,
        ].into_iter()
    }
}

impl From<CommonResolutions> for Resolution {
    fn from(value: CommonResolutions) -> Self {
        match value {
            CommonResolutions::R360p => Resolution::R360p(AspectRatio::SIXTEEN_NINE),
            CommonResolutions::R720p => Resolution::R720p(AspectRatio::SIXTEEN_NINE),
            CommonResolutions::R1080p => Resolution::R1080p(AspectRatio::SIXTEEN_NINE),
            CommonResolutions::R1440p => Resolution::R1440p(AspectRatio::SIXTEEN_NINE),
        }
    }
}

#[cfg(feature="bevy_window")]
impl From<CommonResolutions> for bevy_window::WindowResolution {
    fn from(value: CommonResolutions) -> Self {
        bevy_window::WindowResolution::from(Vec2::from(value))
    }
}

impl From<CommonResolutions> for UVec2 {
    fn from(value: CommonResolutions) -> Self {
        UVec2::from(Resolution::from(value))
    }
}

impl From<CommonResolutions> for Vec2 {
    fn from(value: CommonResolutions) -> Self {
        Vec2::from(Resolution::from(value))
    }
}

impl Display for CommonResolutions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = UVec2::from(Resolution::from(*self));
        write!(f, "{} x {}", res.x, res.y)
    }
}

/// These provide additional Resolutions that, while common, are in 4:3.
pub mod common4x3 {
    use std::fmt::{Display, Formatter};
    use bevy_math::{AspectRatio, UVec2, Vec2};
    use crate::resolutions::Resolution;

    /// Provides an alternative CommonResolutions that are 4:3 instead of 16:9
    ///
    /// 1080 is not provided because there technically 1080 in 4:3 does not exist/is not defined.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum CommonResolutions {
        R360p,
        R480p,
        R720p,
        R1440p,
    }

    impl From<CommonResolutions> for Resolution {
        fn from(value: CommonResolutions) -> Self {
            match value {
                CommonResolutions::R360p => Resolution::R360p(AspectRatio::FOUR_THREE),
                CommonResolutions::R480p => Resolution::R480p(AspectRatio::FOUR_THREE),
                CommonResolutions::R720p => Resolution::R720p(AspectRatio::FOUR_THREE),
                CommonResolutions::R1440p => Resolution::R1440p(AspectRatio::FOUR_THREE),
            }
        }
    }

    impl CommonResolutions {
        /// Iterates through the pre-defined [`Resolutions`] variants
        pub fn iter() -> impl Iterator<Item = CommonResolutions> {
            [
                CommonResolutions::R360p,
                CommonResolutions::R480p,
                CommonResolutions::R720p,
                CommonResolutions::R1440p,
            ].into_iter()
        }
    }

    impl Display for CommonResolutions {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let res = UVec2::from(Resolution::from(*self)) ;
            write!(f, "{} x {}", res.x, res.y)
        }
    }

    impl From<CommonResolutions> for UVec2 {
        fn from(value: CommonResolutions) -> Self {
            UVec2::from(Resolution::from(value))
        }
    }


    impl From<CommonResolutions> for Vec2 {
        fn from(value: CommonResolutions) -> Self {
            Vec2::from(Resolution::from(value))
        }
    }

    #[cfg(feature="bevy_window")]
    impl From<CommonResolutions> for bevy_window::WindowResolution {
        fn from(value: CommonResolutions) -> Self {
            bevy_window::WindowResolution::from(Vec2::from(value))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn resolution_uvec2() {
            let r360 = CommonResolutions::R360p.into();
            let r480 = CommonResolutions::R480p.into();
            let r720 = CommonResolutions::R720p.into();
            let r1440 = CommonResolutions::R1440p.into();

            assert_eq!(UVec2::new(1920, 1440), r1440);
            assert_eq!(UVec2::new(960, 720), r720);
            assert_eq!(UVec2::new(640, 480), r480);
            assert_eq!(UVec2::new(480, 360), r360);
        }

        #[test]
        fn resolution_vec2() {
            let r360 = CommonResolutions::R360p.into();
            let r480 = CommonResolutions::R480p.into();
            let r720 = CommonResolutions::R720p.into();
            let r1440 = CommonResolutions::R1440p.into();

            assert_eq!(Vec2::new(1920., 1440.), r1440);
            assert_eq!(Vec2::new(960., 720.), r720);
            assert_eq!(Vec2::new(640., 480.), r480);
            assert_eq!(Vec2::new(480., 360.), r360);
        }

        #[test]
        fn iter() {
            let iter = CommonResolutions::iter().collect::<Vec<CommonResolutions>>();

            assert_eq!(iter[0], CommonResolutions::R360p.into());
            assert_eq!(iter[1], CommonResolutions::R480p.into());
            assert_eq!(iter[2], CommonResolutions::R720p.into());
            assert_eq!(iter[3], CommonResolutions::R1440p.into());
        }

        #[cfg(feature="bevy_window")]
        #[test]
        fn resolution_to_window() {
            let r360 = CommonResolutions::R360p.into();

            assert_eq!(bevy_window::WindowResolution::new(480., 360.), r360);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_uvec2() {
        let r360 = CommonResolutions::R360p.into();
        let r720 = CommonResolutions::R720p.into();
        let r1080 = CommonResolutions::R1080p.into();
        let r1440 = CommonResolutions::R1440p.into();

        assert_eq!(UVec2::new(2560, 1440), r1440);

        assert_eq!(UVec2::new(1920, 1080), r1080);

        assert_eq!(UVec2::new(1280, 720), r720);

        assert_eq!(UVec2::new(640, 360), r360);
    }

    #[test]
    fn resolution_vec2() {
        let r360 = CommonResolutions::R360p.into();
        let r720 = CommonResolutions::R720p.into();
        let r1080 = CommonResolutions::R1080p.into();
        let r1440 = CommonResolutions::R1440p.into();

        assert_eq!(Vec2::new(2560., 1440.), r1440);
        assert_eq!(Vec2::new(1920., 1080.), r1080);
        assert_eq!(Vec2::new(1280., 720.), r720);
        assert_eq!(Vec2::new(640., 360.), r360);
    }

    #[test]
    fn iter() {
        let iter = CommonResolutions::iter().collect::<Vec<CommonResolutions>>();

        assert_eq!(iter[0], CommonResolutions::R360p.into());
        assert_eq!(iter[1], CommonResolutions::R720p.into());
        assert_eq!(iter[2], CommonResolutions::R1080p.into());
        assert_eq!(iter[3], CommonResolutions::R1440p.into());
    }

    #[cfg(feature="bevy_window")]
    #[test]
    fn resolution_to_window() {
        let r360 = CommonResolutions::R360p.into();

        assert_eq!(bevy_window::WindowResolution::new(640., 360.), r360);
    }
}
