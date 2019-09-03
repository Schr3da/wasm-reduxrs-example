extern crate quicksilver;
extern crate common;

mod entry;

use quicksilver::geom::Vector;
use quicksilver::lifecycle::{Settings, run};

use entry::{Canvas};

fn main() {
    
    common::init();
    
    let title = "War of Empires";
    let size = Vector::new(800, 600);
    let settings = Settings::default();
    run::<Canvas>(title, size, settings);
}
