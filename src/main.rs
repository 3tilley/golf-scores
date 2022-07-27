use crate::data::read_leaderboard;
use crate::sweepstake::Player;

mod data;
mod sport_content_leaderboard;
mod sweepstake;

fn main() {
    let leaderboard = read_leaderboard();
    println!("\nStandings for {} as of {}", leaderboard.results.tournament.name, leaderboard.results.tournament.live_details.updated);
    let players: Vec<Player>   = leaderboard.results.leaderboard.into_iter().map(|x| Player::from(x)).collect();
    println!("{} players in the field", players.len());
    for player in players.into_iter().take(10) {
        println!("{}\t{}\t{} {}", player.position, player.total_to_par, player.first_name, player.last_name);
    }
}

