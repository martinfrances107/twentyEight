mod cell;
mod game;
mod pacman;
mod wall;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::HtmlDivElement;

use crate::game::Game;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    log(&"main run() entry");
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let game_element: HtmlDivElement = document
        .get_element_by_id("game")
        .unwrap()
        .dyn_into::<web_sys::HtmlDivElement>()?;

    // Manufacture the element we're gonna append

    // Initialise game board.
    let game = Game::default();
    let _ = game.generate(&document, game_element);

    // Spawn actors.

    // Attach listeners.
    log(&"main exit");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::Cell;
    use crate::game::Game;

    #[test]
    fn tiny_grid() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        // let input_grid: [[u8; 4]; 1] = [[0u8, 1u8, 2u8, 3u8]];
        // let game: Game<4, 1> = input_grid.into();
        // assert_eq!(
        //     game,
        //     Game {
        //         grid: [Cell::Dot, Cell::Empty, Cell::GhostLair, Cell::PowerPellet]
        //     }
        // )
    }
}
