extern crate quicksilver;
extern crate core;

mod entry;

use quicksilver::geom::Vector;
use quicksilver::lifecycle::{run, Settings};
use entry::Canvas;

fn main() {
    let title = "War of Empires";
    let size = Vector::new(800, 600);
    let settings = Settings {
        draw_rate: 30.,
        update_rate: 30.,
        vsync: false,
        ..Settings::default()
    };

    run::<Canvas>(title, size, settings);
}
