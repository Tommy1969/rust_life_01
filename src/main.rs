#[macro_use] extern crate itertools;

struct LifeBoard {
    size:       u32,
    cells:      Vec<bool>
}

impl LifeBoard {
    fn offset(&self, x: u32, y: u32) -> usize {
        (y * self.size + x) as usize
    }

    fn is_alive_nw(&self, x: u32, y: u32) -> bool {
        match x > 0 && y > 0 {
            true => self.cells[(self.offset(x-1, y-1))],
            _ => false
        }
    }
    fn is_alive_nn(&self, x: u32, y: u32) -> bool {
        match y > 0 {
            true => self.cells[self.offset(x, y-1)],
            _ => false
        }
    }
    fn is_alive_ne(&self, x: u32, y: u32) -> bool {
        match x+1 < self.size && y > 0 {
            true => self.cells[self.offset(x+1, y-1)],
            _ => false
        }
    }
    fn is_alive_ww(&self, x: u32, y: u32) -> bool {
        match x > 0 {
            true => self.cells[self.offset(x-1, y)],
            _ => false
        }
    }
    fn is_alive_ee(&self, x: u32, y: u32) -> bool {
        match x+1 < self.size {
            true => self.cells[self.offset(x+1, y)],
            _ => false
        }
    }
    fn is_alive_sw(&self, x: u32, y: u32) -> bool {
        match x > 0 && y+1 < self.size {
            true => self.cells[self.offset(x-1, y+1)],
            _ => false
        }
    }
    fn is_alive_ss(&self, x: u32, y: u32) -> bool {
        match y+1 < self.size {
            true => self.cells[self.offset(x, y+1)],
            _ => false
        }
    }
    fn is_alive_se(&self, x: u32, y: u32) -> bool {
        match x+1 < self.size && y+1 < self.size {
            true => self.cells[self.offset(x+1, y+1)],
            _ => false
        }
    }

    fn around(&self, x: u32, y: u32) -> u32 {
        let survivors: Vec<bool> = vec![
            self.is_alive_nw(x, y),
            self.is_alive_nn(x, y),
            self.is_alive_ne(x, y),
            self.is_alive_ww(x, y),
            self.is_alive_ee(x, y),
            self.is_alive_sw(x, y),
            self.is_alive_ss(x, y),
            self.is_alive_se(x, y)
        ];
        (survivors.iter().filter(|&&i| i).count()) as u32
    }

    fn next_cell(&self, x: u32, y: u32) -> bool {
        match self.around(x, y) {
            2 => self.cells[self.offset(x, y)],
            3 => true,
            _ => false
        }
    }

    fn next_board(&self) -> LifeBoard {
        let next_board = iproduct!(0..self.size, 0..self.size)
            .map(|(y, x)| self.next_cell(x, y))
            .collect::<Vec<bool>>();

        LifeBoard {size: self.size, cells: next_board}
    }
}

#[cfg(test)]
mod test_life_board {
    use super::*;

    #[test]
    fn test_offset() {
        let target = LifeBoard {size: 3, cells: vec![]};
        assert_eq!(0, target.offset(0, 0));
        assert_eq!(1, target.offset(1, 0));
        assert_eq!(2, target.offset(2, 0));
        assert_eq!(3, target.offset(0, 1));
        assert_eq!(4, target.offset(1, 1));
        assert_eq!(5, target.offset(2, 1));
        assert_eq!(6, target.offset(0, 2));
        assert_eq!(7, target.offset(1, 2));
        assert_eq!(8, target.offset(2, 2));
    }

    #[test]
    fn test_around() {
        let cells = vec![true, true, true, true, true, true, true, true, true];
        let target = LifeBoard {size: 3, cells: cells};
        assert_eq!(3, target.around(0, 0));
        assert_eq!(5, target.around(0, 1));
        assert_eq!(8, target.around(1, 1));
        assert_eq!(3, target.around(2, 2));
        assert_eq!(3, target.around(2, 2));
    }
}

fn main() {
    let cells = vec![
        false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false,
        false, false, true, false, false, false, true, false, false, false, false,
        false, true, false, false, false, false, false, true, false, false, false,
        false, true, false, false, false, true, false, true, true, false, false,
        false, true, false, false, false, false, false, true, false, false, false,
        false, false, true, false, false, false, true, false, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false
        ];
    let mut target = LifeBoard {size: 11, cells: cells};
    for i in 1..=100 {
        target = target.next_board();
    }
    println!("{:?}", target.cells);
}
