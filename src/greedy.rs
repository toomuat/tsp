use rand::{thread_rng, Rng};

use crate::common::{distance, total_distance};
use crate::unionfind::UnionFind;
use std::fs::File;
use std::io::Write;

// Sort edges by distance
pub fn solver(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let cities_idx = greedy_internal(gp, cities);

    let mut visit_cities = cities_idx
        .iter()
        .map(|idx| cities[*idx])
        .collect::<Vec<(f32, f32)>>();

    println!("Total distance: {}", total_distance(&visit_cities));

    visit_cities
}

pub fn two_opt(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut cities_idx = greedy_internal(gp, cities);
    cities_idx.pop();
    let city_len = cities_idx.len();

    let mut visit_cities = cities_idx
        .iter()
        .map(|idx| cities[*idx])
        .collect::<Vec<(f32, f32)>>();

    // Swap
    let mut rng = thread_rng();
    let mut n = 100_000;
    while n > 0 {
        let mut i = rng.gen_range(0..city_len);
        let mut j = rng.gen_range(0..city_len);

        if i > j {
            std::mem::swap(&mut i, &mut j);
        }

        let x = (i + city_len - 1) % city_len;
        let y = (j + city_len + 1) % city_len;

        // Current distance
        let d1 =
            distance(visit_cities[x], visit_cities[i]) + distance(visit_cities[j], visit_cities[y]);
        // Distance after swapped
        let d2 =
            distance(visit_cities[x], visit_cities[j]) + distance(visit_cities[i], visit_cities[y]);

        if d1 > d2 && x != j && y != i {
            if i == 0 && j == city_len - 1 {
                visit_cities.swap(i, j);
            } else {
                let v = visit_cities.clone();
                for (l, m) in (i..=j).enumerate() {
                    visit_cities[m] = v[j - l];
                }
            }

            let mut v = visit_cities.clone();
            v.push(v[0]);
            let new = total_distance(&v);
            println!("[{}] {}", n, new);

            #[cfg(feature = "plot")]
            {
                let mut edges = Vec::new();

                for i in 0..city_len - 1 {
                    edges.push(vec![
                        visit_cities[i].0,
                        visit_cities[i].1,
                        visit_cities[i + 1].0,
                        visit_cities[i + 1].1,
                    ]);
                }

                // Connect start and end city to make cycle
                edges.push(vec![
                    visit_cities[city_len - 1].0,
                    visit_cities[city_len - 1].1,
                    visit_cities[0].0,
                    visit_cities[0].1,
                ]);

                plot2(gp, &mut edges);
            }
        }
        n -= 1;
    }

    // Connect start and end city to make cycle
    // cities_idx.push(cities_idx[0]);
    visit_cities.push(visit_cities[0]);

    println!("Total distance: {}", total_distance(&visit_cities));

    visit_cities
}

fn greedy_internal(gp: &mut std::process::Child, cities: &mut Vec<(f32, f32)>) -> Vec<usize> {
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

    // Sequence of visiting cities
    let mut visit_cities = Vec::new();

    visit_cities.push(connected_edges[0].0);
    visit_cities.push(connected_edges[0].1);
    connected_edges.remove(0);

    while !connected_edges.is_empty() {
        let last_city = *visit_cities.last().unwrap();

        let city = connected_edges
            .iter()
            .enumerate()
            .filter(|(idx, city)| city.0 == last_city || city.1 == last_city)
            .map(|(idx, city)| {
                if city.0 == last_city {
                    return (idx, city.1);
                }
                (idx, city.0)
            })
            .collect::<Vec<(usize, usize)>>();

        assert_eq!(city.len(), 1);

        connected_edges.remove(city[0].0);
        visit_cities.push(city[0].1);
    }

    visit_cities
}

// fn total_distance(cities: &mut Vec<(f32, f32)>, edges: Vec<(usize, usize)>) -> i32 {
//     edges
//         .iter()
//         .fold(0, |sum, i| sum + distance(cities[i.0], cities[i.1]))
// }

fn plot(gp: &mut std::process::Child) {
    let cmd = "plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
    'edges.txt' using 1:2:($3-$1):($4-$2) with vectors lw 3 linetype 1 linecolor rgb 'cyan' nohead\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(20));
}

fn plot2(gp: &mut std::process::Child, edges: &mut Vec<Vec<f32>>) {
    let cmd = "plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
    '-' using 1:2:($3-$1):($4-$2) with vectors lw 3 linetype 1 linecolor rgb 'cyan' nohead\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    for edge in edges.iter() {
        let cmd = format!("{} {} {} {}\n", edge[0], edge[1], edge[2], edge[3]);
        // let cmd: &str = &cmd;

        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }
    // End data input
    gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();

    std::thread::sleep(std::time::Duration::from_millis(20));
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
        test_tsp!(solver, "greedy", true, TSP_FILE_BERLIN52);
        test_tsp!(solver, "greedy", true, TSP_FILE_KROC100);
        test_tsp!(solver, "greedy", true, TSP_FILE_TS225);
    }

    // To show Gnuplot window, we need to enable plot feature
    // `cargo test --features plot greedy::tests::plot -- --nocapture`
    #[test]
    fn plot() {
        test_tsp!(solver, "greedy", false, TSP_FILE_BERLIN52);
    }

    #[test]
    fn twoopt_plot() {
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_BERLIN52);
    }

    #[test]
    fn twoopt_plot_all() {
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_BERLIN52);
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_KROC100);
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_TS225);
    }

    #[test]
    fn twoopt_plot_gif_all() {
        test_tsp!(two_opt, "greedy_twoopt", true, TSP_FILE_BERLIN52);
        test_tsp!(two_opt, "greedy_twoopt", true, TSP_FILE_KROC100);
        test_tsp!(two_opt, "greedy_twoopt", true, TSP_FILE_TS225);
    }
}
