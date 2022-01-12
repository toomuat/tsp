use crate::common::{distance, total_distance};
use std::fs::File;
use std::io::Write;

pub fn nearest_insertion(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) {
    let mut file = File::create("cities.txt").expect("Unable to create file");
    for city in cities.iter() {
        let line = format!("{} {}\n", city.0, city.1,);
        file.write_all(line.as_bytes())
            .expect("Unable to write data");
    }

    let start_city = cities[0];
    let mut optimal_path: Vec<(f32, f32)> = Vec::new();
    for _ in 0..3 {
        optimal_path.push((cities[0].0, cities[0].1));
        optimal_path.remove(0);
    }
    optimal_path.push(start_city);

    // Loop over all city of current optimal path and check the distance with all the other city not included in optimal path and insert the minimum distance city to optimal path
    while !cities.is_empty() {
        let mut min_dist = f32::MAX;
        let mut insert_idx = 0;
        let mut city_idx = 0;

        for (i, visit_city) in optimal_path.iter().enumerate() {
            for (j, city) in cities.iter().enumerate() {
                let dist = distance(*visit_city, *city);

                if dist < min_dist {
                    min_dist = dist;
                    insert_idx = i + 1;
                    city_idx = j;
                }
            }
        }

        //  Insert closest city to cities in current optimal path
        optimal_path.insert(insert_idx, cities[city_idx]);
        cities.remove(city_idx);

        // Plot all cities in points and current optimal path in lines
        plot(gp, &mut optimal_path);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("Total distance: {}", total_distance(optimal_path));
}

fn plot(gp: &mut std::process::Child, optimal_path: &mut Vec<(f32, f32)>) {
    let cmd = "plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
        '-' with line linewidth 5 linetype 1 linecolor rgb 'cyan'\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{
        load_cities, save_image, setup_gnuplot, TSP_FILE_BERLIN52, TSP_FILE_KROC100, TSP_FILE_TS225,
    };

    fn test_tsp(enable_gif: bool, tsp_file: &str) {
        let file_name = "nearest_insertion";
        let mut cities: Vec<(f32, f32)> = Vec::new();
        load_cities(&mut cities, tsp_file).unwrap();

        let tsp_name = tsp_file.split('.').collect::<Vec<&str>>()[0];
        let file_name = format!("{}_{}", file_name, tsp_name);

        let mut gp = setup_gnuplot(&mut cities, &file_name, enable_gif);

        nearest_insertion(&mut gp, &mut cities);

        // Save final result of optimal pass as an image
        save_image(&mut gp, &file_name);
    }

    #[test]
    fn test_nearest_insertion() {
        test_tsp(true, TSP_FILE_BERLIN52);
        test_tsp(true, TSP_FILE_KROC100);
        test_tsp(true, TSP_FILE_TS225);
    }

    #[test]
    fn test_nearest_insertion_no_gif() {
        test_tsp(false, TSP_FILE_BERLIN52);
    }
}