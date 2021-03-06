use crate::common::{distance, total_distance};
use crate::unionfind::UnionFind;
use std::fs::File;
use std::io::Write;

// Sort edges by distance
pub fn solver(
    gp: &mut std::process::Child,
    cities: &mut Vec<(f32, f32)>,
) -> (Vec<(f32, f32)>, Vec<usize>) {
    let cities_idx = greedy_internal(gp, cities);

    (
        cities_idx
            .iter()
            .map(|idx| cities[*idx])
            .collect::<Vec<(f32, f32)>>(),
        cities_idx,
    )
}

pub fn two_opt(
    gp: &mut std::process::Child,
    cities: &mut Vec<(f32, f32)>,
) -> (Vec<(f32, f32)>, Vec<usize>) {
    let mut cities_idx = greedy_internal(gp, cities);
    // In greedy_internal, start city is pushed at tail to make circle so remove it.
    cities_idx.pop();

    // greedy_internal returns index of cities so convert to coordinate
    let mut visit_cities = cities_idx
        .iter()
        .map(|idx| cities[*idx])
        .collect::<Vec<(f32, f32)>>();

    crate::two_opt::solver(gp, &mut visit_cities, &mut cities_idx)
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

    let city_len = cities.len();
    // Distance between two nodes and node indexes making a edge
    let mut edges: Vec<(i32, usize, usize)> = vec![];
    // Pairs of node indexes making edges and they are sorted in ascending order
    let mut connected_edges: Vec<(usize, usize)> = vec![];
    // Count degree of nodes. Check the nodes so that no more than three edges are connected.
    let mut count_connected = vec![0; city_len];

    for i in 0..city_len {
        for j in i..city_len {
            if i != j {
                edges.push((distance(cities[i], cities[j]), i, j));
            }
        }
    }
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut uf = UnionFind::new(city_len);

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

    assert_eq!(idx.len(), 2);
    // uf.unite(idx[0], idx[1]);
    connected_edges.push((idx[0], idx[1]));

    #[cfg(feature = "plot")]
    {
        file.write_all(
            format!(
                "{} {} {} {}\n",
                cities[idx[0]].0, cities[idx[0]].1, cities[idx[1]].0, cities[idx[1]].1
            )
            .as_bytes(),
        )
        .expect("Unable to write data");

        plot(gp);
    }

    // Code above created pairs of edges creating cycle.
    // So we next need to get the sequence of visiting cities.

    // Sequence of indexes of visiting cities
    let mut cities_idx = vec![connected_edges[0].0, connected_edges[0].1];

    connected_edges.remove(0);

    while !connected_edges.is_empty() {
        let last_city = *cities_idx.last().unwrap();

        // Search city connected to last_city
        let city = connected_edges
            .iter()
            .enumerate()
            .filter(|(_idx, city)| city.0 == last_city || city.1 == last_city)
            .map(|(idx, city)| {
                if city.0 == last_city {
                    return (idx, city.1);
                }
                (idx, city.0)
            })
            .collect::<Vec<(usize, usize)>>();

        assert_eq!(city.len(), 1);
        connected_edges.remove(city[0].0);
        cities_idx.push(city[0].1);
    }

    cities_idx
}

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

fn plot2(gp: &mut std::process::Child, edges: &Vec<Vec<f32>>) {
    let cmd = "plot 'cities.txt' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
    '-' using 1:2:($3-$1):($4-$2) with vectors lw 3 linetype 1 linecolor rgb 'cyan' nohead\n";

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    let mut cmd = "".to_owned();
    for edge in edges.iter() {
        let c = format!("{} {} {} {}\n", edge[0], edge[1], edge[2], edge[3]);
        cmd.push_str(&c);
    }
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
    // End data input
    gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();

    std::thread::sleep(std::time::Duration::from_millis(20));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bench_tsp,
        common::{
            load_cities, save_image, setup_gnuplot, TSP_FILE_BERLIN52, TSP_FILE_KROC100,
            TSP_FILE_TS225,
        },
        test_tsp,
    };
    use test::Bencher;

    // Gnuplot window cannot be seem with gif enabled
    #[test]
    fn gif() {
        test_tsp!(solver, "greedy", true, TSP_FILE_BERLIN52);
        test_tsp!(solver, "greedy", true, TSP_FILE_KROC100);
        test_tsp!(solver, "greedy", true, TSP_FILE_TS225);
    }

    #[test]
    fn all() {
        test_tsp!(solver, "greedy", false, TSP_FILE_BERLIN52);
        test_tsp!(solver, "greedy", false, TSP_FILE_KROC100);
        test_tsp!(solver, "greedy", false, TSP_FILE_TS225);
    }

    // To show Gnuplot window, we need to enable plot feature
    // `cargo test --features plot greedy::tests::plot -- --nocapture`
    #[test]
    fn berlin() {
        test_tsp!(solver, "greedy", false, TSP_FILE_BERLIN52);
    }

    // Debug mode is slow so 2 opt tests are recommended to run in release mode

    #[test]
    fn twoopt_berlin() {
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_BERLIN52);
    }

    #[test]
    fn twoopt_kroc() {
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_KROC100);
    }

    #[test]
    fn twoopt_ts() {
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_TS225);
    }

    #[test]
    fn twoopt_all() {
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_BERLIN52);
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_KROC100);
        test_tsp!(two_opt, "greedy_twoopt", false, TSP_FILE_TS225);
    }

    #[test]
    fn twoopt_gif_all() {
        test_tsp!(two_opt, "greedy_twoopt", true, TSP_FILE_BERLIN52);
        test_tsp!(two_opt, "greedy_twoopt", true, TSP_FILE_KROC100);
        test_tsp!(two_opt, "greedy_twoopt", true, TSP_FILE_TS225);
    }

    // Executed 301 times
    #[bench]
    fn bench_berlin(b: &mut Bencher) {
        bench_tsp!(b, solver, TSP_FILE_BERLIN52);
    }

    #[bench]
    fn bench_kroc(b: &mut Bencher) {
        bench_tsp!(b, solver, TSP_FILE_KROC100);
    }

    #[bench]
    fn bench_ts(b: &mut Bencher) {
        bench_tsp!(b, solver, TSP_FILE_TS225);
    }

    #[bench]
    fn bench_twoopt_berlin(b: &mut Bencher) {
        bench_tsp!(b, two_opt, TSP_FILE_BERLIN52);
    }

    #[bench]
    fn bench_twoopt_kroc(b: &mut Bencher) {
        bench_tsp!(b, two_opt, TSP_FILE_KROC100);
    }

    #[bench]
    fn bench_twoopt_ts(b: &mut Bencher) {
        bench_tsp!(b, two_opt, TSP_FILE_TS225);
    }
}
