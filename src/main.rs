#![allow(dead_code)]
fn main() -> () {
    println!("Hello, world!");
    let squad: Vec<Player> = vec![
        Player {
            name: String::from("R. Cunningham \"The Ultimate Weapon\""),
            chem_options: vec![String::from("PHI.2")]
        }
    ];
    println!("{:?}", squad);
}

#[derive(Debug)]
struct Player {
    name: String,
    chem_options: Vec<String>
}

enum Team {
    ARI,
    ATL,
    BAL,
    BUF,
    CAR,
    CHI,
    CIN,
    CLE,
    DAL,
    DEN,
    DET,
    GB,
    HOU,
    IND,
    JAC,
    KC,
    LAC,
    LAR,
    LV,
    MIA,
    MIN,
    NE,
    NO,
    NYG,
    NYJ,
    PHI,
    PIT,
    SEA,
    SF,
    TB,
    TEN,
    WAS ,
}

fn string_to_team(input_string: &str) -> Vec<Team> {
    let mut return_val: Vec<Team> = vec![];

    let _parts: Vec<&str> = input_string.split('.').collect();

    let team_string : &str = _parts[0];
    let num_string: i32 = match _parts[1].parse::<i32>() {
        Ok(n) => n,
        Err(_) => 1
    };

    for _ in 0..num_string {
        match team_string {
            "ARI" => return_val.push(Team::ARI),
            "ATL" => return_val.push(Team::ATL),
            "BAL" => return_val.push(Team::BAL),
            "BUF" => return_val.push(Team::BUF),
            "CAR" => return_val.push(Team::CAR),
            "CHI" => return_val.push(Team::CHI),
            "CIN" => return_val.push(Team::CIN),
            "CLE" => return_val.push(Team::CLE),
            "DAL" => return_val.push(Team::DAL),
            "DEN" => return_val.push(Team::DEN),
            "DET" => return_val.push(Team::DET),
            "GB" => return_val.push(Team::GB),
            "HOU" => return_val.push(Team::HOU),
            "IND" => return_val.push(Team::IND),
            "JAC" => return_val.push(Team::JAC),
            "KC" => return_val.push(Team::KC),
            "LAC" => return_val.push(Team::LAC),
            "LAR" => return_val.push(Team::LAR),
            "LV" => return_val.push(Team::LV),
            "MIA" => return_val.push(Team::MIA),
            "MIN" => return_val.push(Team::MIN),
            "NE" => return_val.push(Team::NE),
            "NO" => return_val.push(Team::NO),
            "NYG" => return_val.push(Team::NYG),
            "NYJ" => return_val.push(Team::NYJ),
            "PHI" => return_val.push(Team::PHI),
            "PIT" => return_val.push(Team::PIT),
            "SEA" => return_val.push(Team::SEA),
            "SF" => return_val.push(Team::SF),
            "TB" => return_val.push(Team::TB),
            "TEN" => return_val.push(Team::TEN),
            "WAS" => return_val.push(Team::WAS),
            _ => (),
        }
    }

    return return_val;
}