use std::fmt;
use ansistr::{TextAlign, repaire_str, trucate_str, wrap_str, pad_str};

/// Column structure which represents a formatted column in a row.
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    width: Option<usize>,
    text: Option<String>,
    text_width: Option<usize>,
    text_align: Option<TextAlign>,
    text_tail: Option<String>,
    text_pad: Option<String>,
}

/// Style structure implementation.
impl Column {

    /// Returns new instance.
    pub fn new() -> Self {
        Self {
            width: None,
            text: None,
            text_width: None,
            text_align: None,
            text_tail: None,
            text_pad: None,
        }
    }

    /// Returns new instance.
    pub fn from_str<S: Into<String>>(txt: S) -> Self {
        Self {
            width: None,
            text: Some(txt.into()),
            text_width: None,
            text_align: None,
            text_tail: None,
            text_pad: None,
        }
    }

    /// Returns column width.
    pub fn width(&self) -> &Option<usize> {
        &self.width
    }

    /// Returns column width.
    pub fn text(&self) -> &Option<String> {
        &self.text
    }
    
    /// Returns text width
    pub fn text_width(&self) -> &Option<usize> {
        &self.text_width
    }

    /// Returns text width
    pub fn text_align(&self) -> &TextAlign {
        match &self.text_align {
            Some(t) => t,
            None => &TextAlign::Left,
        }
    }

    /// Returns truncation tail string.
    pub fn text_tail(&self) -> &str {
        match &self.text_tail {
            Some(t) => t,
            None => "...",
        }
    }

    /// Returns text pad character.
    pub fn text_pad(&self) -> &str {
        match &self.text_pad {
            Some(t) => t,
            None => &" ",
        }
    }

    /// Sets column width.
    pub fn set_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets column text.
    pub fn set_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }
    
    /// Sets column text width.
    pub fn set_text_width(mut self, width: usize) -> Self {
        self.text_width = Some(width);
        self
    }

    /// Sets column text alignement.
    pub fn set_text_align(mut self, align: TextAlign) -> Self {
        self.text_align = Some(align);
        self
    }

    /// Sets text truncation tail string.
    pub fn set_text_tail<S: Into<String>>(mut self, tail: S) -> Self {
        self.text_tail = Some(tail.into());
        self
    }

    /// Sets text pad character.
    pub fn set_text_pad<S: Into<String>>(mut self, pad: S) -> Self {
        self.text_pad = Some(pad.into());
        self
    }

    /// Returns a formatted column content as multiline string.
    pub fn build(&self) -> String {
        let mut text = match &self.text {
            Some(t) => t.to_string(),
            None => return "".to_string(),
        };
        if self.text_width.is_some() {
            text = trucate_str(text, self.text_width.unwrap(), self.text_align(), self.text_tail());
        }
        let text = if self.width.is_some() {
            let width = self.width.unwrap();
            wrap_str(text, width)
                .split("\n")
                .map(|r| pad_str(r, width, self.text_align(), self.text_pad()))
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            text
        };
        format!("{}\n", repaire_str(text))
    }
}

impl fmt::Display for Column {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.build())
    }
}

impl From<Column> for String {
    fn from(item: Column) -> String {
        item.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_multiline_string() {
        let column = Column::new()
            .set_text("Allocating memory \x1B[31mis actually quite fast, and regardless you’re going to be copying the entire\x1B[39m string around.")
            .set_width(30)
            .set_text_width(72)
            .set_text_align(TextAlign::Center)
            .set_text_tail("+++")
            .set_text_pad("!");
        assert_eq!(column.build(), [
            "Allocating memory \u{1b}[31mis actually!\u{1b}[39m\n",
            "\u{1b}[31m!quit+++be copying the entire\u{1b}[39m!\n",
            "!!!!!!!!!string aroun!!!!!!!!!\n",
        ].join(""));
    }

    #[test]
    fn converts_to_string() {
        fn convert<S: Into<String>>(txt: S) -> String {
            txt.into()
        }
        assert_eq!(convert(Column::new().set_text("foo")), "foo\n");
    }

}
