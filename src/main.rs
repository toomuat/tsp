mod common;
mod greedy;
mod nearest_neighbor;
mod unionfind;
use common::{load_cities, save_image, setup_gnuplot, TSP_FILE_BERLIN52};
use greedy::greedy;
use nearest_neighbor::nearest_neighbor;

fn main() {
    let file_name = "greedy";
    let mut cities: Vec<(f32, f32)> = Vec::new();
    load_cities(&mut cities, TSP_FILE_BERLIN52).unwrap();

    let mut gp = setup_gnuplot(&mut cities, file_name, true);

    greedy(&mut gp, &mut cities);
    // nearest_neighbor(&mut gp, &mut cities);

    // Save final result of optimal pass as an image
    save_image(&mut gp, file_name);
}
