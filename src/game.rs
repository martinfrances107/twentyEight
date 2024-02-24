use super::cell::Cell;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::Document;
use web_sys::HtmlDivElement;

// Initial grid layouto
static GRID_INIT: [&str; 30] = [
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
    r#"+++++|o||          ||o|+++++"#,
    r#"-----,o\, LL    LL \,o\-----"#,
    r#"      o   L      L   o      "#,
    r#"-----¬o/¬ LLLLLLLL /¬o/-----"#,
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
    pub(crate) grid: [Cell; 28 * 30],
}

impl Default for Game {
    fn default() -> Self {
        GRID_INIT.into()
    }
}

impl From<[&str; 30]> for Game {
    fn from(value: [&str; 30]) -> Self {
        let mut grid: [Cell; 28 * 30] = [Cell::Blank; 28 * 30];

        // i - row identifiero
        // j - column identifier
        for (i, row) in value.into_iter().enumerate() {
            for (j, cell_u8) in row.chars().enumerate() {
                let index = i * 28 + j;
                grid[index] = cell_u8.into()
            }
        }

        Self { grid }
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
        for (i, cell) in self.grid.into_iter().enumerate() {
            let cell_element = document.create_element("div").unwrap();
            cell_element.set_id(&i.to_string());
            cell_element.set_class_name(&cell.to_string());
            game_element.append_child(&cell_element)?;
        }
        Ok(())
    }
}
