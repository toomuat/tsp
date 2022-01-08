use std::io::Write;
use std::process::{Command, Stdio};
use std::{fs::File, io::BufRead, io::BufReader, io::Read};

// const TSP_FILE: &str = "kroC100.tsp.txt";
// const TSP_FILE: &str = "ts225.tsp.txt";
const TSP_FILE: &str = "berlin52.tsp.txt";

fn main() {
    let mut cities: Vec<(f32, f32)> = Vec::new();
    load_cities(&mut cities).unwrap();

    let max_x: i32 = cities.iter().map(|t| t.0 as i32).max().unwrap();
    let max_y: i32 = cities.iter().map(|t| t.1 as i32).max().unwrap();

    let mut gp = Command::new("gnuplot")
        .arg("-geometry")
        .arg("640x480")
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

    // Plot optimal path
    let start_city = cities[0];
    let mut current_city = start_city;
    let mut visit_cities: Vec<bool> = vec![false; cities.len()];
    let mut optimal_path: Vec<(f32, f32)> = Vec::new();
    let mut next_city: (f32, f32) = cities[1];
    let mut city_idx = 1;

    visit_cities[0] = true;
    optimal_path.push(start_city);

    println!("{} {}", start_city.0, start_city.1);

    for i in 0..cities.len() - 1 {
        let mut min_dist = f32::MAX;

        for j in 1..cities.len() {
            let dist = distance(current_city, cities[j]);
            if dist < min_dist && !visit_cities[j] {
                next_city = cities[j];
                city_idx = j;
                min_dist = dist;
            }
        }

        optimal_path.push(next_city);
        current_city = next_city;
        visit_cities[city_idx] = true;

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

        for j in 0..optimal_path.len() {
            let cmd = format!("{} {}\n", optimal_path[j].0, optimal_path[j].1);
            let cmd: &str = &cmd;

            gp.stdin
                .as_mut()
                .unwrap()
                .write_all(cmd.as_bytes())
                .unwrap();
        }

        if i == cities.len() - 2 {
            let cmd = format!("{} {}\n", start_city.0, start_city.1);
            let cmd: &str = &cmd;
            gp.stdin
                .as_mut()
                .unwrap()
                .write_all(cmd.as_bytes())
                .unwrap();
        }

        // End data input
        gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();

        // std::thread::sleep(std::time::Duration::from_millis(200));
    }

    replot(&mut gp);
}

fn swap_cities(cities: &mut Vec<(f32, f32)>, i: usize, j: usize) {
    let tmp = cities[i];
    cities[i] = cities[j];
    cities[j] = tmp;
}

fn distance(v1: (f32, f32), v2: (f32, f32)) -> f32 {
    ((v1.0 - v2.0).powf(2.0) + (v1.1 - v2.1).powf(2.0)).sqrt()
}

fn total_distance(cities: Vec<(f32, f32)>) -> f32 {
    (0..cities.len() - 1).fold(0.0, |sum, i| sum + distance(cities[i], cities[i + 1]))
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

fn replot(gp: &mut std::process::Child) {
    let cmd = format!("set terminal png; set output 'graph.png'; replot\n");
    let cmd: &str = &cmd;
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    println!("bye");
    std::thread::sleep(std::time::Duration::from_millis(3000));
}
