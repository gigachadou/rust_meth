#![allow(unused)]

use guess_game::x_y_plane::{Circle, Graph, Point};

fn main() {
    let mut circle1 = Circle::new_on_origin(35);
    let point1 = Point::new(-10, 30);

    let mut cr2 = Circle::make_from(&point1, 10);

    circle1.move_center(&Point::new(-100, 100));

    circle1.print();

    cr2.print();
}
