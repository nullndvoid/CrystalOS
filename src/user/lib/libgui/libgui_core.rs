use crate::kernel::render::{BUFFER_HEIGHT, BUFFER_WIDTH, RENDERER};
use crate::std::io::Frame;
use crate::{print, println};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
/*

- this library will provide useful structures for creating simple
  command line based interfaces using ascii

*/

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}
impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

/// all interface elements must implement this trait in order to be
/// rendered on the screen
pub trait Element {
    // default behaviour for all elements

    fn render(&self) -> (Vec<Vec<char>>, Pos) {
        // recursive method for rendering the
        // specified frame to the screen
        // insert rendering code for specific frame here
        // this should also render all children of the element
        (Vec::<Vec<char>>::new(), Pos::new(0, 0))
    }
}

pub struct Container<'a> {
    // a simple container objects for grouping
    // other containers together
    frame: Vec<Vec<char>>,
    elements: Vec<Box<&'a dyn Element>>,
    position: Pos, // x,y
    //
    outlined: bool,
    dimensions: Pos, // x,y
}

impl Container<'a> {
    fn new(position: Pos, dimensions: Pos, outlined: bool) -> Container<'a> {
        Self {
            frame: vec![vec![' '; dimensions.x as usize]; dimensions.y as usize],
            elements: Vec::new(),
            position,
            outlined,
            dimensions,
        }
    }
}

impl Element for Container<'_> {
    fn render(&self) -> (Vec<Vec<char>>, Pos) {
        // returns all elements as a single frame

        let mut charmap = Vec::<Vec<char>>::new();

        let mut frstline: Vec<char>;
        let mut midlines: Vec<char>;
        let mut lastline: Vec<char>;

        if self.outlined {
            charmap = gen_outline(self.dimensions);
        }

        // render child elements

        for element in &self.elements {
            let r = (*element).render();

            // rendering code for child elements goes here

            // code to render the object at the position marked by offset within the container

            for (i, row) in r.0.iter().enumerate() {
                for (j, chr) in row.iter().enumerate() {
                    // r.0 is the rendered element
                    // r.1.0 is the x offset
                    charmap[i + r.1.y][j + r.1.x] = *chr; // r.1.1 is the y offset
                }
            }
        }

        return (charmap, self.position);
    }
}

pub struct IndicatorBar {
    length: usize,
    filled: usize,
    abs: usize,
    position: Pos,
    text: Option<String>,
}

impl IndicatorBar {
    fn new(position: Pos, length: usize) -> IndicatorBar {
        IndicatorBar {
            position,
            length,
            abs: 0,
            filled: 0,
            text: None,
        }
    }
    fn set_value(&mut self, value: usize) {
        // takes a value from 1-100%
        // and turns it into a corresponding length filled
        self.filled = value;
    }
    fn set_text(&mut self, s: String) {
        self.text = Some(s);
    }
}

impl Element for IndicatorBar {
    fn render(&self) -> (Vec<Vec<char>>, Pos) {
        let numlen = (self.abs.to_string().as_str()).len();
        let relfilled = (self.filled as f64 / 100.0 * ((self.length - numlen) as f64)) as usize;

        let mut line = Vec::<char>::new();
        if let Some(t) = &self.text {
            line.append(&mut t.chars().collect());
            line.push(':');
            line.push(' ');
        }
        line.append(&mut (self.abs.to_string().chars().collect()));
        line.append(&mut vec!['▓'; relfilled]);
        line.append(&mut vec!['░'; self.length - numlen - relfilled]);

        let mut rendered = Vec::new();
        rendered.push(line);

        println!("RENDERED: {:?}", rendered);

        return (rendered, (self.position));
    }
}

// functions that deal with the rendering and interaction between objects being
// rendered.

pub fn render_frame(elements: Vec<Container>) {
    let mut buffer: Frame = [[' '; BUFFER_WIDTH]; BUFFER_HEIGHT];

    for frame in elements.iter() {
        let f = frame.render();

        for (i, row) in f.0.iter().enumerate() {
            for (j, chr) in row.iter().enumerate() {
                let mut current = &buffer[i + f.1.y][j + f.1.x];
                let newchar = overlap_check(*current, *chr);
                buffer[i + f.1.y][j + f.1.x] = newchar;

                //print!("{}", buffer[i+frame.position.1][j+frame.position.0]);
            }
        }
    }

    //println!("{:?}", buffer);

    RENDERER.lock().render_frame(buffer)
}

pub fn overlap_check(oldchar: char, newchar: char) -> char {
    match (oldchar, newchar) {
        //┌│└ ┐┘─
        ('│', '─') | ('┌', '┘') | ('└', '┐') => '┼',
        ('┌', '└') => '├',
        ('┐', '┐') => '┤',

        (_, _) => newchar,
    }
}

// function to return a charmap of the outline of an object

pub fn gen_outline(dimensions: Pos) -> Vec<Vec<char>> {
    let mut charmap = Vec::<Vec<char>>::new();

    let mut frstline = vec!['┌'];
    let mut midlines = vec!['│'];
    let mut lastline = vec!['└'];

    frstline.append(&mut vec!['─'; dimensions.x - 2]);
    midlines.append(&mut vec![' '; dimensions.x - 2]);
    lastline.append(&mut vec!['─'; dimensions.x - 2]);

    frstline.append(&mut vec!['┐']);
    midlines.append(&mut vec!['│']);
    lastline.append(&mut vec!['┘']);

    charmap.push(frstline);
    for _ in 0..dimensions.y - 2 {
        charmap.push(midlines.clone());
    }
    charmap.push(lastline);

    return charmap;
}

// testing functions

pub fn test_elements() {
    use super::libgui_elements;

    let mut containers = Vec::<Container>::new();

    /*

    //for _ in 0..10 {
    //    containers.push(generate_box());
    //}

    containers.push(Container::new((5, 5), (15, 5), true));
    containers.push(Container::new((10, 3), (50, 20), true));

    let mut bar = IndicatorBar::new((10, 6), 12);
    let mut bar2 = IndicatorBar::new((10, 7), 12);

    bar.set_value(43);
    bar.abs = 101;
    bar2.set_value(14);
    bar2.abs = 15;
    containers[1].elements.push(Box::new(bar));
    containers[1].elements.push(Box::new(bar2));

    let tbox = libgui_elements::TextBox::new(
        String::from("panic attack simps"),
        String::from("i have finally obtained evidence of his simpiness against tari and crystal, however i cannot reveal this evidence for now, however, once the contract is over NO ONE CAN STOP ME MWHAHAHAHAHA"),
        Pos::new(25, 10),
        Pos::new(10, 9),
        true,
    );

    containers[1].elements.push(Box::new(tbox));

    */

    containers.push(Container::new(Pos::new(0, 1), Pos::new(80, 24), true));

    let tbox = libgui_elements::TextBox::new(
        String::from("ANNOUNCEMENTS"),
        String::from(
            "CrystalRPG coming soon! XD
this is gonna be the best game ever",
        ),
        Pos::new(25, 10),
        Pos::new(0, 0),
        true,
    );

    containers[0].elements.push(Box::new(&tbox));

    render_frame(containers);

    return;
}

// function to generate a box in a random location on the screen.

fn generate_box() -> Container<'static> {
    use crate::std::random::Random;
    let width = Random::int(5, 20);
    let height = Random::int(5, 10);
    let xoffset = Random::int(5, 50);
    let yoffset = Random::int(5, 10);
    Container::new(Pos::new(width, height), Pos::new(xoffset, yoffset), true)
}
