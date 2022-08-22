#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Alive = 1,
    Dead = 0,
}

pub struct Space {
    pub field: Vec<Vec<State>>,
}

impl Space {
    pub fn init(field: Vec<Vec<State>>) -> Self {
        return Self { field: field };
    }

    pub fn get_neighbor_at(&self, row: usize, column: usize) -> State {
        if *self
            .field
            .get(row)
            .unwrap_or(&Vec::new())
            .get(column)
            .unwrap_or(&State::Dead)
            == State::Alive
        {
            State::Alive
        } else {
            State::Dead
        }
    }

    pub fn get_next_state(&self, row: usize, column: usize) -> State {
        let current = *self.field.get(row).unwrap().get(column).unwrap();

        let right = self.get_neighbor_at(row, column + 1) as usize;
        let down_right = self.get_neighbor_at(row + 1, column + 1) as usize;
        let down = self.get_neighbor_at(row + 1, column) as usize;

        let mut down_left: usize = 0;
        let mut left: usize = 0;
        let mut up_left: usize = 0;
        let mut up: usize = 0;
        let mut up_right: usize = 0;

        if column > 0 {
            down_left = self.get_neighbor_at(row + 1, column - 1) as usize;
            left = self.get_neighbor_at(row, column - 1) as usize;
        }

        if row > 0 {
            up = self.get_neighbor_at(row - 1, column) as usize;
            up_right = self.get_neighbor_at(row - 1, column + 1) as usize
        }

        if row > 0 && column > 0 {
            up_left = self.get_neighbor_at(row - 1, column - 1) as usize;
        }

        let neighbors = up_left + up + up_right + right + down_right + down + down_left + left;
        if current == State::Alive {
            if neighbors <= 1 {
                return State::Dead;
            } else if neighbors <= 3 {
                return State::Alive;
            } else {
                return State::Dead;
            }
        } else {
            if neighbors == 3 {
                return State::Alive;
            }
            return State::Dead;
        }
    }

    pub fn size(&self) -> usize {
        self.field.len()
    }
}

pub fn evaluate(space: &Space) -> Space {
    let height = space.field.len();
    let width = space.field.first().unwrap().len();
    let mut next_space: Space = Space::init(vec![vec![State::Dead; width]; height]);

    for r in 0..height {
        for c in 0..width {
            next_space.field[r][c] = space.get_next_state(r, c);
        }
    }

    return next_space;
}
