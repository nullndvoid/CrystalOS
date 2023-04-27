use super::libgui_core;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

// TEXT BOX

pub struct TextBox {
    dimensions: (usize, usize),
    position: (usize, usize),
    content: String,
    title: String,
    outlined: bool,
}

impl libgui_core::Element for TextBox {
    fn render(&self) -> (Vec<Vec<char>>, (usize, usize)) {
        let mut charmap = Vec::<Vec<char>>::new();

        if self.outlined {
            charmap = libgui_core::gen_outline(self.dimensions);
        }

        let inner_dims = (self.dimensions.0 - 2, self.dimensions.1 - 2);

        let mut titlechars = self.title.chars().collect::<Vec<char>>();
        // render title

        for (i, char) in titlechars.iter().enumerate() {}

        let mut idx = 0;

        while idx < self.content.len() {
            ()
        }

        return (charmap, self.position);
    }
}

impl TextBox {
    pub fn new(
        title: String,
        content: String,
        dimensions: (usize, usize),
        position: (usize, usize),
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
