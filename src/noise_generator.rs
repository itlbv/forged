use noise::{Fbm, MultiFractal, Seedable, SuperSimplex};
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};

pub type Noise = NoiseMap;

pub fn fbm(width: usize, height: usize, granularity: usize) -> Noise {
    let noise = Fbm::new()
        .set_seed(0)
        .set_octaves(16);
    let mut noise_map = PlaneMapBuilder::new(&noise)
        .set_size(width, height)
        .set_x_bounds(0.0, granularity as f64)
        .set_y_bounds(0.0, granularity as f64)
        .build();
    normalize_noise_map(&mut noise_map);
    noise_map
}

fn normalize_noise_map(noise_map: &mut Noise) {
    let (width, height) = noise_map.size();

    let mut min = 0.0;
    let mut max = 0.0;
    for y in 0..height {
        for x in 0..width {
            if noise_map.get_value(x, y) < min { min = noise_map.get_value(x, y);}
            if noise_map.get_value(x, y) > max { max = noise_map.get_value(x, y);}
        }
    }
    let factor = 1.0 / (max - min);

    for y in 0..height {
        for x in 0..width {
            let v = noise_map.get_value(x, y);
            noise_map.set_value(x, y, (v - min) * factor);
        }
    }
}