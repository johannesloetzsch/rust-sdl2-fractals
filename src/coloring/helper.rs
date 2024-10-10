pub fn bound(n: f32, min: f32, max: f32) -> f32 {
    let mut vals = [min, n, max];
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    vals[1]
}

pub fn norm(n: f32, min: f32, max: f32) -> f32 {
    let n_bound = bound(n, min, max);
    (n_bound-min) / (max-min)
}

pub fn norm_u8(n: f32, min: f32, max: f32) -> u8 {
    (255.0 * norm(n, min, max)) as u8
}
