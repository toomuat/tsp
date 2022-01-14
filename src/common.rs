use std::io::Write;
use std::process::{Command, Stdio};
use std::{fs::File, io::BufRead, io::BufReader, io::Read};

pub const TSP_FILE_KROC100: &str = "kroC100.tsp.txt";
pub const TSP_FILE_TS225: &str = "ts225.tsp.txt";
pub const TSP_FILE_BERLIN52: &str = "berlin52.tsp.txt";

pub fn distance(v1: (f32, f32), v2: (f32, f32)) -> i32 {
    ((v1.0 - v2.0).powf(2.0) + (v1.1 - v2.1).powf(2.0)).sqrt() as i32
}

pub fn total_distance(cities: &[(f32, f32)]) -> i32 {
    (0..cities.len() - 1).fold(0, |sum, i| sum + distance(cities[i], cities[i + 1]))
}

// Save final result of caluculated optimal pass as an image
pub fn save_image(
    gp: &mut std::process::Child,
    file_name: &str,
    cities: Vec<(f32, f32)>,
    visit_cities: Vec<(f32, f32)>,
) {
    return;
    let cmd = format!(
        "set terminal png; set output 'images/{}.png'; \
        plot '-' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
        '-' with line linewidth 5 linetype 1 linecolor rgb 'cyan'\n",
        file_name
    );
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    replot(gp, &cities, &visit_cities);
}

pub fn plot(
    gp: &mut std::process::Child,
    cities: &Vec<(f32, f32)>,
    visit_cities: &Vec<(f32, f32)>,
) {
    let cmd = "plot '-' with point pointtype 7 pointsize 2 linecolor rgb 'black', \
        '-' with line linewidth 5 linetype 1 linecolor rgb 'cyan'\n";
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    replot(gp, &cities, &visit_cities);
}

pub fn replot(
    gp: &mut std::process::Child,
    cities: &Vec<(f32, f32)>,
    visit_cities: &Vec<(f32, f32)>,
) {
    // Plot all cities
    let mut cmd: String = "".to_owned();
    for city in cities.iter() {
        let c = format!("{} {}\n", city.0, city.1);
        cmd.push_str(&c);
    }
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
    // End data input
    gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();

    // Plot optimal pass
    cmd = "".to_owned();
    for city in visit_cities.iter() {
        let c = format!("{} {}\n", city.0, city.1);
        cmd.push_str(&c);
    }
    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();
    // End data input
    gp.stdin.as_mut().unwrap().write_all(b"e\n").unwrap();
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
    let min_x: i32 = cities.iter().map(|t| t.0 as i32).min().unwrap();
    let min_y: i32 = cities.iter().map(|t| t.1 as i32).min().unwrap();

    let mut gp = Command::new("gnuplot")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute gnuplot");

    // Slightly enlarge the x and y range covering the data.
    let xrange = max_x - min_x;
    let yrange = max_y - min_y;
    let max_x = max_x + xrange / 7;
    let min_x = min_x - xrange / 7;
    let max_y = max_y + yrange / 7;
    let min_y = min_y - yrange / 7;
    let mut cmd = format!(
        "set xrange [{}:{}]; set yrange [{}:{}]\n",
        min_x, max_x, min_y, max_y
    );

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

    if cfg!(feature = "plot") && enable_gif {
        cmd = format!(
            "unset key; set term gif animate delay 1; \
                set output 'images/{}.gif\n",
            file_name
        );
    } else {
        cmd = "unset key\n".to_string();
    }

    gp.stdin
        .as_mut()
        .unwrap()
        .write_all(cmd.as_bytes())
        .unwrap();

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

#[macro_export]
macro_rules! test_tsp {
    ($solver:ident, $name:expr, $enable_gif:expr, $tsp_file:expr) => {
        let mut cities: Vec<(f32, f32)> = Vec::new();
        load_cities(&mut cities, $tsp_file).unwrap();

        let tsp_name = $tsp_file.split('.').collect::<Vec<&str>>()[0];
        let file_name = format!("{}_{}", $name, tsp_name);

        let mut gp = setup_gnuplot(&mut cities, &file_name, $enable_gif);

        let now = std::time::Instant::now();

        let visit_cities = $solver(&mut gp, &mut cities);

        // println!("{}", now.elapsed().as_millis());
        println!("{}", now.elapsed().as_micros());
        // println!("{}", now.elapsed().as_nanos());

        // Save final result of optimal pass as an image
        save_image(&mut gp, &file_name, cities, visit_cities);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replot() {
        let mut cities = vec![(-50., -50.), (-50., 0.), (-50., 50.), (50., 50.)];
        let visit_cities = vec![
            (-50., -50.),
            (-50., 0.),
            (-50., 50.),
            (50., 50.),
            (-50., -50.),
        ];
        let file_name = "test_replot";

        let mut gp = setup_gnuplot(&mut cities, file_name, false);

        save_image(&mut gp, file_name, cities, visit_cities);
    }
}
