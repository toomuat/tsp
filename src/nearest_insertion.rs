use crate::common::{distance, replot, total_distance};
use std::fs::File;
use std::io::Write;

pub fn solver(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    nearest_insertion_internal(gp, cities)
}

pub fn two_opt(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut visit_cities = nearest_insertion_internal(gp, cities);
    // In nearest_insertion_internal, start city is pushed at tail to make circle so remove it.
    visit_cities.pop();

    crate::two_opt::solver(gp, &mut visit_cities)
}

fn nearest_insertion_internal(
    gp: &mut std::process::Child,
    cities: &mut Vec<(f32, f32)>,
) -> Vec<(f32, f32)> {
    if cfg!(feature = "plot") {
        let mut file = File::create("cities.txt").expect("Unable to create file");
        for city in cities.iter() {
            let line = format!("{} {}\n", city.0, city.1,);
            file.write_all(line.as_bytes())
                .expect("Unable to write data");
        }
    }

    let mut all_cities = cities.clone();
    let start_city = all_cities[0];
    let mut visit_cities = vec![];
    for _ in 0..3 {
        visit_cities.push((all_cities[0].0, all_cities[0].1));
        all_cities.remove(0);
    }
    // Add start city to make cycle
    visit_cities.push(start_city);

    // Loop over all city of current optimal path and check the distance with all the other city not included in optimal path and insert the nearest city to optimal path
    while !all_cities.is_empty() {
        let mut min_dist = i32::MAX;
        let mut insert_idx = 0;
        let mut city_idx = 0;

        // Serch nearest city
        for (i, visit_city) in visit_cities.iter().enumerate() {
            for (j, city) in all_cities.iter().enumerate() {
                let dist = distance(*visit_city, *city);

                if dist < min_dist {
                    min_dist = dist;
                    insert_idx = i + 1;
                    city_idx = j;
                }
            }
        }

        //  Insert nearest city to cities in current optimal path
        visit_cities.insert(insert_idx, all_cities[city_idx]);
        all_cities.remove(city_idx);

        // Plot all cities in points and current optimal path in lines
        #[cfg(feature = "plot")]
        crate::common::plot(gp, cities, &visit_cities);
    }

    visit_cities
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
    fn all() {
        test_tsp!(solver, "nearest_insertion", true, TSP_FILE_BERLIN52);
        test_tsp!(solver, "nearest_insertion", true, TSP_FILE_KROC100);
        test_tsp!(solver, "nearest_insertion", true, TSP_FILE_TS225);
    }

    #[test]
    fn plot() {
        test_tsp!(solver, "nearest_insertion", false, TSP_FILE_BERLIN52);
    }

    // Debug mode is slow so 2 opt tests are recommended to run in release mode

    #[test]
    fn twoopt_berlin() {
        test_tsp!(
            two_opt,
            "nearest_insertion_twoopt",
            false,
            TSP_FILE_BERLIN52
        );
    }

    #[test]
    fn twoopt_kroc() {
        test_tsp!(two_opt, "nearest_insertion_twoopt", false, TSP_FILE_KROC100);
    }

    #[test]
    fn twoopt_ts() {
        test_tsp!(two_opt, "nearest_insertion_twoopt", false, TSP_FILE_TS225);
    }

    #[test]
    fn twoopt_all() {
        test_tsp!(
            two_opt,
            "nearest_insertion_twoopt",
            false,
            TSP_FILE_BERLIN52
        );
        test_tsp!(two_opt, "nearest_insertion_twoopt", false, TSP_FILE_KROC100);
        test_tsp!(two_opt, "nearest_insertion_twoopt", false, TSP_FILE_TS225);
    }

    #[test]
    fn twoopt_gif_all() {
        test_tsp!(two_opt, "nearest_insertion_twoopt", true, TSP_FILE_BERLIN52);
        test_tsp!(two_opt, "nearest_insertion_twoopt", true, TSP_FILE_KROC100);
        test_tsp!(two_opt, "nearest_insertion_twoopt", true, TSP_FILE_TS225);
    }

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
