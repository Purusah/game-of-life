use rand::random;

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Alive = 1,
    Dead = 0,
}

pub type Space = Vec<Vec<State>>;

pub fn gen_space(is_random: bool) -> Space {
    if is_random {
        // let mut next_space: Space = ;
        let height = 12;
        let width = 12;
        let mut space: Space = vec![vec![State::Dead; width]; height];
        for r in 0..height {
            for c in 0..width {
                space[r][c] = if random() { State::Alive } else { State::Dead };
            }
        }
        return space;
    }

    return vec![
        vec![State::Dead, State::Alive, State::Dead, State::Dead],
        vec![State::Dead, State::Dead, State::Alive, State::Dead],
        vec![State::Alive, State::Alive, State::Alive, State::Dead],
        vec![State::Dead, State::Dead, State::Dead, State::Dead],
        vec![State::Dead, State::Dead, State::Dead, State::Dead],
        vec![State::Dead, State::Dead, State::Dead, State::Dead],
    ];
}

fn get_neighbor_at(space: &Space, row: usize, column: usize) -> State {
    if *space
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

fn get_next_state(space: &Space, row: usize, column: usize) -> State {
    let current = *space.get(row).unwrap().get(column).unwrap();

    let right = get_neighbor_at(space, row, column + 1) as usize;
    let down_right = get_neighbor_at(space, row + 1, column + 1) as usize;
    let down = get_neighbor_at(space, row + 1, column) as usize;

    let mut down_left: usize = 0;
    let mut left: usize = 0;
    let mut up_left: usize = 0;
    let mut up: usize = 0;
    let mut up_right: usize = 0;

    if column > 0 {
        down_left = get_neighbor_at(space, row + 1, column - 1) as usize;
        left = get_neighbor_at(space, row, column - 1) as usize;
    }

    if row > 0 {
        up = get_neighbor_at(space, row - 1, column) as usize;
        up_right = get_neighbor_at(space, row - 1, column + 1) as usize
    }

    if row > 0 && column > 0 {
        up_left = get_neighbor_at(space, row - 1, column - 1) as usize;
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

pub fn evaluate(space: &Space) -> Space {
    let height = space.len();
    let width = space.first().unwrap().len();
    let mut next_space: Space = vec![vec![State::Dead; width]; height];

    for r in 0..height {
        for c in 0..width {
            next_space[r][c] = get_next_state(space, r, c);
        }
    }

    return next_space;
}
