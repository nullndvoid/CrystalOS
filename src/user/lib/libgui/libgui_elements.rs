use super::libgui_core::{self, Pos};
use crate::std::io::println;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

// TEXT BOX
// a widget to display text in a box
// has a title and a body
pub struct TextBox {
    dimensions: Pos,
    position: Pos,
    content: String,
    title: String,
    outlined: bool,
}

// implements all rendering for TextBox widget
impl libgui_core::Element for TextBox {
    fn render(&self) -> (Vec<Vec<char>>, Pos) {
        let mut charmap = Vec::<Vec<char>>::new();
        let mut inner_dims = Pos::new(self.dimensions.x - 2, self.dimensions.y - 2);

        // generate outline if required
        if self.outlined {
            charmap = libgui_core::gen_outline(self.dimensions);
        }

        // render title
        let mut titlechars = self.title.chars().collect::<Vec<char>>();
        for (i, char) in titlechars.iter().enumerate() {
            if i < inner_dims.x {
                charmap[0][i + 1] = *char;
            } else {
                charmap[0][inner_dims.x - 0] = '.';
                charmap[0][inner_dims.x - 1] = '.';
                charmap[0][inner_dims.x - 2] = '.';
                break;
            }
        }

        // render text
        let mut pos = Pos::new(0, 0);

        for chr in self.content.chars().collect::<Vec<char>>() {
            if pos.x < inner_dims.x {
                if chr != '\n' {
                    charmap[pos.y + 1][pos.x + 1] = chr;
                    pos.x += 1;
                } else {
                    pos.y += 1;
                    pos.x = 0;
                }
            } else {
                // next line
                pos.y += 1;
                pos.x = 1;

                if pos.y < inner_dims.y {
                    charmap[pos.y + 1][1] = chr;
                } else {
                    // handles overflow out of the end of the box
                    charmap[inner_dims.y][inner_dims.x] = '»';
                    charmap[inner_dims.y + 1][inner_dims.x - 0] = '.';
                    charmap[inner_dims.y + 1][inner_dims.x - 1] = '.';
                    charmap[inner_dims.y + 1][inner_dims.x - 2] = '.';
                    break;
                }
            }
        }

        return (charmap, self.position);
    }
}

impl TextBox {
    pub fn new(
        title: String,
        content: String,
        dimensions: Pos,
        position: Pos,
        outlined: bool,
    ) -> TextBox {
        TextBox {
            dimensions,
            position,
            content,
            title,
            outlined,
        }
    }
}

struct IndicatorBox {
    pub bars: Vec<libgui_core::IndicatorBar>,
    position: Pos,
    dimensions: Pos,
}
impl IndicatorBox {
    pub fn new(position: Pos, dimensions: Pos) -> IndicatorBox {
        Self {
            bars: Vec::new(),
            position,
            dimensions,
        }
    }
    pub fn add_item(&mut self) {}
}
impl libgui_core::Element for IndicatorBox {
    fn render(&self) -> (Vec<Vec<char>>, Pos) {
        (Vec::<Vec<char>>::new(), Pos::new(0, 0))
    }
}
