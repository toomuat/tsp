use crate::common::{distance, replot, total_distance};
use std::io::Write;

pub fn solver(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut visit_cities: Vec<(f32, f32)> = Vec::new();
    let mut all_cities = cities.clone();

    let start_city = all_cities.remove(0);
    visit_cities.push(start_city);
    let mut current_city = start_city;

    while !all_cities.is_empty() {
        let mut min_dist = i32::MAX;

        let city_idx = all_cities.iter().enumerate().fold(0, |idx, city| {
            let d = distance(current_city, *city.1);
            if d < min_dist {
                min_dist = d;
                return city.0;
            }
            idx
        });

        let city = all_cities.remove(city_idx);
        visit_cities.push(city);
        current_city = city;

        #[cfg(feature = "plot")]
        crate::common::plot(gp, cities, &visit_cities);
    }

    // Connect start and end city to make cycle
    visit_cities.push(start_city);

    #[cfg(feature = "plot")]
    crate::common::plot(gp, cities, &visit_cities);

    println!("Total distance: {}", total_distance(&visit_cities));

    visit_cities
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{
            load_cities, save_image, setup_gnuplot, TSP_FILE_BERLIN52, TSP_FILE_KROC100,
            TSP_FILE_TS225,
        },
        test_tsp,
    };

    // Gnuplot window cannot be seem with gif enabled

    #[test]
    fn all() {
        test_tsp!(solver, "nearest_neighbor", true, TSP_FILE_BERLIN52);
        test_tsp!(solver, "nearest_neighbor", true, TSP_FILE_KROC100);
        test_tsp!(solver, "nearest_neighbor", true, TSP_FILE_TS225);
    }

    #[test]
    fn plot() {
        test_tsp!(solver, "nearest_neighbor", false, TSP_FILE_BERLIN52);
    }
}
