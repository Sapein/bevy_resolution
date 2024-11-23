use bevy_math::{AspectRatio, Vec2};
use bevy_resolution::resolutions;
use bevy_resolution::resolutions::{r360p, r480p, r720p};

fn main() {
    let res_1 = r360p(AspectRatio::SIXTEEN_NINE);
    let res_2 = r480p(AspectRatio::SIXTEEN_NINE);
    let res_3 = r720p(AspectRatio::SIXTEEN_NINE);
    let res_4 = r720p(AspectRatio::FOUR_THREE);

    println!(
        "360p has integer scale with 480p? {}",
        res_1.has_integer_scale(&res_2)
    );
    println!(
        "360p has integer scale with 720p? {}",
        res_1.has_integer_scale(&res_3)
    );
    println!(
        "360p has integer scale to 4:3 720p? {}",
        res_1.has_integer_scale(&res_4)
    );

    println!("How to scale 360 to 720? {}", res_1.scale_factor(&res_3));
    println!("How to scale 720 to 360? {}", res_3.scale_factor(&res_1));
    println!(
        "How to scale 360 to 4:3 720? {}",
        res_1.scale_factor(&res_4)
    );
    println!(
        "How to scale 4:3 720 to 360? {}",
        res_4.scale_factor(&res_1)
    );

    println!("360p scaled 3x: {}", res_1.scale(Vec2::splat(3.)));
    println!(
        "360p scaled 2x and maintain aspect ratio: {}",
        res_1.scale_and_keep_aspect_ratio(Vec2::splat(2.)).unwrap()
    );
    println!(
        "360p scaled by 1 width and 2 height: {}",
        res_1.scale(Vec2::new(1., 2.))
    );
    println!(
        "360p scaled by 1 width and 2 height with aspect ratio: {:?}",
        res_1.scale_and_keep_aspect_ratio(Vec2::new(1., 2.))
    );
    println!(
        "480p scalar to get to 4:3 720p: {}",
        res_2.scale_factor(&res_4)
    );

    println!(
        "Is 4:3 480p an exact resolution: {}",
        resolutions::fits_aspect_ratio(480., &AspectRatio::FOUR_THREE)
    );
    println!(
        "Is 16:9 480p an exact resolution: {}",
        resolutions::fits_aspect_ratio(480., &AspectRatio::SIXTEEN_NINE)
    );
    println!(
        "Is 16:9 480p an exact resolution: {}",
        resolutions::resolution_fits_aspect_ratio(
            &resolutions::Resolution::new(1., 480.),
            &AspectRatio::SIXTEEN_NINE
        )
    );
}
