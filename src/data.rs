use std::path::Path;
use std::fs;
use crate::sport_content_leaderboard::Root;

pub fn read_file(name: &str, relative_to: &str) -> String {
    let path = Path::new(relative_to);
    let mut relative = path;
    if path.is_file() {
        relative = path.parent().unwrap();
    }

    let this_file = relative.join(name);
    //println!("Trying to read from: {}", this_file.display());
    let contents = fs::read_to_string(&this_file).expect("Unable to load file");
    contents
}


// Open Championship is 403 on US tour
pub fn read_leaderboard() -> Root {
    let json = read_file("./example-data/2022-open-leaderboard-pre.json", file!());

    let model : crate::sport_content_leaderboard::Root = serde_json::from_str(&json).unwrap();
    model
}