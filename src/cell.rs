use rawcmd_utils::{Alignment, repaire_text, trucate_text, wrap_text, pad_text};

/// Cell structure which represents a formatted column in a row.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    width: Option<usize>,
    text: Option<String>,
    text_width: Option<usize>,
    text_align: Option<Alignment>,
    text_tail: Option<String>,
    text_pad: Option<char>,
}

/// Style structure implementation.
impl Cell {

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

    /// Returns cell width.
    pub fn width(&self) -> &Option<usize> {
        &self.width
    }

    /// Returns cell width.
    pub fn text(&self) -> &Option<String> {
        &self.text
    }
    
    /// Returns text width
    pub fn text_width(&self) -> &Option<usize> {
        &self.text_width
    }

    /// Returns text width
    pub fn text_align(&self) -> &Alignment {
        match &self.text_align {
            Some(t) => t,
            None => &Alignment::Left,
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
    pub fn text_pad(&self) -> &char {
        match &self.text_pad {
            Some(t) => t,
            None => &' ',
        }
    }

    /// Sets cell width.
    pub fn set_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets cell text.
    pub fn set_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
    
    /// Sets cell text width.
    pub fn set_text_width(mut self, width: usize) -> Self {
        self.text_width = Some(width);
        self
    }

    /// Sets cell text alignement.
    pub fn set_text_align(mut self, align: Alignment) -> Self {
        self.text_align = Some(align);
        self
    }

    /// Sets text truncation tail string.
    pub fn set_text_tail(mut self, tail: &str) -> Self {
        self.text_tail = Some(tail.to_string());
        self
    }

    /// Sets text pad character.
    pub fn set_text_pad(mut self, pad: char) -> Self {
        self.text_pad = Some(pad);
        self
    }

    /// Returns a formatted cell content as multiline string.
    pub fn build_rows(&self) -> Vec<String> {
        let mut text = match &self.text {
            Some(t) => t.to_string(),
            None => return Vec::new(),
        };
        if self.text_width.is_some() {
            text = trucate_text(text.as_str(), self.text_width.unwrap(), self.text_align(), self.text_tail());
        }
        let text = if self.width.is_some() {
            let width = self.width.unwrap();
            wrap_text(text.as_str(), width).iter().map(|r| pad_text(r, width, self.text_align(), *self.text_pad())).collect()
        } else {
            vec![text]
        };
        repaire_text(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_rows() {
        let cell0 = Cell::new()
            .set_text("Allocating memory \x1B[31mis actually quite fast, and regardless you’re going to be copying the entire\x1B[39m string around.")
            .set_width(30)
            .set_text_width(72)
            .set_text_align(Alignment::Center)
            .set_text_tail("+++")
            .set_text_pad('!');
        assert_eq!(cell0.build_rows(), [
            "Allocating memory \u{1b}[31mis actually!\u{1b}[39m",
            "\u{1b}[31m!quit+++be copying the entire\u{1b}[39m!",
            "!!!!!!!!!string aroun!!!!!!!!!",
        ]);
    }
}
