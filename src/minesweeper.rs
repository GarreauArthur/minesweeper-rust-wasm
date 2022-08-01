use std::{collections::HashSet, fmt::{Display, Write}};

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8), // number of mines around
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if self.flagged_fields.contains(&pos) {
                    f.write_str("🚩 ")?;
                } else if !self.open_fields.contains(&pos) {
                    f.write_str("🟪 ")?;
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let nb_neighbor_mines = self.neighboring_mines((x, y));
                    if nb_neighbor_mines == 0 {
                        f.write_str("⬛ ")?;
                    } else {
                        write!(f, " {} ", nb_neighbor_mines)?;
                    }
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Minesweeper {

    fn create_mines(mine_count: &usize, width: &usize, height: &usize) -> HashSet<Position> {

        use rand::{thread_rng, Rng};
        let mut mines = HashSet::new();
        let mut rng = thread_rng();
        while mines.len() < *mine_count {
            let x: usize = rng.gen_range(0..*height); 
            let y: usize = rng.gen_range(0..*width); 
            mines.insert((x, y));
        }

        mines
    }

    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {

        if (mine_count >= (width*height)) || mine_count <= 0 {
            panic!("Invalid number of mines: {}, should be in [1;{}]", mine_count, width*height-1)
        }

        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: Minesweeper::create_mines(&mine_count, &width, &height),
            flagged_fields: HashSet::new(),
        }
    }

    /// Create an iterator over the neighbors of a given position.
    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;
        (x.max(1)-1..=(x+1).min(width-1)).flat_map(move |i| {
            (y.max(1)-1..=(y+1).min(height-1)).map(move |j| (i, j))
        }).filter(move |&pos| pos != (x, y))
    }

    /// Returns the number of mines around a given position.
    pub fn neighboring_mines(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(|position| self.mines.contains(position))
            .count() as u8
    }

    /// When the user click a field, we check if it's a mine or an opened field
    pub fn open(&mut self, clicked_position: Position) -> Option<OpenResult> {
        if self.flagged_fields.contains(&clicked_position) {
            return None; 
        }
        if self.open_fields.contains(&clicked_position) {
            return Some(OpenResult::NoMine(self.neighboring_mines(clicked_position)));
        }
        self.open_fields.insert(clicked_position);
        if self.mines.contains(&clicked_position) {
            return Some(OpenResult::Mine);
        }
        let nb_mines = self.neighboring_mines(clicked_position);
        if nb_mines == 0 {
            for neighbor in self.iter_neighbors(clicked_position) {
                self.open(neighbor);
            }
        }
        Some(OpenResult::NoMine(nb_mines))
    }


    pub fn toggle_flag(&mut self, clicked_position: Position) {

        if self.open_fields.contains(&clicked_position) {
            return;
        }

        if self.flagged_fields.contains(&clicked_position) {
            self.flagged_fields.remove(&clicked_position);
        } else {
            self.flagged_fields.insert(clicked_position);
        }
    }


}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
       let mut ms = Minesweeper::new(10, 10, 1); 
       println!("{:?}", ms);
       ms.open((3,4));
       ms.toggle_flag((4,5));
       println!("{}", ms);
    }
}