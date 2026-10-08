#![allow(unused)]

use guess_game::x_y_plane::{self, Graph, Point};

#[derive(Debug)]
enum Provinces {
    Bukhara,
    Samarkand,
    Tashkent,
}

impl Provinces {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            Provinces::Bukhara => year >= 500,
            Provinces::Samarkand => year >= 100,
            Provinces::Tashkent => year >= 750,
        }
    }
}

enum Coin {
    Penny,
    Nicel,
    Dime,
    Quarter(Provinces),
}
fn main() {
    let mut acc = 0;
    let coins = [
        Coin::Dime,
        Coin::Nicel,
        Coin::Penny,
        Coin::Quarter(Provinces::Bukhara),
    ];

    for coin in coins {
        acc += in_cents(coin) as u32;
    }

    println!("{acc}");

    {
        let coor1 = x_y_plane::Point{x: 13, y:25};
        let coor2 = x_y_plane::Point{x: 16, y: 18};

        let lin1 = x_y_plane::Line::Vertical(10);

        let graph = Graph::Point(Point{x: 10, y: 5});

        println!("the Distance: {}", coor1.distance_to_line(&lin1));
    }
}

fn describe(coin: Coin) -> Option<String> {
    let Coin::Quarter(province) = coin else {
        return None;
    };

    if province.existed_in(400) {
        Some(format!("The Province {province:?} is pretty old"))
    } else {
        Some(format!("The Province {province:?} is new!"))
    }
}

fn in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nicel => 5,
        Coin::Dime => 10,
        Coin::Quarter(province) => {
            println!("Quarter of The Province of {province:?}");
            25
        }
    }
}
