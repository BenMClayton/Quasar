pub fn fractal_noise(x: f32, y: f32, seed: u32, octaves: usize, scale: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0 / scale.max(1.0);
    let mut total = 0.0;

    for octave in 0..octaves {
        value += value_noise(
            x * frequency,
            y * frequency,
            seed ^ (octave as u32 * 0x9e37),
        ) * amplitude;
        total += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    value / total
}

pub fn ridged_noise(x: f32, y: f32, seed: u32, scale: f32) -> f32 {
    let n = fractal_noise(x, y, seed, 4, scale);
    1.0 - (n * 2.0 - 1.0).abs()
}

fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let tx = x - x.floor();
    let ty = y - y.floor();

    let a = hash_float(x0, y0, seed);
    let b = hash_float(x0 + 1, y0, seed);
    let c = hash_float(x0, y0 + 1, seed);
    let d = hash_float(x0 + 1, y0 + 1, seed);

    let u = smooth(tx);
    let v = smooth(ty);
    lerp(lerp(a, b, u), lerp(c, d, u), v)
}

pub fn hash_float(x: i32, y: i32, seed: u32) -> f32 {
    hash(x, y, seed) as f32 / u32::MAX as f32
}

pub fn hash(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = seed;
    h ^= (x as u32).wrapping_mul(0x8da6_b343);
    h ^= (y as u32).wrapping_mul(0xd816_3841);
    h ^= h >> 13;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^ (h >> 16)
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
