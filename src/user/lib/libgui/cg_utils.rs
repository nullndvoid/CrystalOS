use crate::std::frame::{ColouredChar, Dimensions, Frame, Position};

pub(crate) fn render_outline(frame: &mut Frame, dimensions: Dimensions) {
    // draws the sides of the container
    for i in 0..frame.dimensions.x {
        frame.write(Position::new(i, 0), ColouredChar::new('─'));
        frame.write(Position::new(i, frame.dimensions.y - 1), ColouredChar::new('─'));
    }

    // draws the top and bottom of the container
    for i in 0..frame.dimensions.y {
        frame.write(Position::new(0, i), ColouredChar::new('│'));
        frame.write(Position::new(frame.dimensions.x - 1, i), ColouredChar::new('│'));
    }

    // draws the corners of the container
    frame.write(Position::new(0, 0), ColouredChar::new('┌'));
    frame.write(Position::new(dimensions.x - 1, 0), ColouredChar::new('┐'));
    frame.write(Position::new(0, dimensions.y - 1), ColouredChar::new('└'));
    frame.write(Position::new(dimensions.x - 1, dimensions.y - 1), ColouredChar::new('┘'));
}
