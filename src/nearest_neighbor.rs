use crate::common::{distance, total_distance};
use std::io::Write;

pub fn nearest_neighbor(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) {
    let mut optimal_path: Vec<(f32, f32)> = Vec::new();
    let mut all_cities = cities.clone();

    let start_city = cities.remove(0);
    optimal_path.push(start_city);
    let mut current_city = start_city;

    while !cities.is_empty() {
        let mut min_dist = i32::MAX;

        let city_idx = cities.iter().enumerate().fold(0, |idx, city| {
            let d = distance(current_city, *city.1);
            if d < min_dist {
                min_dist = d;
                return city.0;
            }
            idx
        });

        let city = cities.remove(city_idx);
        optimal_path.push(city);
        current_city = city;

        #[cfg(feature = "plot")]
        plot(gp, &mut all_cities, &mut optimal_path);
    }

    // Connect start and end city to make cycle
    optimal_path.push(start_city);

    #[cfg(feature = "plot")]
    plot(gp, &mut all_cities, &mut optimal_path);

    println!("Total distance: {}", total_distance(optimal_path));
}

fn plot(
    gp: &mut std::process::Child,
    cities: &mut Vec<(f32, f32)>,
    optimal_path: &mut Vec<(f32, f32)>,
) {
    let cmd = "plot '-' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
        '-' with line linewidth 5 linetype 1 linecolor rgb 'cyan'\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    // Plot all cities
    for city in cities.iter() {
        let cmd = format!("{} {}\n", city.0, city.1);
        let cmd: &str = &cmd;

        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }
    // End data input
    gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();

    // Plot optimal pass
    for city in optimal_path.iter() {
        let cmd = format!("{} {}\n", city.0, city.1);
        let cmd: &str = &cmd;

        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }

    // End data input
    gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();

    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{
        load_cities, save_image, setup_gnuplot, TSP_FILE_BERLIN52, TSP_FILE_KROC100, TSP_FILE_TS225,
    };

    fn test_tsp(enable_gif: bool, tsp_file: &str) {
        let file_name = "nearest_neighbor";
        let mut cities: Vec<(f32, f32)> = Vec::new();
        load_cities(&mut cities, tsp_file).unwrap();

        let tsp_name = tsp_file.split('.').collect::<Vec<&str>>()[0];
        let file_name = format!("{}_{}", file_name, tsp_name);

        let mut gp = setup_gnuplot(&mut cities, &file_name, enable_gif);

        nearest_neighbor(&mut gp, &mut cities);

        // Save final result of optimal pass as an image
        #[cfg(feature = "plot")]
        save_image(&mut gp, &file_name);
    }

    // Gnuplot window cannot be seem with gif enabled

    #[test]
    fn all() {
        test_tsp(true, TSP_FILE_BERLIN52);
        test_tsp(true, TSP_FILE_KROC100);
        test_tsp(true, TSP_FILE_TS225);
    }

    #[test]
    fn plot() {
        test_tsp(false, TSP_FILE_BERLIN52);
    }
}
