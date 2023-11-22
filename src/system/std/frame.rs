use alloc::string::String;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use crate::kernel::render::{BUFFER_HEIGHT, BUFFER_WIDTH, RENDERER, ScreenChar};
use crate::println;
use spin::Mutex;

/// TODO: get a working implementation for CLI apps
/// elements can be created using their from_str() method
/// you can then render the element to the current frame using the render() method
/// the position of the element by passing a tuple (x,y) to render()
///
/// nothing will appear on the screen until the frame is actually rendered by
/// the render_frame method on the renderer
///
pub type Frame = [ [ ScreenChar; BUFFER_WIDTH ]; BUFFER_HEIGHT];

#[derive(Clone)]
pub struct Element {
    frame: Vec<Vec<char>>,
    dimensions: (u8, u8)
}

impl Element {
    pub fn from_str(elemstr: String) -> Self {
        let mut element = Element { frame: Vec::<Vec<char>>::new(), dimensions: (0, 0) };

        for line in elemstr.split("\n") {
            let mut ln = Vec::<char>::new();
            for col in line.chars() {
                ln.push(col)
            };
            element.frame.push(ln);
        }

        for row in element.clone().frame {
            let n = row.len();
            if n > element.dimensions.0 as usize {
                element.dimensions.0 = n as u8;
            }
        }
        element
    }

    pub fn generate(frame: Vec::<Vec<char>>, dims: (u8, u8)) -> Self {
        Element { frame, dimensions: dims }
    }

    pub fn render(&mut self,  pos: (u8, u8)) { // x,y
        for (i, row) in self.frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                //println!("{} {} {}", i, j, col);
                FRAMEGEN.lock().frame[i + pos.1 as usize][j + pos.0 as usize] = ScreenChar::white(*col as u8);
            };
        }
        FRAMEGEN.lock().render_frame();
    }
}

#[derive(Clone)]
pub struct ColouredElement {
    pub frame: Vec<Vec<ScreenChar>>,
    pub dimensions: (u8, u8)
}

impl ColouredElement {
    pub fn from_str(elemstr: String) -> Self {
        let mut element = ColouredElement { frame: Vec::<Vec<ScreenChar>>::new(), dimensions: (0, 0) };

        for line in elemstr.split("\n") {
            let mut ln = Vec::<ScreenChar>::new();
            for col in line.chars() {
                ln.push(ScreenChar::white(col as u8))
            };
            element.frame.push(ln);
        }

        for row in element.clone().frame {
            let n = row.len();
            if n > element.dimensions.0 as usize {
                element.dimensions.0 = n as u8;
            }
        }
        element
    }

    pub fn generate(frame: Vec::<Vec<ScreenChar>>, dims: (u8, u8)) -> Self {
        ColouredElement { frame, dimensions: dims }
    }

    pub fn render(&mut self,  pos: (u8, u8)) -> Result<(), ()> { // x,y

        // this block returns an error if any characters will be drawn out of the bounds of the screen
        if self.dimensions.0 + pos.0 > BUFFER_WIDTH as u8 {
            return Err(());
        } else if self.dimensions.1 + pos.1 > BUFFER_HEIGHT as u8 {
            return Err(());
        } else if self.frame.len() != self.dimensions.1 as usize {
            return Err(())
        } else if self.frame.iter().map(|r| r.len()).max().ok_or_else(|| ())? > self.dimensions.0 as usize {
            return Err(())
        }

        for (i, row) in self.frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                //println!("{} {} {}", i, j, col);
                FRAMEGEN.lock().frame[i + pos.1 as usize][j + pos.0 as usize] = *col;
            };
        }
        FRAMEGEN.lock().render_frame();
        Ok(())
    }
}


#[derive(Clone, Copy)]
pub struct FrameGen {
    frame: Frame,
}


impl FrameGen {
    pub fn render_frame(&self) {
        RENDERER.lock().render_frame(self.frame)
    }

    fn new() -> Self {
        let mut frame: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT];
        Self { frame: Frame::from(frame) }
    }

    fn set_frame(&mut self, frame: Frame) {
        self.frame = frame;
    }

    pub fn get_frame(&self) -> &[ [ ScreenChar; BUFFER_WIDTH ]; BUFFER_HEIGHT] {
        &self.frame
    }
}

lazy_static! {
    pub static ref FRAMEGEN: Mutex<FrameGen> = Mutex::new(FrameGen::new() );
}


impl core::fmt::Display for FrameGen {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        println!(" ");
        for row in &self.frame {
            println!("{}", row.iter().map(|c| c.character as char ).collect::<String>());
        };
        Ok(())
    }
}
