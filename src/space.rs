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

    pub fn get_neighbor_at(&self, row: i64, column: i64) -> State {
        let size = self.field.len() as i64;

        let row_wrapped = if row < 0 || row > size {
            size % row
        } else {
            row
        } as usize;
        let column_wrapped = if column < 0 || column > size {
            size % column
        } else {
            column
        } as usize;

        if *self
            .field
            .get(row_wrapped)
            .unwrap_or(&Vec::new())
            .get(column_wrapped)
            .unwrap_or(&State::Dead)
            == State::Alive
        {
            State::Alive
        } else {
            State::Dead
        }
    }

    fn get_next_state(&self, row_current: usize, column_current: usize) -> State {
        let row = row_current as i64;
        let column = column_current as i64;
        let current = *self
            .field
            .get(row_current)
            .unwrap()
            .get(column_current)
            .unwrap();

        let right = self.get_neighbor_at(row, column + 1) as usize;
        let down_right = self.get_neighbor_at(row + 1, column + 1) as usize;
        let down = self.get_neighbor_at(row + 1, column) as usize;
        let down_left = self.get_neighbor_at(row + 1, column - 1) as usize;
        let left = self.get_neighbor_at(row, column - 1) as usize;
        let up_left = self.get_neighbor_at(row - 1, column - 1) as usize;
        let up = self.get_neighbor_at(row - 1, column) as usize;
        let up_right = self.get_neighbor_at(row - 1, column + 1) as usize;

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

    pub fn dim(&self) -> usize {
        self.field.len()
    }
}

pub fn evaluate(space: &Space) -> Space {
    let dim = space.dim();
    let mut next_space: Space = Space::init(vec![vec![State::Dead; dim]; dim]);

    for r in 0..dim {
        for c in 0..dim {
            next_space.field[r][c] = space.get_next_state(r, c);
        }
    }

    return next_space;
}
