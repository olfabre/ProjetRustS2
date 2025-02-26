use serde:: {Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Joueur {
    r#type: String,
    r#position: String,
}


fn main() {
    println!("Hello, world!");

    let json_str = fs::read_to_string("src/data.json").expect("Can't read file");
    let joueur: Joueur = serde_json::from_str(&json_str).expect("Can't parse json");

    println!("{}", joueur.r#type);

}
