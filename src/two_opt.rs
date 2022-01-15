use rand::{thread_rng, Rng};

use crate::common::{distance, total_distance};
use std::fs::File;
use std::io::Write;

pub fn solver(
    gp: &mut std::process::Child,
    visit_cities: &mut Vec<(f32, f32)>,
    cities_idx: &mut Vec<usize>,
) -> (Vec<(f32, f32)>, Vec<usize>) {
    let mut last_update_idx = 0;
    let city_len = visit_cities.len();

    // Swap
    let mut rng = thread_rng();
    // Number of iteration
    let limit = 10_000_000;
    for k in 0..limit {
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
            last_update_idx = k;

            if i == 0 && j == city_len - 1 {
                visit_cities.swap(i, j);
                cities_idx.swap(i, j);
            } else {
                let v = visit_cities.clone();
                let c = cities_idx.clone();
                for (l, m) in (i..=j).enumerate() {
                    visit_cities[m] = v[j - l];
                    cities_idx[m] = c[j - l]
                }
            }

            #[cfg(feature = "plot")]
            {
                let mut v = visit_cities.clone();
                v.push(v[0]);
                let new = total_distance(&v);
                // println!("[{}] {}", limit, new);

                let mut edges = vec![];

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

                plot(gp, &mut edges);
            }
        }
    }

    // Connect start and end city to make cycle
    visit_cities.push(visit_cities[0]);

    // println!("Last update index: {}", last_update_idx);

    (visit_cities.to_vec(), cities_idx.to_vec())
}

fn plot(gp: &mut std::process::Child, edges: &mut Vec<Vec<f32>>) {
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
