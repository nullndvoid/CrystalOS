// use async_trait::async_trait;
// use alloc::boxed::Box;
// use alloc::string::String;
// use alloc::vec;
// use alloc::vec::Vec;
// use crate::std::frame::{ColouredChar};
// use crate::{serial_print, serial_println};
// use crate::std::application::{Application, Error};
// use crate::std::io::{Display, Screen};
// use crate::user::lib::coords::{Direction, Position, PositionReal};
//
//
// pub(crate) struct TetrisEngine {
//     score: u32,
//     next: TetrisPiece,
//     completed_frame: [[ColouredChar; 80]; 25], // this frame does not contain falling blocks, only static ones
//
// }
//
// #[async_trait]
// impl Application for TetrisEngine {
//     fn new() -> Self {
//         Self {
//             score: 0,
//             next: TetrisPiece::new(PieceType::OPiece),
//             completed_frame: [[ColouredChar::null(); 80]; 25],
//         }
//     }
//     async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {
//         // setup:
//         let d = Display::borrow();
//
//         let piece_type = PieceType::OPiece;
//         let mut piece = TetrisPiece::new(piece_type);
//
//         serial_println!("{:?}", piece.get_positions());
//         piece.rotate_right();
//         serial_println!("{:?}", piece.get_positions());
//
//         Ok(())
//     }
// }
//
//
//
//
//
//
//
//
//
//
//
// enum PieceType {
//     OPiece,
//     IPiece,
//     JPiece,
//     LPiece,
//     SPiece,
//     ZPiece,
// }
//
// struct TetrisPiece {
//     type_: PieceType,
//     pos: Position,
//     rotation: Direction,
// }
//
// impl TetrisPiece {
//     fn new(type_: PieceType) -> Self {
//         Self {
//             type_,
//             pos: Position { x: 40, y: 30 },
//             rotation: Direction::Degrees0,
//         }
//     }
//     fn rotate_right(&mut self) {
//         self.rotation = match self.rotation {
//             Direction::Degrees90 => Direction::Degrees180,
//             Direction::Degrees180 => Direction::Degrees270,
//             Direction::Degrees270 => Direction::Degrees0,
//             Direction::Degrees0 => Direction::Degrees90,
//             Direction::None => panic!("direction should never be none in this application"),
//         };
//     }
//
//     /// function that maps the coordinates of the object.
//     fn get_positions(&self) -> Vec<Position> {
//         match self.type_ {
//             PieceType::OPiece => {
//                 let positions = vec![
//                     PositionReal { x: -0.5, y: -0.5 },
//                     PositionReal { x: 0.5, y: -0.5 },
//                     PositionReal { x: -0.5, y: 0.5 },
//                     PositionReal { x: 0.5, y: 0.5 },
//                 ];
//                 positions.into_iter().map(|p|
//                     ( p.rotate(self.rotation.clone()) + self.pos.clone().real() + PositionReal { x: -0.5, y: 0.5 } ).integer()
//                 ).collect::<Vec<Position>>()
//             }
//             _ => unimplemented!("E"),
//         }
//     }
// }
//
//
//
//
//
//
//
//
