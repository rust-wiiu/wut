// position

#[derive(Debug, Default, Copy, Clone)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Default)]
pub struct TextPosition {
    pub column: u32,
    pub row: u32,
    pub align: TextAlign,
}

impl Into<TextPosition> for (u32, u32) {
    fn into(self) -> TextPosition {
        TextPosition {
            column: self.0,
            row: self.1,
            align: TextAlign::default(),
        }
    }
}

impl Into<TextPosition> for (u32, u32, TextAlign) {
    fn into(self) -> TextPosition {
        TextPosition {
            column: self.0,
            row: self.1,
            align: self.2,
        }
    }
}

impl Into<TextPosition> for (f32, f32) {
    fn into(self) -> TextPosition {
        todo!()
    }
}

impl Into<TextPosition> for (f32, f32, TextAlign) {
    fn into(self) -> TextPosition {
        todo!()
    }
}

impl TextPosition {
    pub fn format<'a>(&self, text: &'a str) -> impl Iterator<Item = (&'a str, u32, u32)> {
        let mut row = self.row;
        let column = self.column;
        let align = self.align;

        text.split('\n')
            .map(move |line| {
                //
                let col = match align {
                    TextAlign::Left => column,
                    TextAlign::Center => column.saturating_sub(line.len() as u32 / 2),
                    TextAlign::Right => column.saturating_sub(line.len() as u32),
                };
                row += 1;
                //
                (line, col, row - 1)
            })
            .into_iter()
    }
}
