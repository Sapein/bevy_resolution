use bevy_math::AspectRatio;
use bevy_resolution::resolutions::{r360p, r720p, Resolution};
use bevy_window::WindowResolution;

fn main() {
    let supported_resolutions = [
        Resolution::from_height(280., AspectRatio::FOUR_THREE),
        r360p(AspectRatio::SIXTEEN_NINE),
        r360p(AspectRatio::FOUR_THREE),
        r720p(AspectRatio::SIXTEEN_NINE),
    ]
    .map(|r| WindowResolution::from(r));

    for resolutions in supported_resolutions {
        println!("{:?}", resolutions);
    }
}
