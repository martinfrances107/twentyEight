use super::cell::Cell;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::Document;
use web_sys::HtmlDivElement;

use super::pacman::PacMan;

// Initial grid layouto
static GRID_INIT: [&str; 29] = [
    r#"/------------¬/------------¬"#,
    r#"|oooooooooooo||oooooooooooo|"#,
    r#"|o/--¬o/---¬o||o/---¬o/--¬o|"#,
    r#"|P|++|o|+++|o||o|+++|o|++|P|"#,
    r#"|o\--,o\---,o\,o\---,o\--,o|"#,
    r#"|oooooooooooooooooooooooooo|"#,
    r#"|o/--¬o/¬o/------¬o/¬o/--¬o|"#,
    r#"|o\--,o||o\--¬/--,o||o\--,o|"#,
    r#"|oooooo||oooo||oooo||oooooo|"#,
    r#"\----¬o|\--¬ || /--,|o/----,"#,
    r#"+++++|o|/--, \, \--¬|o|+++++"#,
    r#"+++++|o||          ||o|+++++"#,
    r#"-----,o\, /--gG--¬ \,o\-----"#,
    r#"      o   |      |   o      "#,
    r#"-----¬o/¬ \------, /¬o/-----"#,
    r#"+++++|o||          ||o|+++++"#,
    r#"+++++|o|| /------¬ ||o|+++++"#,
    r#"/----,o\, \--¬/--, \,o\----¬"#,
    r#"|oooooooooooo||oooooooooooo|"#,
    r#"|o/--¬o/---¬o||o/---¬o/--¬o|"#,
    r#"|o\-¬|o\---,o\,o\---,o|/-,o|"#,
    r#"|Poo||oooooooooooooooo||ooP|"#,
    r#"\-¬o||o/¬o/------¬o/¬o||o/-,"#,
    r#"/-,o\,o||o\--¬/--,o||o\,o\-¬"#,
    r#"|oooooo||oooo||oooo\,oooooo|"#,
    r#"|o/----,\--¬o||o/--------¬o|"#,
    r#"|o\--------,o\,o\--------,o|"#,
    r#"|oooooooooooooooooooooooooo|"#,
    r#"\--------------------------,"#,
];

#[derive(Debug, PartialEq)]
pub(crate) struct Game {
    pub(crate) grid: Grid,
    pub(crate) pac_man: PacMan,
    // TODO add monster state here.
}

#[derive(Debug, PartialEq)]
pub(crate) struct Grid([Cell; 28 * 29]);

impl Default for Game {
    fn default() -> Self {
        Self {
            grid: GRID_INIT.into(),
            pac_man: PacMan::default(),
        }
    }
}

impl From<[&str; 29]> for Grid {
    fn from(value: [&str; 29]) -> Self {
        let mut grid: [Cell; 28 * 29] = [Cell::Blank; 28 * 29];

        // i - row identifiero
        // j - column identifier
        for (i, row) in value.into_iter().enumerate() {
            for (j, cell_u8) in row.chars().enumerate() {
                let index = i * 28 + j;
                grid[index] = cell_u8.into()
            }
        }

        Grid(grid)
    }
}

impl Game {
    // Populate HTML element with a grid of cells.
    pub fn generate(
        &self,
        document: &Document,
        game_element: HtmlDivElement,
    ) -> Result<(), JsValue> {
        log_1(&JsValue::from("generate"));
        for (i, cell) in self.grid.0.into_iter().enumerate() {
            let cell_element = document.create_element("div").unwrap();
            cell_element.set_id(&i.to_string());
            cell_element.set_class_name(&cell.to_string());
            game_element.append_child(&cell_element)?;
        }

        // Add actors.

        Ok(())
    }
}
