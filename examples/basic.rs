use bevy_math::AspectRatio;
use bevy_resolution::common::common4x3::CommonResolutions;
use bevy_resolution::resolutions::{r360p, r720p, Resolution};

fn main() {
    let supported_resolutions = [
        Resolution::from_height(280., AspectRatio::FOUR_THREE),
        r360p(AspectRatio::SIXTEEN_NINE),
        r360p(AspectRatio::FOUR_THREE),
        r720p(AspectRatio::SIXTEEN_NINE),
    ];

    for resolutions in supported_resolutions {
        println!("{}", resolutions);
    }

    for common in CommonResolutions::iter() {
        println!("{}", common);
    }
}
