pub fn swap_cities(cities: &mut Vec<(f32, f32)>, i: usize, j: usize) {
    let tmp = cities[i];
    cities[i] = cities[j];
    cities[j] = tmp;
}

pub fn distance(v1: (f32, f32), v2: (f32, f32)) -> f32 {
    ((v1.0 - v2.0).powf(2.0) + (v1.1 - v2.1).powf(2.0)).sqrt()
}

pub fn total_distance(cities: Vec<(f32, f32)>) -> f32 {
    (0..cities.len() - 1).fold(0.0, |sum, i| sum + distance(cities[i], cities[i + 1]))
}
