use std::fmt;
use rand::{Rand, Rng, SeedableRng, XorShiftRng};

pub enum Terrain {
    Field,
    Woods,
    Mountain,
    Building,
}

pub enum ResourceType {
    Wood,
    Minerals,
    Food
}

// TODO(teider): Move this to another module later on
pub struct Resource {
    resource_type: ResourceType,
    quantity: usize,
}

impl Rand for Resource {
    fn rand<R:Rng>(rng: &mut R) -> Resource {
        Resource {
            resource_type: ResourceType::Minerals,
            quantity: 200,
        }
    }
}

pub struct Cell {
    terrain: Terrain,
    resources: Vec<Resource>,
}

impl Cell {
    pub fn populate_resources(&mut self, rng: &mut XorShiftRng) {
        if rng.gen_weighted_bool(2) {
            self.resources.push(rng.gen());
        }
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    cell: Vec<Cell>,
}

pub struct GridBuilder {
    width: usize,
    height: usize,
    seed: [u32; 4],
    origin: (usize, usize),
}

impl GridBuilder {
    pub fn new(width: usize, height: usize) -> GridBuilder {
        GridBuilder {
            width: width,
            height: height,
            seed: [1, 2, 3, 4],
            origin: (0, 0),
        }
    }

    pub fn seed(&mut self, seed: [u32; 4]) -> &mut GridBuilder {
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
        let mut rng: XorShiftRng = SeedableRng::from_seed(self.seed);
        for i in 0..grid.height {
            for j in 0..grid.width {
                let terrain: Terrain = match rng.next_u32() % 3 {
                    0 => Terrain::Woods,
                    1 => Terrain::Mountain,
                    _ => Terrain::Field,
                };
                let mut cell: Cell = Cell {
                    terrain: if (i, j) == self.origin {
                        Terrain::Building
                    } else {
                        terrain
                    },
                    resources: Vec::new(),
                };
                cell.populate_resources(&mut rng);
                grid.cell.push(cell);
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
