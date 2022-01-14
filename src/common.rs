use std::io::Write;
use std::process::{Command, Stdio};
use std::{fs::File, io::BufRead, io::BufReader, io::Read};

pub const TSP_FILE_KROC100: &str = "kroC100.tsp.txt";
pub const TSP_FILE_TS225: &str = "ts225.tsp.txt";
pub const TSP_FILE_BERLIN52: &str = "berlin52.tsp.txt";

pub fn distance(v1: (f32, f32), v2: (f32, f32)) -> i32 {
    ((v1.0 - v2.0).powf(2.0) + (v1.1 - v2.1).powf(2.0)).sqrt() as i32
}

pub fn total_distance(cities: &Vec<(f32, f32)>) -> i32 {
    (0..cities.len() - 1).fold(0, |sum, i| sum + distance(cities[i], cities[i + 1]))
}

// Save final result of caluculated optimal pass as an image
pub fn save_image(gp: &mut std::process::Child, file_name: &str) {
    let cmd = format!(
        "set terminal png; set output 'images/{}.png'; replot\n",
        file_name
    );
    let cmd: &str = &cmd;
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
}

// If we enable gif output, then we can't see Gnuplot window
// So if disable gif to see it.
pub fn setup_gnuplot(
    cities: &mut Vec<(f32, f32)>,
    file_name: &str,
    enable_gif: bool,
) -> std::process::Child {
    let max_x: i32 = cities.iter().map(|t| t.0 as i32).max().unwrap();
    let max_y: i32 = cities.iter().map(|t| t.1 as i32).max().unwrap();

    let mut gp = Command::new("gnuplot")
        .stdin(Stdio::piped())
        // .stderr(Stdio::piped())
        // .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute gnuplot");

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

    let mut commands: Vec<&str>;
    let cmd = format!("set output 'images/{}.gif'\n", file_name);
    let cmd: &str = &cmd;
    commands = vec!["unset key\n", "set term gif animate delay 1\n", cmd];

    if !enable_gif {
        commands = vec!["unset key\n"];
    }

    for cmd in commands {
        gp.stdin
            .as_mut()
            .unwrap()
            .write_all(cmd.as_bytes())
            .unwrap();
    }

    gp
}

pub fn load_cities(cities: &mut Vec<(f32, f32)>, tsp_file: &str) -> std::io::Result<()> {
    let f = File::open(tsp_file)?;
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
