use crate::common::{distance, replot, total_distance};
use std::io::Write;

pub fn solver(
    gp: &mut std::process::Child,
    cities: &mut Vec<(f32, f32)>,
) -> (Vec<(f32, f32)>, Vec<usize>) {
    nearest_neighbor_internal(gp, cities)
}

pub fn two_opt(
    gp: &mut std::process::Child,
    cities: &mut Vec<(f32, f32)>,
) -> (Vec<(f32, f32)>, Vec<usize>) {
    let (mut visit_cities, mut cities_idx) = nearest_neighbor_internal(gp, cities);
    // In nearest_insertion_internal, start city is pushed at tail to make circle so remove it.
    visit_cities.pop();

    crate::two_opt::solver(gp, &mut visit_cities, &mut cities_idx)
}

fn nearest_neighbor_internal(
    gp: &mut std::process::Child,
    cities: &mut Vec<(f32, f32)>,
) -> (Vec<(f32, f32)>, Vec<usize>) {
    let mut cities_idx = vec![];
    let mut visit_cities: Vec<(f32, f32)> = vec![];

    // Index of cities and x, y coordinate
    // Index is for identify which pass do we follow by index of cities
    let mut all_cities = cities
        .clone()
        .iter()
        .enumerate()
        .map(|i| (i.0, i.1 .0, i.1 .1))
        .collect::<Vec<(usize, f32, f32)>>();

    let start_city = all_cities.remove(0);
    cities_idx.push(start_city.0);
    visit_cities.push((start_city.1, start_city.2));

    let mut current_city = (start_city.1, start_city.2);

    while !all_cities.is_empty() {
        let mut min_dist = i32::MAX;

        // Get index of cities indicating which city to visit next
        // Find nearest city which current city and all the other city
        // not included in visit_cities
        let city_idx = all_cities.iter().enumerate().fold(0, |idx, city| {
            let d = distance(current_city, (city.1 .1, city.1 .2));
            // If calculated distance is lower than current recoreded
            // minimum distance, then update minimum distance and return
            // that city index
            if d < min_dist {
                min_dist = d;
                return city.0;
            }
            idx
        });

        let city = all_cities.remove(city_idx);
        cities_idx.push(city.0);
        visit_cities.push((city.1, city.2));
        current_city = (city.1, city.2);

        #[cfg(feature = "plot")]
        crate::common::plot(gp, cities, &visit_cities);
    }

    // Connect start and end city to make cycle
    visit_cities.push((start_city.1, start_city.2));

    #[cfg(feature = "plot")]
    crate::common::plot(gp, cities, &visit_cities);

    (visit_cities, cities_idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bench_tsp,
        common::{
            load_cities, save_image, setup_gnuplot, TSP_FILE_BERLIN52, TSP_FILE_KROC100,
            TSP_FILE_TS225,
        },
        test_tsp,
    };
    use test::Bencher;

    // Gnuplot window cannot be seem with gif enabled
    #[test]
    fn gif() {
        test_tsp!(solver, "nearest_neighbor", true, TSP_FILE_BERLIN52);
        test_tsp!(solver, "nearest_neighbor", true, TSP_FILE_KROC100);
        test_tsp!(solver, "nearest_neighbor", true, TSP_FILE_TS225);
    }

    #[test]
    fn all() {
        test_tsp!(solver, "nearest_neighbor", false, TSP_FILE_BERLIN52);
        test_tsp!(solver, "nearest_neighbor", false, TSP_FILE_KROC100);
        test_tsp!(solver, "nearest_neighbor", false, TSP_FILE_TS225);
    }

    // To show Gnuplot window, we need to enable plot feature
    // `cargo test --features plot nearest_neighbor::tests::plot -- --nocapture`
    #[test]
    fn berlin() {
        test_tsp!(solver, "nearest_neighbor", false, TSP_FILE_BERLIN52);
    }

    // Debug mode is slow so 2 opt tests are recommended to run in release mode

    #[test]
    fn twoopt_berlin() {
        test_tsp!(two_opt, "nearest_neighbor_twoopt", false, TSP_FILE_BERLIN52);
    }

    #[test]
    fn twoopt_kroc() {
        test_tsp!(two_opt, "nearest_neighbor_twoopt", false, TSP_FILE_KROC100);
    }

    #[test]
    fn twoopt_ts() {
        test_tsp!(two_opt, "nearest_neighbor_twoopt", false, TSP_FILE_TS225);
    }

    #[test]
    fn twoopt_all() {
        test_tsp!(two_opt, "nearest_neighbor_twoopt", false, TSP_FILE_BERLIN52);
        test_tsp!(two_opt, "nearest_neighbor_twoopt", false, TSP_FILE_KROC100);
        test_tsp!(two_opt, "nearest_neighbor_twoopt", false, TSP_FILE_TS225);
    }

    #[test]
    fn twoopt_gif_all() {
        test_tsp!(two_opt, "nearest_neighbor_twoopt", true, TSP_FILE_BERLIN52);
        test_tsp!(two_opt, "nearest_neighbor_twoopt", true, TSP_FILE_KROC100);
        test_tsp!(two_opt, "nearest_neighbor_twoopt", true, TSP_FILE_TS225);
    }

    // Executed 301 times
    // Executed 301 times
    #[bench]
    fn bench_berlin(b: &mut Bencher) {
        bench_tsp!(b, solver, TSP_FILE_BERLIN52);
    }

    #[bench]
    fn bench_kroc(b: &mut Bencher) {
        bench_tsp!(b, solver, TSP_FILE_KROC100);
    }

    #[bench]
    fn bench_ts(b: &mut Bencher) {
        bench_tsp!(b, solver, TSP_FILE_TS225);
    }

    #[bench]
    fn bench_twoopt_berlin(b: &mut Bencher) {
        bench_tsp!(b, two_opt, TSP_FILE_BERLIN52);
    }

    #[bench]
    fn bench_twoopt_kroc(b: &mut Bencher) {
        bench_tsp!(b, two_opt, TSP_FILE_KROC100);
    }

    #[bench]
    fn bench_twoopt_ts(b: &mut Bencher) {
        bench_tsp!(b, two_opt, TSP_FILE_TS225);
    }
}
