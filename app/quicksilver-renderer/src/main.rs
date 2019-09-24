extern crate quicksilver;
extern crate core;

mod entry;

use core::reducers::DEFAULT_RESOLUTION;

use quicksilver::geom::Vector;
use quicksilver::lifecycle::{run, Settings};
use entry::Canvas;

fn main() {
    let title = "War of Empires";
    
    let size = Vector{
        x: DEFAULT_RESOLUTION.w as f32,
        y: DEFAULT_RESOLUTION.h as f32
    };

    let settings = Settings {
        draw_rate: 30.,
        update_rate: 30.,
        vsync: false,
        ..Settings::default()
    };

    run::<Canvas>(title, size, settings);
}
