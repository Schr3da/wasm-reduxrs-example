use web_sys::CanvasRenderingContext2d;

use crate::core::reducers::state::State;

pub fn draw_world(context: &CanvasRenderingContext2d, state: &State) {
    for tiles in state.next.game.views.values() {
        for tile in tiles {
            match tile {
                Some(t) => {
                    context.begin_path();
                    context.set_fill_style(&"transparent".into());
                    context.fill_rect(
                        t.position.x as f64,
                        t.position.y as f64,
                        t.size.w as f64,
                        t.size.h as f64,
                    );
                    context.fill();

                    context.set_fill_style(&"black".into());
                    context.set_stroke_style(&"blue".into());
                    context.set_text_align(&"center");
                    context.set_font(&"12px Arial");
                    context
                        .fill_text(
                            &t.symbol.to_string(),
                            t.position.x as f64 + 8.0,
                            t.position.y as f64 + 12.0,
                        )
                        .unwrap();
                }
                _ => println!("not a valid tile"),
            };
        }
    }
}
