use crate::common::distance;
use crate::unionfind::UnionFind;
use std::fs::File;
use std::io::Write;

// Sort edges by distance
pub fn greedy(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) {
    if cfg!(feature = "plot") {
        let mut file = File::create("cities.txt").expect("Unable to create file");
        for city in cities.iter() {
            let line = format!("{} {}\n", city.0, city.1,);
            file.write_all(line.as_bytes())
                .expect("Unable to write data");
        }
    }

    #[cfg(feature = "plot")]
    let mut file = File::create("edges.txt").expect("Unable to create file");

    // Distance and edge index
    let mut edges: Vec<(i32, usize, usize)> = Vec::new();
    let mut connected_edges: Vec<(usize, usize)> = Vec::new();
    let mut count_connected = vec![0; cities.len()];

    for i in 0..cities.len() {
        for j in i..cities.len() {
            if i != j {
                edges.push((distance(cities[i], cities[j]), i, j));
            }
        }
    }
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut uf = UnionFind::new(cities.len());

    for edge in edges.iter() {
        // Check if there is cycle when connecting edge.1 and edge.2
        // or vertex is already connected to two lines
        // then we can't connect edge.1 nor edge.2
        if uf.same(edge.1, edge.2) || count_connected[edge.1] == 2 || count_connected[edge.2] == 2 {
            continue;
        }

        connected_edges.push((edge.1, edge.2));

        count_connected[edge.1] += 1;
        count_connected[edge.2] += 1;

        uf.unite(edge.1, edge.2);

        #[cfg(feature = "plot")]
        file.write_all(
            format!(
                "{} {} {} {}\n",
                cities[edge.1].0, cities[edge.1].1, cities[edge.2].0, cities[edge.2].1
            )
            .as_bytes(),
        )
        .expect("Unable to write data");

        #[cfg(feature = "plot")]
        plot(gp);
    }

    // Connect remaining two points and make a cycle
    let idx = count_connected
        .iter()
        .enumerate()
        .filter(|j| *j.1 == 1)
        .map(|j| j.0)
        .collect::<Vec<usize>>();

    if !idx.is_empty() {
        // uf.unite(idx[0], idx[1]);
        connected_edges.push((idx[0], idx[1]));

        #[cfg(feature = "plot")]
        file.write_all(
            format!(
                "{} {} {} {}\n",
                cities[idx[0]].0, cities[idx[0]].1, cities[idx[1]].0, cities[idx[1]].1
            )
            .as_bytes(),
        )
        .expect("Unable to write data");
    }

    #[cfg(feature = "plot")]
    plot(gp);

    println!(
        "Total distance: {}",
        total_distance(cities, connected_edges)
    );
}

fn total_distance(cities: &mut Vec<(f32, f32)>, edges: Vec<(usize, usize)>) -> i32 {
    edges
        .iter()
        .fold(0, |sum, i| sum + distance(cities[i.0], cities[i.1]))
}

fn plot(gp: &mut std::process::Child) {
    let cmd = "plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
    'edges.txt' using 1:2:($3-$1):($4-$2) with vectors lw 3 linetype 1 linecolor rgb 'cyan' nohead\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(200));
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

    // To show Gnuplot window, we need to enable plot feature
    // `cargo test --features plot greedy::tests::plot -- --nocapture`
    #[test]
    fn plot() {
        test_tsp(false, TSP_FILE_BERLIN52);
    }
}
