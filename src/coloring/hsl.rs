use sdl2::pixels::Color;

/** like https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB_alternative **/
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let a = s * l.min(1.0-l);

    let f = |n: f32| {
        let k = (n + 12.0*h) % 12.0;
        let r = l - a * 1.0_f32.min(k-3.0).min(9.0-k).max(-1.0);
        (255.0 * r) as u8
    };

    Color::RGB(f(0.0), f(8.0), f(4.0))
}

pub fn hsl_to_cartesian(h: f32, s: f32, l: f32) -> Color {
    let a = s * l.min(1.0-l);

    let f = |n: f32| {
        let k = (n + 12.0*h) % 12.0;
        let r = l - a * 1.0_f32.min(k-2.0).min(10.0-k).max(-1.0);
        (127.0 * r) as u8
    };

    Color::RGB(f(6.0)+f(9.0), f(3.0)+f(9.0), f(0.0)+f(6.0), )
}
