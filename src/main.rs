use crate::data::read_leaderboard;

mod data;
mod leaderboard;

fn main() {
    println!("Hello, world!");
    let leaderboard = read_leaderboard();
    let players   = leaderboard.results.leaderboard;
    println!("{} players in the field", players.len())
}
