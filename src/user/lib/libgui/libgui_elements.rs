use super::libgui_core::{self, Pos};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use crate::std::io::println;

// TEXT BOX

pub struct TextBox {
    dimensions: Pos,
    position: Pos,
    content: String,
    title: String,
    outlined: bool,
}

impl libgui_core::Element for TextBox {
    fn render(&self) -> (Vec<Vec<char>>, (usize, usize)) {
        let mut charmap = Vec::<Vec<char>>::new();

        let mut inner_dims = Pos::new(self.dimensions.x -2, self.dimensions.y -2);

        if self.outlined {
            charmap = libgui_core::gen_outline(self.dimensions);
        }


        let mut titlechars = self.title.chars().collect::<Vec<char>>();
        // render title

        for (i, char) in titlechars.iter().enumerate() {
            if i < inner_dims.x {
                charmap[0][i + 1] = *char;
            } else {
                break;
            }
        }

        let mut idx = 0;

        // render text

        let mut pos = Pos::new(0,0);

        for chr in self.content.chars().collect::<Vec<char>>() {
            
            if pos.x < inner_dims.x {
                charmap[pos.y + 1][pos.x + 1] = chr;
                pos.x += 1;
            } else {
                pos.y += 1;
                pos.x = 1;

                if pos.y < inner_dims.y {
                    charmap[pos.y + 1][1] = chr;
                } else {
                    charmap[inner_dims.y][inner_dims.x] = '.';
                    charmap[inner_dims.y][inner_dims.x -1] = '.';
                    charmap[inner_dims.y][inner_dims.x -2] = '.';
                    break;
                }
            }
        }




        return (charmap, (self.position.x, self.position.y));
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
