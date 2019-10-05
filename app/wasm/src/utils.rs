use web_sys::CanvasRenderingContext2d;

use crate::core::reducers::state::State;

pub fn draw_cursor(context: &CanvasRenderingContext2d, state: &State) {
    let tile_size = state.next.settings.default_tile_size;
    let cursor_position = state.next.game.cursor.position;

    context.save();
    context.set_stroke_style(&"blue".into());
    context.set_line_width(2.into());
    context.stroke_rect(
        (cursor_position.x * tile_size.w) as f64,
        (cursor_position.y * tile_size.h) as f64,
        tile_size.w as f64,
        tile_size.h as f64,
    );
    context.restore();
}

pub fn draw_world(context: &CanvasRenderingContext2d, state: &State) {
    let translation = state.next.game.translation;

    context.save();
    context
        .translate(-translation.x as f64, -translation.y as f64)
        .unwrap();

    for tiles in state.next.game.views.values() {
        for tile in tiles {
            match tile {
                Some(t) => {
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
    context.restore();
}
