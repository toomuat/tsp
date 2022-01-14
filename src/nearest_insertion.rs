use crate::common::{distance, total_distance};
use std::fs::File;
use std::io::Write;

pub fn solver(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    if cfg!(feature = "plot") {
        let mut file = File::create("cities.txt").expect("Unable to create file");
        for city in cities.iter() {
            let line = format!("{} {}\n", city.0, city.1,);
            file.write_all(line.as_bytes())
                .expect("Unable to write data");
        }
    }

    let start_city = cities[0];
    let mut visit_cities: Vec<(f32, f32)> = Vec::new();
    for _ in 0..3 {
        visit_cities.push((cities[0].0, cities[0].1));
        visit_cities.remove(0);
    }
    visit_cities.push(start_city);

    // Loop over all city of current optimal path and check the distance with all the other city not included in optimal path and insert the minimum distance city to optimal path
    while !cities.is_empty() {
        let mut min_dist = i32::MAX;
        let mut insert_idx = 0;
        let mut city_idx = 0;

        for (i, visit_city) in visit_cities.iter().enumerate() {
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
        visit_cities.insert(insert_idx, cities[city_idx]);
        cities.remove(city_idx);

        // Plot all cities in points and current optimal path in lines
        #[cfg(feature = "plot")]
        plot(gp, &mut visit_cities);
    }

    println!("Total distance: {}", total_distance(&visit_cities));

    visit_cities
}

fn plot(gp: &mut std::process::Child, visit_cities: &mut Vec<(f32, f32)>) {
    let cmd = "plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
        '-' with line linewidth 5 linetype 1 linecolor rgb 'cyan'\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    for city in visit_cities.iter() {
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
        test_tsp!(solver, "nearest_insertion", true, TSP_FILE_BERLIN52);
        test_tsp!(solver, "nearest_insertion", true, TSP_FILE_KROC100);
        test_tsp!(solver, "nearest_insertion", true, TSP_FILE_TS225);
    }

    #[test]
    fn plot() {
        test_tsp!(solver, "nearest_insertion", false, TSP_FILE_BERLIN52);
    }
}
