use ansistr::{pad_str};
use crate::{Column};

/// Row structure which represents multiple formated columns.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    columns: Vec<Column>,
}

/// Style structure implementation.
impl Row {

    /// Returns new instance.
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    /// Adds new row column.
    pub fn add_column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    /// Returns a formatted row content as multiline string.
    pub fn to_string(&self) -> String {
        let mut result = Vec::new();

        let xcount = match self.columns.len() {
            0 => return "".to_string(),
            v => v,
        };
        let ycount = match self.columns.iter()
            .map(|c| c.to_string().matches("\n").count())
            .max() {
            Some(v) => v,
            None => return "".to_string(),
        };

        for _ in 0..ycount {
            result.push("".to_string());
        }

        for x in 0..xcount {
            let column = self.columns.get(x).unwrap();
            let rows: Vec<String> = column.to_string()
                .split("\n")
                .map(|c| c.to_string())
                .collect();

            for y in 0..ycount {
                let row = rows.get(y);
                let text = match row {
                    Some(t) => t,
                    None => "",
                };
                let text = match column.width() {
                    Some(v) => pad_str(text, *v, column.text_align(), column.text_pad()),
                    None => text.to_string(),
                };
                result[y] = format!("{}{}", result[y], text);
            }
        }
    
        format!("{}\n", result.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Column};

    #[test]
    fn builds_multiline_string() {
        let column0 = Column::new()
            .set_width(30)
            .set_text("Allocating \x1B[31mmemory is actually quite fast, and regardless you’re going to be copying the entire\x1B[39m string around.")
            .set_text_pad("*");
        let column1 = Column::new()
            .set_width(3)
            .set_text_pad("|");
        let column2 = Column::new()
            .set_width(20)
            .set_text("Going 利干 to be the entire string around.")
            .set_text_pad("+");
        let row = super::Row::new()
            .add_column(column0)
            .add_column(column1)
            .add_column(column2);
        assert_eq!(row.to_string(), vec![
            "Allocating \u{1b}[31mmemory is actually*\u{1b}[39m|||Going 利干 to be the\n",
            "\u{1b}[31mquite fast, and regardless****\u{1b}[39m|||entire string+++++++\n",
            "\u{1b}[31myou’re going to be copying****\u{1b}[39m|||around.+++++++++++++\n",
            "\u{1b}[31mthe entire\u{1b}[39m string around.*****|||++++++++++++++++++++\n",
        ].join(""));
    }
}
