use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub meta: Meta,
    pub results: Results,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub title: String,
    pub description: String,
    pub fields: Fields,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    #[serde(rename = "tournament_object")]
    pub tournament_object: TournamentObject,
    #[serde(rename = "leaderboard_array")]
    pub leaderboard_array: SportContentPlayer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentObject {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "tour_id")]
    pub tour_id: String,
    pub name: String,
    pub country: String,
    pub course: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
    pub timezone: String,
    #[serde(rename = "prize_fund")]
    pub prize_fund: String,
    #[serde(rename = "fund_currency")]
    pub fund_currency: String,
    #[serde(rename = "live_details")]
    pub live_details: LiveDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDetails {
    pub status: String,
    #[serde(rename = "current_round")]
    pub current_round: String,
    #[serde(rename = "total_rounds")]
    pub total_rounds: String,
    pub players: String,
    pub updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SportContentPlayer {
    pub position: String,
    #[serde(rename = "player_id")]
    pub player_id: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub country: String,
    #[serde(rename = "holes_played")]
    pub holes_played: String,
    #[serde(rename = "current_round")]
    pub current_round: String,
    pub status: String,
    pub strokes: String,
    pub updated: String,
    #[serde(rename = "prize_money")]
    pub prize_money: String,
    #[serde(rename = "ranking_points")]
    pub ranking_points: String,
    #[serde(rename = "total_to_par")]
    pub total_to_par: String,
    #[serde(rename = "rounds_array")]
    pub rounds_array: RoundsArray,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundsArray {
    #[serde(rename = "round_number")]
    pub round_number: String,
    #[serde(rename = "course_number")]
    pub course_number: String,
    #[serde(rename = "position_round")]
    pub position_round: String,
    #[serde(rename = "tee_time_local")]
    pub tee_time_local: String,
    #[serde(rename = "total_to_par")]
    pub total_to_par: String,
    pub strokes: String,
    pub updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Results {
    pub tournament: Tournament,
    pub leaderboard: Vec<Leaderboard>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "tour_id")]
    pub tour_id: i64,
    pub name: String,
    pub country: String,
    pub course: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
    pub timezone: String,
    #[serde(rename = "prize_fund")]
    pub prize_fund: String,
    #[serde(rename = "fund_currency")]
    pub fund_currency: String,
    #[serde(rename = "live_details")]
    pub live_details: LiveDetails2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDetails2 {
    pub status: String,
    #[serde(rename = "current_round")]
    pub current_round: i64,
    #[serde(rename = "total_rounds")]
    pub total_rounds: i64,
    pub players: i64,
    #[serde(rename = "cut_value")]
    pub cut_value: i64,
    pub updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub position: i64,
    #[serde(rename = "player_id")]
    pub player_id: i64,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub country: String,
    #[serde(rename = "holes_played")]
    pub holes_played: i64,
    #[serde(rename = "current_round")]
    pub current_round: i64,
    pub status: String,
    pub strokes: i64,
    pub updated: String,
    #[serde(rename = "prize_money")]
    pub prize_money: String,
    #[serde(rename = "ranking_points")]
    pub ranking_points: String,
    #[serde(rename = "total_to_par")]
    pub total_to_par: i64,
    pub rounds: Vec<Round>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    #[serde(rename = "round_number")]
    pub round_number: i64,
    #[serde(rename = "course_number")]
    pub course_number: i64,
    #[serde(rename = "position_round")]
    pub position_round: i64,
    #[serde(rename = "tee_time_local")]
    pub tee_time_local: String,
    #[serde(rename = "total_to_par")]
    pub total_to_par: i64,
    pub strokes: Value,
    pub updated: String,
}
