use alloc::vec::Vec;

use super::game::Cell;
use crate::serial_println;
use crate::std::random::Random;

#[derive(Debug, Clone)]
pub struct MoveQueue {
    points: Vec<(i32, i32)>,
}

impl MoveQueue {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.points.len() == 0
    }

    fn current(&self) -> Option<(i32, i32)> {
        self.points.first().cloned()
    }

    fn next(&mut self) -> Option<(i32, i32)> {
        if let Some(point) = self.current() {
            self.points.remove(0);
            self.current()
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.points.clear();
    }

    fn push(&mut self, point: (i32, i32)) {
        self.points.push(point);
    }
}

#[derive(Debug, Clone)]
pub enum AiBehavior {
    Expand,
    Hunt,
    Defend,
    Escape,
}

#[derive(Clone)]
pub struct Player {
    pub id: u8,
    pub alive: bool,
    pub position: (i32, i32),
    pub ai_direction: (i32, i32), // Current direction for AI
    pub ai_controlled: bool,      // Whether this player is AI controlled
    pub ai_behavior: AiBehavior,  // Current AI behavior state
    pub moves: MoveQueue,
}

impl Player {
    const DIRS: [(i32, i32); 8] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0), // Cardinal
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1), // Diagonal
    ];

    pub fn new(id: u8, position: (i32, i32), ai_controlled: bool) -> Self {
        // Set initial direction based on position to encourage better expansion
        let ai_direction = if position.0 < 40 {
            (1, 0) // If on left side, move right
        } else {
            (-1, 0) // If on right side, move left
        };

        Self {
            id,
            alive: true,
            position,
            ai_direction,
            ai_controlled,
            ai_behavior: AiBehavior::Expand,
            moves: MoveQueue::new(),
        }
    }

    pub fn update_ai(&mut self, board: &[[Cell; 80]; 25]) -> (i32, i32) {
        if !self.ai_controlled || !self.alive {
            return (0, 0);
        }

        match self.ai_behavior {
            AiBehavior::Expand => self.run_expand_behavior(board),
            AiBehavior::Hunt => self.run_hunt_behavior(board),
            AiBehavior::Defend => self.run_defend_behavior(board),
            AiBehavior::Escape => self.run_escape_behavior(board),
        }
    }

    // BEHAVIOR METHOD
    fn run_expand_behavior(&mut self, board: &[[Cell; 80]; 25]) -> (i32, i32) {
        if self.moves.is_empty() {
            let points = self.generate_expand_points(board);
            for point in points {
                self.moves.push(point);
            }
        }

        if let Some(target) = self.moves.current() {
            if self.position == target {
                // Only get next point if we've reached the current target
                self.moves.next();

                // If we've completed all points, generate new ones
                if self.moves.is_empty() {
                    let points = self.generate_expand_points(board);
                    for point in points {
                        self.moves.push(point);
                    }
                }
            }
        }

        // Get current target and move towards it
        self.moves
            .current()
            .map(|target| self.get_move_to_position(target))
            .unwrap_or((0, 0))
    }

    fn run_hunt_behavior(&mut self, board: &[[Cell; 80]; 25]) -> (i32, i32) {
        if let Some(tail_pos) = self.find_nearest_enemy_tail(board, 20) {
            self.get_move_to_position(tail_pos)
        } else if let Some(territory_pos) = self.find_nearest_territory_point(board, self.position)
        {
            self.get_move_to_position(territory_pos)
        } else {
            (0, 0)
        }
    }

    fn run_defend_behavior(&mut self, board: &[[Cell; 80]; 25]) -> (i32, i32) {
        if let Some(territory_pos) = self.find_nearest_territory_point(board, self.position) {
            self.get_move_to_position(territory_pos)
        } else {
            (0, 0)
        }
    }

    fn run_escape_behavior(&mut self, board: &[[Cell; 80]; 25]) -> (i32, i32) {
        if let Some(territory_pos) = self.find_nearest_territory_point(board, self.position) {
            self.get_move_to_position(territory_pos)
        } else {
            // Move away from center if no territory found
            let (x, y) = self.position;
            let center = (40, 12);
            let away_x = if x < center.0 { -1 } else { 1 };
            let away_y = if y < center.1 { -1 } else { 1 };
            (away_x, away_y)
        }
    }

    //
    // HELPER FUNCTIONS
    //

    // Calculate Manhattan distance to nearest owned territory
    fn distance_to_territory(&self, board: &[[Cell; 80]; 25]) -> i32 {
        let (x, y) = self.position;
        let mut min_dist = i32::MAX;

        for (y2, row) in board.iter().enumerate() {
            for (x2, cell) in row.iter().enumerate() {
                if let Cell::Solid(id, _) = cell {
                    if *id == self.id {
                        let dist = (x2 as i32 - x).abs() + (y2 as i32 - y).abs();
                        min_dist = min_dist.min(dist);
                    }
                }
            }
        }

        min_dist
    }

    // Find nearest territory point to a given position
    fn find_nearest_territory_point(
        &self,
        board: &[[Cell; 80]; 25],
        from: (i32, i32),
    ) -> Option<(i32, i32)> {
        let (x, y) = from;
        let mut nearest_point = None;
        let mut min_dist = i32::MAX;

        for (y2, row) in board.iter().enumerate() {
            for (x2, cell) in row.iter().enumerate() {
                if let Cell::Solid(id, _) = cell {
                    if *id == self.id {
                        let dist = (x2 as i32 - x).abs() + (y2 as i32 - y).abs();
                        if dist < min_dist {
                            min_dist = dist;
                            nearest_point = Some((x2 as i32, y2 as i32));
                        }
                    }
                }
            }
        }
        nearest_point
    }

    // Get optimal move direction to reach a target position
    fn get_move_to_position(&self, target: (i32, i32)) -> (i32, i32) {
        let (x, y) = self.position;
        let (target_x, target_y) = target;

        let dx = (target_x - x).signum();
        let dy = (target_y - y).signum();

        // If we're already at the target, return no movement
        if dx == 0 && dy == 0 {
            return (0, 0);
        }

        // Move both horizontally and vertically if possible
        if dx != 0 && dy != 0 {
            return (dx, dy);
        }

        // Otherwise move in the available direction
        (dx, dy)
    }

    // Get locations of enemy tails adjacent to our territory
    fn get_adjacent_enemy_tails(&self, board: &[[Cell; 80]; 25]) -> Vec<(i32, i32)> {
        let mut tails = Vec::new();

        // First find all our territory cells
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Solid(id, _) = cell {
                    if *id == self.id {
                        // Check adjacent cells for enemy tails
                        for &(dx, dy) in Self::DIRS.iter() {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0 && nx < 80 && ny >= 0 && ny < 25 {
                                if let Cell::Tail(id, _) = board[ny as usize][nx as usize] {
                                    if id != self.id {
                                        tails.push((nx, ny));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        tails
    }

    // Find nearest enemy tail within specified range
    fn find_nearest_enemy_tail(&self, board: &[[Cell; 80]; 25], range: i32) -> Option<(i32, i32)> {
        let (x, y) = self.position;
        let mut nearest_tail = None;
        let mut min_dist = range + 1; // Initialize to just over range to find only tails within range

        // Search in a square area around the position
        for dy in -range..=range {
            for dx in -range..=range {
                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < 80 && ny >= 0 && ny < 25 {
                    if let Cell::Tail(id, _) = board[ny as usize][nx as usize] {
                        if id != self.id {
                            let dist = dx.abs() + dy.abs(); // Manhattan distance
                            if dist < min_dist {
                                min_dist = dist;
                                nearest_tail = Some((nx, ny));
                            }
                        }
                    }
                }
            }
        }

        nearest_tail
    }

    fn find_territory_edge(&self, board: &[[Cell; 80]; 25], dir_x: i32, dir_y: i32) -> (i32, i32) {
        let (x, y) = self.position;
        let mut edge_x = x;
        let mut edge_y = y;
        let mut found_x = false;
        let mut found_y = false;

        // Search horizontally
        let mut search_x = x;
        while search_x >= 0 && search_x < 80 && !found_x {
            if let Cell::Solid(id, _) = board[y as usize][search_x as usize] {
                if id == self.id {
                    edge_x = search_x + dir_x;
                    found_x = true;
                }
            }
            search_x -= dir_x;
        }

        // Search vertically
        let mut search_y = y;
        while search_y >= 0 && search_y < 25 && !found_y {
            if let Cell::Solid(id, _) = board[search_y as usize][x as usize] {
                if id == self.id {
                    edge_y = search_y + dir_y;
                    found_y = true;
                }
            }
            search_y -= dir_y;
        }

        // If no territory found, use map boundaries
        if !found_x {
            edge_x = if dir_x > 0 { 0 } else { 79 };
        }
        if !found_y {
            edge_y = if dir_y > 0 { 0 } else { 24 };
        }

        (edge_x, edge_y)
    }

    fn generate_expand_points(&self, board: &[[Cell; 80]; 25]) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        let (x, y) = self.position;

        // Distance to expand beyond territory
        let x_distance = 8;
        let y_distance = 8;

        // Determine horizontal direction based on position
        let dir_x = if x < 40 { 1 } else { -1 };
        // Random vertical direction
        let dir_y = if Random::int(0, 1) == 0 { 1 } else { -1 };

        // Find territory edge in both directions
        let (edge_x, edge_y) = self.find_territory_edge(board, dir_x, dir_y);

        // Move beyond territory edge
        let p1 = (edge_x + (dir_x * x_distance), y);
        points.push(p1);

        // Move perpendicular
        let p2 = (p1.0, p1.1 + (dir_y * y_distance));
        points.push(p2);

        // Move back parallel to territory
        let p3 = (p2.1, y);
        points.push(p3);

        // Complete rectangle
        if let Some(p4) = self.find_nearest_territory_point(board, p3) {
            serial_println!("START: [{}, {}][{:?} {:?} {:?} {:?}]", x, y, p1, p2, p3, p4);
            points.push(p4);
        }

        points
    }

    // MOVEMENT AND COLLISION

    pub fn move_player(&mut self, dx: i32, dy: i32, board: &mut [[Cell; 80]; 25]) -> Option<u8> {
        if !self.alive {
            return None;
        }

        let (ox, oy) = self.position;
        let nx = ox + dx;
        let ny = oy + dy;

        // Bounds checking
        if nx < 0 || nx >= 80 || ny < 0 || ny >= 25 {
            if !self.ai_controlled {
                self.alive = false;
            }
            return None;
        }

        // Check for collisions with tails
        let mut player_to_eliminate = None;

        // Check if player hit an enemy's tail
        if let Cell::Tail(id, _) = board[ny as usize][nx as usize] {
            if id != self.id {
                // Hit other player's tail
                player_to_eliminate = Some(id);
            }
        }

        // Convert old head to tail
        if let Cell::Head(id, owned) = board[oy as usize][ox as usize] {
            if owned {
                board[oy as usize][ox as usize] = Cell::Solid(id, true);
            } else {
                board[oy as usize][ox as usize] = Cell::Tail(id, false);
            }
        }

        // Place new head
        if let Cell::Solid(p, _) = board[ny as usize][nx as usize] {
            board[ny as usize][nx as usize] = Cell::Head(self.id, p == self.id);
        } else {
            board[ny as usize][nx as usize] = Cell::Head(self.id, false);
        }

        // Update the player's position
        self.position = (nx, ny);

        player_to_eliminate
    }
}
