use super::libgui_core;

pub struct TextBox {
    dimensions: (usize, usize),
    position: (usize, usize),
    content: String,
    title: String,
    outlined: bool,
}

impl Element for TextBox {
    fn render(&self) -> (Vec<Vec<char>>, (usize, usize)) {

        let mut charmap = Vec::<Vec<char>>::new();

        if self.outlined {
            charmap = libgui_core::gen_outline(self.dimensions);
        }

        let inner_dims = (self.dimensions.0 -2, self.dimensions.1 -2);
        
        let mut titlechars = title.chars().collect::<Vec<char>>();
        // render title

        for (i, char) in titlechars.enumerate() {
            
        }


        let mut idx = 0

        while idx < content.len()


        return (charmap, self.position)
    }
}