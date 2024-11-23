use bevy_math::AspectRatio;
use bevy_resolution::resolutions::r360p;

fn main() {
    let res = r360p(AspectRatio::SIXTEEN_NINE);

    println!(
        "Change 360p's height and maintain aspect_ratio: {}",
        res.change_height(720., true)
    );
    println!(
        "Change 360p's height and don't maintain aspect_ratio: {}",
        res.change_height(720., false)
    );
    println!(
        "Change 360p's width and maintain aspect ratio: {}",
        res.change_width(1280., true)
    );
    println!(
        "Change 360p's width and maintain aspect ratio: {}",
        res.change_width(1280., false)
    );
    println!(
        "Change 360p's aspect ratio to 4:3: {}",
        res.change_ratio(AspectRatio::FOUR_THREE)
    );
    println!(
        "Change 360p's aspect ratio to ultrawide {}",
        res.change_ratio(AspectRatio::ULTRAWIDE)
    );
}
