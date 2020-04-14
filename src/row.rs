use rawcmd_utils::{pad_text};
use crate::{Cell};

/// Row structure which represents multiple formated columns.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    cells: Vec<Cell>,
}

/// Style structure implementation.
impl Row {

    /// Returns new instance.
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
        }
    }

    /// Adds new row column.
    pub fn add_cell(mut self, cell: Cell) -> Self {
        self.cells.push(cell);
        self
    }

    /// Returns a formatted row content as multiline string.
    pub fn build_rows(&self) -> Vec<String> {
        let mut result = Vec::new();

        let xcount = match self.cells.len() {
            0 => return result,
            v => v,
        };
        let ycount = match self.cells.iter().map(|c| c.build_rows().len()).max() {
            Some(v) => v,
            None => return result,
        };

        for _ in 0..ycount {
            result.push("".to_string());
        }

        for x in 0..xcount {
            let cell = self.cells.get(x).unwrap();
            let rows = &cell.build_rows();

            for y in 0..ycount {
                let row = rows.get(y);
                let text = match row {
                    Some(t) => t,
                    None => "",
                };
                let text = match cell.width() {
                    Some(v) => pad_text(text, *v, cell.text_align(), *cell.text_pad()),
                    None => text.to_string(),
                };
                result[y] = format!("{}{}", result[y], text);
            }
        }
    
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cell};

    #[test]
    fn builds_rows() {
        let cell0 = Cell::new()
            .set_width(30)
            .set_text("Allocating \x1B[31mmemory is actually quite fast, and regardless you’re going to be copying the entire\x1B[39m string around.")
            .set_text_pad('*');
        let cell1 = Cell::new()
            .set_width(3)
            .set_text_pad('|');
        let cell2 = Cell::new()
            .set_width(20)
            .set_text("Going 利干 to be the entire string around.")
            .set_text_pad('+');
        let row = super::Row::new()
            .add_cell(cell0)
            .add_cell(cell1)
            .add_cell(cell2);
        assert_eq!(row.build_rows(), [
            "Allocating \u{1b}[31mmemory is actually*\u{1b}[39m|||Going 利干 to be the",
            "\u{1b}[31mquite fast, and regardless****\u{1b}[39m|||entire string+++++++",
            "\u{1b}[31myou’re going to be copying****\u{1b}[39m|||around.+++++++++++++",
            "\u{1b}[31mthe entire\u{1b}[39m string around.*****|||++++++++++++++++++++",
        ]);
    }
}
