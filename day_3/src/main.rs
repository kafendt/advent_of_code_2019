use serde::Deserialize;
use serde_yaml;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone)]
struct ParseError;

#[derive(Debug)]
struct WireSection {
    direction: Direction,
    amount: i32,
}

impl WireSection {
    pub fn new(raw_section: &String) -> Result<WireSection, ParseError> {
        let (raw_direction, amount) = raw_section.split_at(1);
        Ok(WireSection {
            direction: match raw_direction {
                "U" => Direction::UP,
                "D" => Direction::DOWN,
                "L" => Direction::LEFT,
                "R" => Direction::RIGHT,
                _ => return Err(ParseError),
            },
            amount: amount.parse().expect("Amount cannot be converted to int."),
        })
    }
}

#[derive(Debug)]
struct Wire {
    id: u8,
    sections: Vec<WireSection>,
}

#[derive(Debug)]
struct Input {
    wire1: Wire,
    wire2: Wire,
}

#[derive(Debug, Deserialize)]
struct RawInput {
    wire1: Vec<String>,
    wire2: Vec<String>,
}

#[derive(Clone)]
struct GridSpec {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl GridSpec {
    pub fn new(wire: &Wire) -> GridSpec {
        let mut grid_spec = GridSpec {
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        };

        let (mut current_x, mut current_y) = (0, 0);
        wire.sections
            .iter()
            .fold(&mut grid_spec, |spec, section| match section.direction {
                Direction::UP => {
                    current_y += section.amount;
                    if current_y > spec.max_y {
                        spec.max_y = current_y;
                    }
                    spec
                }
                Direction::DOWN => {
                    current_y -= section.amount;
                    if current_y < spec.min_y {
                        spec.min_y = current_y;
                    }
                    spec
                }
                Direction::RIGHT => {
                    current_x += section.amount;
                    if current_x > spec.max_x {
                        spec.max_x = current_x;
                    }
                    spec
                }
                Direction::LEFT => {
                    current_x -= section.amount;
                    if current_x < spec.min_x {
                        spec.min_x = current_x;
                    }
                    spec
                }
            });

        grid_spec
    }

    pub fn merge(&self, grid_spec: &GridSpec) -> GridSpec {
        GridSpec {
            min_x: self.min_x.min(grid_spec.min_x),
            min_y: self.min_y.min(grid_spec.min_y),
            max_x: self.max_x.max(grid_spec.max_x),
            max_y: self.max_y.max(grid_spec.max_y),
        }
    }
}

#[derive(Clone)]
struct Grid {
    #[allow(dead_code)]
    spec: GridSpec,
    values: Vec<Vec<u8>>,
    central_port: (i32, i32),
    intersections: Vec<(i32, i32)>,
}

impl Grid {
    pub fn new(spec: &GridSpec) -> Self {
        let size_x = (spec.max_x - spec.min_x + 1) as usize;
        let size_y = (spec.max_y - spec.min_y + 1) as usize;
        Self {
            spec: spec.clone(),
            values: vec![vec![0; size_x]; size_y],
            central_port: (spec.min_y.abs(), spec.min_x.abs()),
            intersections: Vec::new(),
        }
    }

    pub fn add_wire(&mut self, wire: &Wire) -> &mut Self {
        let (mut current_y, mut current_x) = self.central_port;
        wire.sections.iter().for_each(|section| {
            match section.direction {
                Direction::UP => {
                    for i in current_y + 1..current_y + section.amount + 1 {
                        let i = i as usize;
                        let field = self.values[i][current_x as usize];
                        if field != wire.id && field != 3 && field != 0 {
                            self.intersections.push((i as i32, current_x));
                            self.values[i][current_x as usize] = 3;
                        } else {
                            self.values[i][current_x as usize] = wire.id;
                        }
                    }
                    current_y += section.amount;
                }
                Direction::DOWN => {
                    for i in current_y - section.amount..current_y {
                        let i = i as usize;
                        let field = self.values[i][current_x as usize];
                        if field != wire.id && field != 3 && field != 0 {
                            self.intersections.push((i as i32, current_x));
                            self.values[i][current_x as usize] = 3;
                        } else {
                            self.values[i][current_x as usize] = wire.id;
                        }
                    }
                    current_y -= section.amount;
                }
                Direction::RIGHT => {
                    for i in current_x + 1..current_x + section.amount + 1 {
                        let i = i as usize;
                        let field = self.values[current_y as usize][i];
                        if field != wire.id && field != 3 && field != 0 {
                            self.intersections.push((current_y, i as i32));
                            self.values[current_y as usize][i] = 3;
                        } else {
                            self.values[current_y as usize][i] = wire.id;
                        }
                    }
                    current_x += section.amount;
                }
                Direction::LEFT => {
                    for i in current_x - section.amount..current_x {
                        let i = i as usize;
                        let field = self.values[current_y as usize][i];
                        if field != wire.id && field != 3 && field != 0 {
                            self.intersections.push((current_y, i as i32));
                            self.values[current_y as usize][i] = 3;
                        } else {
                            self.values[current_y as usize][i] = wire.id;
                        }
                    }
                    current_x -= section.amount;
                }
            };
        });
        self
    }
}

fn read_input(yaml_path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(yaml_path)?;
    let raw: RawInput = serde_yaml::from_reader(f)?;
    let wire1 = raw
        .wire1
        .into_iter()
        .map(|string| WireSection::new(&string).expect("Couldn't parse section."))
        .collect();
    let wire2 = raw
        .wire2
        .into_iter()
        .map(|string| WireSection::new(&string).expect("Couldn't parse section."))
        .collect();

    Ok(Input {
        wire1: Wire {
            id: 1,
            sections: wire1,
        },
        wire2: Wire {
            id: 2,
            sections: wire2,
        },
    })
}

fn main() {
    // Read the input
    let yaml_path = "./src/data.yaml";
    let input = read_input(yaml_path).unwrap();

    // Create the grid specifications
    let grid_spec1 = GridSpec::new(&input.wire1);
    let grid_spec2 = GridSpec::new(&input.wire2);
    let grid_spec = grid_spec1.merge(&grid_spec2);

    // Create the Grid
    let mut grid = Grid::new(&grid_spec);

    // Place wires in grid and calculate distances
    grid.add_wire(&input.wire1).add_wire(&input.wire2);

    let mut distances: Vec<i32> = grid
        .intersections
        .iter()
        .map(|i| (i.0 - grid.central_port.0).abs() + (i.1 - grid.central_port.1).abs())
        .collect();

    // println!("Port: {:?}", grid.central_port);
    // println!("Intersections: {:?}", grid.intersections);
    //println!("Values: {:?}", grid.values);
    println!("Distances: {:?}", {
        distances.sort();
        distances
    });
}
