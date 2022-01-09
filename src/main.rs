use std::io::Write;
use std::process::{Command, Stdio};
use std::{fs::File, io::BufRead, io::BufReader, io::Read};

mod common;
mod greedy;
mod nearest_neighbor;
use common::distance;
use nearest_neighbor::nearest_neighbor;

// const TSP_FILE: &str = "kroC100.tsp.txt";
// const TSP_FILE: &str = "ts225.tsp.txt";
const TSP_FILE: &str = "berlin52.tsp.txt";

fn main() {
    let mut cities: Vec<(f32, f32)> = Vec::new();
    load_cities(&mut cities).unwrap();

    let mut gp = setup_gnuplot(&mut cities);

    // greedy(&mut gp, &mut cities);
    nearest_neighbor(&mut gp, &mut cities);

    // Save final result of optimal pass as an image
    // save_image(&mut gp);
}

fn setup_gnuplot(cities: &mut Vec<(f32, f32)>) -> std::process::Child {
    let max_x: i32 = cities.iter().map(|t| t.0 as i32).max().unwrap();
    let max_y: i32 = cities.iter().map(|t| t.1 as i32).max().unwrap();

    let mut gp = Command::new("gnuplot")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("gnuplot not found");

    // Slightly enlarge the x and y range covering the data.
    let cmd = format!(
        "set xrange [{}:{}]; set yrange [{}:{}]\n",
        -max_x / 7,
        max_x + max_x / 7,
        -max_y / 7,
        max_y + max_y / 7
    );
    let cmd: &str = &cmd;

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    let commands: Vec<&str> = vec![
        "unset key\n",
        "set term gif animate delay 1\n",
        "set output 'tsp.gif'\n\n",
    ];

    for cmd in commands.iter() {
        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }

    gp
}

fn load_cities(cities: &mut Vec<(f32, f32)>) -> std::io::Result<()> {
    let f = File::open(TSP_FILE)?;
    let mut reader = BufReader::new(f);
    let mut city_num: i32 = 0;

    for result in reader.by_ref().lines() {
        let line = result?;
        // cities.push(line);

        if line.starts_with("DIMENSION") {
            let l = line.split_whitespace().collect::<Vec<&str>>();
            city_num = l[l.len() - 1].parse::<i32>().unwrap();
            // println!("{:?}", l);
            // println!("{}", city_num);
        }

        if line.starts_with("NODE_COORD_SECTION") {
            break;
        }
    }

    for _i in 0..city_num {
        let mut buf = String::new();
        let _size = reader.read_line(&mut buf)?;
        let line = buf.split_whitespace().collect::<Vec<&str>>();
        let x = line[1].parse::<f32>().unwrap();
        let y = line[2].parse::<f32>().unwrap();

        cities.push((x, y));
    }

    Ok(())
}

// Save final result of caluculated optimal pass as an image
fn save_image(gp: &mut std::process::Child) {
    let cmd = format!("set terminal png; set output 'graph.png'; replot\n");
    let cmd: &str = &cmd;
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
}
