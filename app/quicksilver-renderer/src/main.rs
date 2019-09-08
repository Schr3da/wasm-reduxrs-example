extern crate core;
extern crate quicksilver;

mod entry;

use entry::Canvas;
use quicksilver::geom::Vector;
use quicksilver::lifecycle::{run, Settings};

fn main() {
    let title = "War of Empires";
    let size = Vector::new(800, 600);
    let settings = Settings {
        draw_rate: 50.,
        update_rate: 50.,
        vsync: false,
        ..Settings::default()
    };

    run::<Canvas>(title, size, settings);
}
