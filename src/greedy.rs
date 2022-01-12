use crate::common::{distance, total_distance};
use crate::unionfind::UnionFind;
use std::fs::File;
use std::io::Write;

// Sort edges by distance
pub fn greedy(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) {
    // Distance and edge index
    let mut edges: Vec<(f32, usize, usize)> = Vec::new();
    let mut optimal_path: Vec<(f32, f32)> = Vec::new();
    let mut connected_edges: Vec<(usize, usize)> = Vec::new();
    let mut count_connected = vec![0; cities.len()];

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

    let mut uf = UnionFind::new(cities.len());

    let mut file = File::create("cities.txt").expect("Unable to create file");
    for city in cities.iter() {
        let line = format!("{} {}\n", city.0, city.1,);
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
            connected_edges.push((edges[i].1, edges[i].2));

            count_connected[edges[i].1] += 1;
            count_connected[edges[i].2] += 1;

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

            plot(gp);

            // std::thread::sleep(std::time::Duration::from_millis(200));
        }
        i += 1;
    }

    // Connect city1 with city2
    for (i, city1) in cities.iter().enumerate() {
        let mut min_dist = f32::MAX;
        let mut idx = i;

        if count_connected[i] == 2 {
            continue;
        }

        for (j, city2) in cities.iter().enumerate() {
            if i == j || uf.same(i, j) || count_connected[j] == 2 {
                continue;
            }

            let dist = distance(*city1, *city2);
            if dist < min_dist {
                min_dist = dist;
                idx = j;
            }
        }

        if i == idx {
            continue;
        }

        count_connected[i] += 1;
        count_connected[idx] += 1;

        uf.unite(i, idx);

        optimal_path.push(cities[idx]);

        let line = format!(
            "{} {} {} {}\n",
            cities[i].0, cities[i].1, cities[idx].0, cities[idx].1
        );
        file.write_all(line.as_bytes())
            .expect("Unable to write data");

        plot(gp);

        // std::thread::sleep(std::time::Duration::from_millis(200));
    }

    let idx = count_connected
        .iter()
        .enumerate()
        .filter(|j| *j.1 == 1)
        .map(|j| j.0)
        .collect::<Vec<usize>>();

    uf.unite(idx[0], idx[1]);
    let line = format!(
        "{} {} {} {}\n",
        cities[idx[0]].0, cities[idx[0]].1, cities[idx[1]].0, cities[idx[1]].1
    );
    file.write_all(line.as_bytes())
        .expect("Unable to write data");

    plot(gp);

    println!("Total distance: {}", total_distance(optimal_path));
}

fn plot(gp: &mut std::process::Child) {
    let cmd = "plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
    'edges.txt' using 1:2:($3-$1):($4-$2) with vectors lw 3 linetype 1 linecolor rgb 'cyan' nohead\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
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
