use crate::common::{distance, total_distance};
use std::io::Write;

pub fn nearest_neighbor(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) {
    let start_city = cities[0];
    let mut current_city = start_city;
    let mut visit_cities: Vec<bool> = vec![false; cities.len()];
    let mut optimal_path: Vec<(f32, f32)> = Vec::new();
    let mut next_city: (f32, f32) = cities[1];
    let mut city_idx = 1;

    visit_cities[0] = true;
    optimal_path.push(start_city);

    for _i in 0..cities.len() - 1 {
        let mut min_dist = f32::MAX;

        for j in 1..cities.len() {
            let dist = distance(current_city, cities[j]);
            if dist < min_dist && !visit_cities[j] {
                next_city = cities[j];
                city_idx = j;
                min_dist = dist;
            }
        }

        optimal_path.push(next_city);
        current_city = next_city;
        visit_cities[city_idx] = true;

        nearest_neighbor_plot(gp, cities, &mut optimal_path);

        // std::thread::sleep(std::time::Duration::from_millis(200));
    }

    println!("Total distance: {}", total_distance(optimal_path));
}

fn nearest_neighbor_plot(
    gp: &mut std::process::Child,
    cities: &mut Vec<(f32, f32)>,
    optimal_path: &mut Vec<(f32, f32)>,
) {
    let commands: Vec<&str> = vec![
        "plot '-' with point pointtype 7 pointsize 2 linecolor rgb 'black',",
        "'-' with line linewidth 5 linetype 1 linecolor rgb 'cyan'\n",
    ];
    for cmd in commands.iter() {
        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }

    // Plot all cities
    for j in 0..cities.len() {
        let cmd = format!("{} {}\n", cities[j].0, cities[j].1);
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
    for j in 0..optimal_path.len() {
        let cmd = format!("{} {}\n", optimal_path[j].0, optimal_path[j].1);
        let cmd: &str = &cmd;

        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }

    if optimal_path.len() == cities.len() {
        let cmd = format!("{} {}\n", cities[0].0, cities[0].1);
        let cmd: &str = &cmd;
        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }

    // End data input
    gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{load_cities, save_image, setup_gnuplot, TSP_FILE_BERLIN52};

    fn test_tsp(enable_gif: bool, tsp_file: &str) {
        let file_name = "nearest_neighbor";
        let mut cities: Vec<(f32, f32)> = Vec::new();
        load_cities(&mut cities, tsp_file).unwrap();

        let mut gp = setup_gnuplot(&mut cities, file_name, enable_gif);

        nearest_neighbor(&mut gp, &mut cities);

        // Save final result of optimal pass as an image
        save_image(&mut gp, file_name);
    }

    #[test]
    fn test_nearest_neighbor() {
        test_tsp(true, TSP_FILE_BERLIN52);
    }

    #[test]
    fn test_nearest_neighbor_no_gif() {
        test_tsp(false, TSP_FILE_BERLIN52);
    }
}
