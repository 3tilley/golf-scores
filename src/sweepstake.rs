use crate::sport_content_leaderboard::{SportContentPlayer, Leaderboard};
use std::str::FromStr;

pub struct Gambler {
    pub id: u16,
    pub name: str,
}

pub struct Player {
    pub position: i64,
    pub player_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
    // TODO: Should this be unsigned?
    pub holes_played: u8,
    pub current_round: u8,
    pub status: String,
    pub strokes: u8,
    pub updated: String,
    pub prize_money: String,
    pub ranking_points: String,
    pub total_to_par: i16,
}

impl From<SportContentPlayer> for Player {
    // TODO: Tidy this up and use some more idiomatic types
    fn from(s: SportContentPlayer) -> Self {
        Player{
            position: i64::from_str(&*s.position).unwrap(),
            player_id: i64::from_str(&*s.player_id).unwrap(),
            first_name: s.first_name,
            last_name: s.last_name,
            country: s.country,
            holes_played: s.holes_played.parse().unwrap(),
            current_round: s.current_round.parse().unwrap(),
            status: s.status,
            strokes: s.strokes.parse().unwrap(),
            updated: s.updated,
            prize_money: s.prize_money,
            ranking_points: s.ranking_points,
            total_to_par: i16::from_str(&*s.total_to_par).unwrap(),
        }
    }
}

impl From<Leaderboard> for Player {
    fn from(s: Leaderboard) -> Self {
        Player{
            position: s.position,
            player_id: s.player_id,
            first_name: s.first_name,
            last_name: s.last_name,
            country: s.country,
            holes_played: s.holes_played as u8,
            current_round: s.current_round as u8,
            status: s.status,
            strokes: s.strokes as u8,
            updated: s.updated,
            prize_money: s.prize_money,
            ranking_points: s.ranking_points,
            total_to_par: s.total_to_par as i16,
        }
    }
}

pub struct GamblerDraft {
    // The team does not include the captain or wildcard. This decision needs to be considered
    pub team: Vec<Player>,
    pub captain: Player,
    pub wildcard: Player,
}

pub trait RuleSet {
    fn player_score(&self, player: &Player) -> Option<i16>;
    fn team_score(&self, draft: &GamblerDraft) -> Option<i16>;
}

pub struct StandardRules {

}

impl RuleSet for StandardRules {
    fn player_score(&self, player: &Player) -> Option<i16> {
        Some(player.total_to_par)
    }

    fn team_score(&self, draft: &GamblerDraft) -> Option<i16> {
        // Replace player_score
        Some(draft.team.iter().map(|p| p.total_to_par).sum())
    }
}

// TODO: Why does this need a lifetime parameter?
pub struct Draft<'a> {
    pub ruleset: Box<dyn RuleSet>,
    pub draft: Vec<(&'a Gambler, &'a GamblerDraft)>,

}