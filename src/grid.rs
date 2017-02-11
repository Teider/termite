use std::fmt;

pub enum Terrain {
    Field,
    Woods,
    Mountain,
    Building,
}

// TODO(teider): Move this to another module later on
pub struct Resource {
    quantity: usize,
}

pub struct Cell {
    terrain: Terrain,
    resources: Vec<Resource>,
}

pub struct Grid {
    width: usize,
    height: usize,
    cell: Vec<Cell>,
}

pub struct GridBuilder {
    width: usize,
    height: usize,
    seed: f64,
    origin: (usize, usize),
}

impl GridBuilder {
    pub fn new(width: usize, height: usize) -> GridBuilder {
        GridBuilder {
            width: width,
            height: height,
            seed: 0.0,
            origin: (0, 0),
        }
    }

    pub fn seed(&mut self, seed: f64) -> &mut GridBuilder {
        self.seed = seed;
        self
    }

    pub fn origin(&mut self, origin: (usize, usize)) -> &mut GridBuilder {
        self.origin = origin;
        self
    }

    pub fn build(&self) -> Grid {
        let mut grid = Grid {
            width: self.width,
            height: self.height,
            cell: Vec::with_capacity(self.width * self.height),
        };
        let mut count = 0;
        for i in 0..grid.height {
            for j in 0..grid.width {
                // TODO (teider): Generate random cells
                let terrain: Terrain = match count % 3 {
                    0 => Terrain::Woods,
                    _ => Terrain::Field,
                };
                grid.cell.push(Cell {
                    terrain: if (i, j) == self.origin {
                        Terrain::Building
                    } else {
                        terrain
                    },
                    resources: Vec::new(),
                });
                count = count + 1;
            }
        }
        grid
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        let mut pos = 0;
        for _ in 0..self.height {
            for _ in 0..self.width {
                str = str +
                      match self.cell[pos].terrain {
                    Terrain::Field => "~",
                    Terrain::Woods => "#",
                    Terrain::Mountain => "*",
                    Terrain::Building => "@",
                };
                pos = pos + 1;
            }
            str = str + "\n";
        }
        str = str + "\n~ = field; # = woods; * = mountain, @ = building (main base)\n";
        write!(f, "{}", &str)
    }
}
