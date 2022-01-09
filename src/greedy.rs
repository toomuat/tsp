use crate::common::{distance, total_distance};
use crate::unionfind::UnionFind;
use std::fs::File;
use std::io::Write;

// Sort edges by distance
pub fn greedy(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) {
    // let start_city = cities[0];
    // Distance and edge index
    let mut edges: Vec<(f32, usize, usize)> = Vec::new();
    let mut optimal_path: Vec<(f32, f32)> = Vec::new();
    let mut optimal_path_idx: Vec<(usize, usize)> = Vec::new();
    // let mut current_idx = 0;
    let mut visit_cities: Vec<bool> = vec![false; cities.len()];

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

    greedy_plot1(gp, cities);

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

        // Check if there is cycle when connecting edges[i].1 and edges[i].2
        // or vertex is already connected to two lines then we can't connect edges[i].1 ro edges[i].2
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

            greedy_plot1(gp, cities);
            // greedy_plot2(gp, cities[edges[i].1], cities[edges[i].2]);
            greedy_plot3(gp, &mut file);

            if optimal_path.len() == cities.len() {
                // Add start point
                optimal_path.push((cities[edges[0].1].0, cities[edges[0].1].1));
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
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
    return;

    // std::thread::sleep(std::time::Duration::from_millis(2000));

    // for i in 0..50 {
    //     dbg!(edges[i]);
    // }

    // let start_city = cities[0];
    // let mut current_city = start_city;
    // let mut next_city: (f32, f32) = cities[1];
    // let mut city_idx = 1;

    // // loop {}

    // for i in 0..cities.len() - 1 {
    //     let mut min_dist = f32::MAX;

    //     for j in 1..cities.len() {
    //         let dist = distance(current_city, cities[j]);
    //         if dist < min_dist && !visit_cities[j] {
    //             next_city = cities[j];
    //             city_idx = j;
    //             min_dist = dist;
    //         }
    //     }
    // }
}

fn greedy_plot1(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) {
    let cmd =
        format!("plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black'\n");

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    return;

    let cmd: &str = "clear;\nplot '-' with point pointtype 7 pointsize 2 linecolor rgb 'black'\n";
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

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
}

fn greedy_plot2(gp: &mut std::process::Child, v1: (f32, f32), v2: (f32, f32)) {
    // Plot optimal pass
    let cmd: &str = "replot '-' with line linewidth 5 linetype 1 linecolor rgb 'cyan'\n";
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    let cmd1 = format!("{} {}\n", v1.0, v1.1);
    let cmd1: &str = &cmd1;
    let cmd2 = format!("{} {}\n", v2.0, v2.1);
    let cmd2: &str = &cmd2;
    let commands: Vec<&str> = vec![cmd1, cmd2, "e\n"];

    for cmd in commands.iter() {
        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }
}

fn greedy_plot3(gp: &mut std::process::Child, file: &mut File) {
    // let buf = BufReader::new(file);
    // let lines: Vec<String> = buf
    //     .lines()
    //     .map(|l| l.expect("Could not parse line"))
    //     .collect();

    let cmd = format!("replot 'edges.txt' using 1:2:($3-$1):($4-$2) with vectors lw 3 linetype 1 linecolor rgb 'cyan' nohead\n");

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
    // // End data input
    // gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();
}
