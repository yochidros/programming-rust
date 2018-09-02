#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::io;
use serde::Serialize;
use serde_json::Serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Player {
    location: String,
    items: Vec<String>,
    health: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Players {
    player: Vec<Player>,
}

fn main() {
    let mut players = Vec::<Player>::new();
    let player = Player { location: "Cobble crawl".to_string(), 
    items: vec!["hekki".to_string(), "debris Room".to_string()],
    health: 2,
    };
    let player2 = Player { location: "Cobble crawl".to_string(), 
    items: vec!["hekki".to_string(), "debris Room".to_string()],
    health: 2,
    };
    let player3 = Player { location: "Cobble crawl".to_string(), 
    items: vec!["hekki".to_string(), "debris Room".to_string()],
    health: 2,
    };
    players.push(player);
    players.push(player2);
    players.push(player3);

    let p = Players { player: players };
    let mut serializer = Serializer::new(io::stdout());
    let result = p.serialize(&mut serializer);

}

