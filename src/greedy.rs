use crate::common::{distance, total_distance};
use crate::unionfind::UnionFind;
use std::fs::File;
use std::io::Write;

// Sort edges by distance
pub fn greedy(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) {
    // Distance and edge index
    let mut edges: Vec<(f32, usize, usize)> = Vec::new();
    let mut optimal_path: Vec<(f32, f32)> = Vec::new();
    let mut optimal_path_idx: Vec<(usize, usize)> = Vec::new();

    for i in 0..cities.len() {
        for j in i..cities.len() {
            if i != j {
                // println!("{} {}", i, j);
                edges.push((distance(cities[i], cities[j]), i, j));
            }
        }
    }
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    dbg!(edges.len());

    greedy_plot1(gp);

    let mut uf = UnionFind::new(cities.len());

    let mut file = File::create("cities.txt").expect("Unable to create file");
    for i in 0..cities.len() {
        let line = format!("{} {}\n", cities[i].0, cities[i].1,);
        file.write_all(line.as_bytes())
            .expect("Unable to write data");
    }

    let mut file = File::create("edges.txt").expect("Unable to create file");

    let mut i = 0;
    loop {
        let mut is_cycle = false;

        if i == edges.len() - 1 {
            break;
        }

        // Check if there is cycle when connecting edges[i].1 and edges[i].2
        // or vertex is already connected to two lines
        // then we can't connect edges[i].1 nor edges[i].2
        if uf.same(edges[i].1, edges[i].2) || uf.size(edges[i].1) > 1 || uf.size(edges[i].2) > 1 {
            is_cycle = true;
        }

        if !is_cycle {
            optimal_path_idx.push((edges[i].1, edges[i].2));
            optimal_path.push((cities[edges[i].1].0, cities[edges[i].1].1));
            optimal_path.push((cities[edges[i].2].0, cities[edges[i].2].1));

            uf.unite(edges[i].1, edges[i].2);

            let line = format!(
                "{} {} {} {}\n",
                cities[edges[i].1].0,
                cities[edges[i].1].1,
                cities[edges[i].2].0,
                cities[edges[i].2].1
            );
            file.write_all(line.as_bytes())
                .expect("Unable to write data");

            greedy_plot1(gp);
            greedy_plot2(gp);

            if optimal_path.len() == cities.len() {
                // Add start point
                optimal_path.push((cities[edges[0].1].0, cities[edges[0].1].1));
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(200));
            // println!("@");
        }
        i += 1;
    }

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

    println!("Total distance: {}", total_distance(optimal_path));
}

// Plot all cities
fn greedy_plot1(gp: &mut std::process::Child) {
    let cmd = "plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black'\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
}

fn greedy_plot2(gp: &mut std::process::Child) {
    // let buf = BufReader::new(file);
    // let lines: Vec<String> = buf
    //     .lines()
    //     .map(|l| l.expect("Could not parse line"))
    //     .collect();

    let cmd = "replot 'edges.txt' using 1:2:($3-$1):($4-$2) with vectors lw 3 linetype 1 linecolor rgb 'cyan' nohead\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
    // // End data input
    // gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{
        load_cities, save_image, setup_gnuplot, TSP_FILE_BERLIN52, TSP_FILE_KROC100, TSP_FILE_TS225,
    };

    fn test_tsp(enable_gif: bool, tsp_file: &str) {
        let file_name = "greedy";
        let mut cities: Vec<(f32, f32)> = Vec::new();
        load_cities(&mut cities, tsp_file).unwrap();

        let tsp_name = tsp_file.split('.').collect::<Vec<&str>>()[0];
        let file_name = format!("{}_{}", file_name, tsp_name);

        let mut gp = setup_gnuplot(&mut cities, &file_name, enable_gif);

        greedy(&mut gp, &mut cities);

        // Save final result of optimal pass as an image
        save_image(&mut gp, &file_name);
    }

    #[test]
    fn test_greedy() {
        test_tsp(true, TSP_FILE_BERLIN52);
        test_tsp(true, TSP_FILE_KROC100);
        test_tsp(true, TSP_FILE_TS225);
    }

    #[test]
    fn test_greedy_no_gif() {
        test_tsp(false, TSP_FILE_BERLIN52);
    }
}
