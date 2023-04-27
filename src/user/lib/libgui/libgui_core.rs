use crate::std::io::Frame;
use alloc::{vec::Vec, vec, boxed::Box, string::ToString};
use crate::kernel::render::{BUFFER_WIDTH, BUFFER_HEIGHT, RENDERER};
use crate::{println, print};
/*

- this library will provide useful structures for creating simple
  command line based interfaces using ascii

*/



/// all interface elements must implement this trait in order to be
/// rendered on the screen
trait Element { // default behaviour for all elements
    
    fn render(&self) -> (Vec<Vec<char>>, (usize, usize)) { // recursive method for rendering the 
                       // specified frame to the screen
        // insert rendering code for specific frame here
        // this should also render all children of the element
        (Vec::<Vec<char>>::new(), (0, 0))
    }
    
}



pub struct Container { // a simple container objects for grouping
                       // other containers together
    frame: Vec<Vec<char>>,
    elements: Vec<Box<dyn Element>>,
    position: (usize, usize), // x,y
    //
    outlined: bool,
    dimensions: (usize, usize), // x,y
}

impl Container {
    
    fn new(position: (usize, usize), dimensions: (usize, usize), outlined: bool) -> Container {
        Self { 
            frame: vec![vec![' '; dimensions.0 as usize]; dimensions.1 as usize],
            elements: Vec::new(),
            position,
            outlined,
            dimensions,
        }
    }
    fn place(&self, element: Vec<Vec<char>>) {
        return // unimplemented
    }
    
}

impl Element for Container {

    fn render(&self) -> (Vec<Vec<char>>, (usize, usize)) { // returns all elements as a single frame

        let mut charmap = Vec::<Vec<char>>::new();

        let mut frstline: Vec<char>;
        let mut midlines: Vec<char>;
        let mut lastline: Vec<char>;


        if self.outlined {
            frstline = vec!['┌'];
            midlines = vec!['│'];
            lastline = vec!['└'];

            frstline.append(&mut vec!['─'; self.dimensions.0 -2]);
            midlines.append(&mut vec![' '; self.dimensions.0 -2]);
            lastline.append(&mut vec!['─'; self.dimensions.0 -2]);
                    
            frstline.append(&mut vec!['┐']);
            midlines.append(&mut vec!['│']);
            lastline.append(&mut vec!['┘']);
        
        
            charmap.push(frstline);
            for _ in 0..self.dimensions.1 -2 {
                charmap.push(midlines.clone());
            }
            charmap.push(lastline);
        
        }
        
        // render child elements
        
        for element in &self.elements {
            let r = (*element).render();
            
            // rendering code for child elements goes here


            // code to render the object at the position marked by offset within the container

        
            for (i, row) in r.0.iter().enumerate() {
                for (j, chr) in row.iter().enumerate() {             // r.0 is the rendered element
                                                                     // r.1.0 is the x offset
                    charmap[i + r.1.1][j + r.1.0] = *chr;         // r.1.1 is the y offset
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
    position: (usize, usize),
}

impl IndicatorBar {
    fn new(position: (usize, usize), length: usize) -> IndicatorBar {
        IndicatorBar { position, length, abs: 0, filled: 0 }
    }
    fn set_value(&mut self, value: usize) { // takes a value from 1-100% 
        // and turns it into a corresponding length filled
        self.filled = value
    }
}

impl Element for IndicatorBar {
    fn render(&self) -> (Vec<Vec<char>>, (usize, usize)) {
        let numlen = (self.abs.to_string().as_str()).len();
        let relfilled = (self.filled as f64 / 100.0 * ((self.length - numlen) as f64)) as usize;
        
        let mut line = Vec::<char>::new();
        line.append(&mut (self.abs.to_string().chars().collect()));
        line.append(&mut vec!['▓'; relfilled]);
        line.append(&mut vec!['░'; self.length-numlen-relfilled]);


        let mut rendered = Vec::new();
        rendered.push(line);


        println!("RENDERED: {:?}", rendered);
        
        return (rendered, (self.position))
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
                let mut current = &buffer[i + f.1.1][j + f.1.0];
                let newchar = overlap_check(*current, *chr);
                buffer[i + f.1.1][j + f.1.0] = newchar;
                
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
        ('│', '─')|('┌', '┘')|('└', '┐') => '┼',
        ('┌', '└') => '├',
        ('┐', '┐') => '┤',

        (_, _) => newchar
    }
}

// function to return a charmap of the outline of an object

fn gen_outline(dimensions: (usize, usize)) -> Vec<Vec<char>> {
    let mut charmap = Vec::<Vec<char>>::new();
    
    let mut frstline = vec!['┌'];
    let mut midlines = vec!['│'];
    let mut lastline = vec!['└'];

    frstline.append(&mut vec!['─'; dimensions.0 -2]);
    midlines.append(&mut vec![' '; dimensions.0 -2]);
    lastline.append(&mut vec!['─'; dimensions.0 -2]);
                    
    frstline.append(&mut vec!['┐']);
    midlines.append(&mut vec!['│']);
    lastline.append(&mut vec!['┘']);
        
        
    charmap.push(frstline);
    for _ in 0..dimensions.1 -2 {
        charmap.push(midlines.clone());
    }
    charmap.push(lastline);

    return charmap
}












// testing functions

pub fn test_elements() {
    println!("e");

    let mut containers = Vec::<Container>::new();

    //for _ in 0..10 {
    //    containers.push(generate_box());
    //}

    containers.push(Container::new((5, 5), (15, 5), true));
    containers.push(Container::new((10, 3), (50, 15), true));

    let mut bar = IndicatorBar::new((10, 10), 12);
    let mut bar2 = IndicatorBar::new((10, 11), 12);
    
    
    bar.set_value(43);
    bar.abs = 101;
    bar2.set_value(14);
    bar2.abs= 15;
    containers[1].elements.push(Box::new(bar));
    containers[1].elements.push(Box::new(bar2));
    
    
    render_frame(containers);
    return 
}

// function to generate a box in a random location on the screen.

fn generate_box() -> Container {
    use crate::std::random::Random;
    let width = Random::int(5, 20);
    let height = Random::int(5, 10);
    let xoffset = Random::int(5, 50);
    let yoffset = Random::int(5, 10);
    Container::new((width, height), (xoffset, yoffset), true)
}