use crate::error::Error;
use crate::map::Map;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    Free,  
    Trash,
    Buoy,
}


#[derive(Clone, Debug)]
pub struct State {
    xpos: usize,
    ypos: usize,
    steps: usize,
    found_symbols: HashSet<Cell>,
    symbol_counts: HashMap<Cell, usize>,
    last_visited: Option<Cell>,
}

impl State {
    pub fn new() -> Self {
        Self {
            xpos: 0,
            ypos: 0,
            steps: 0,
            found_symbols: HashSet::new(),
            symbol_counts: HashMap::new(),
            last_visited: None,
        }
    }


    pub fn step(&mut self, map: &mut Map, slope: (usize, usize)) {
        let map_width: usize = map.width();
        let map_height: usize = map.height();
    
        if self.get_steps() == 0 {
            let cell = map.get((self.xpos, self.ypos));
        self.last_visited = Some(cell);
        self.found_symbols.insert(cell.clone());
        self.steps += 1;
        let count = self.symbol_counts.entry(cell.clone()).or_insert(0);
        *count += 1;
        if cell == Cell::Trash {
            Map::clean(map, (self.xpos, self.ypos));
        }
        }

        //println!("PreX {}  AND PreY {} !!!!!!!!!!!!!!!!!", self.xpos,self.ypos);
        // Calculate the new position based on go_right and go_down
        let new_ypos = (self.ypos + slope.1) % map_height; // swapped ypos with xpos !!!!!!!!
        let new_xpos = (self.xpos + slope.0) % map_width;  // swapped xpos with ypos !!!!!!
        //println!("PostX {}  AND PostY {} !!!!!!!!!!!!!!!!!", new_xpos,new_ypos);
        // Increment the number of visited cells
        self.steps += 1;

        // Get the cell from the map at the new position
        let cell = map.get((new_xpos, new_ypos));
        //println!("SYMBOL {}", <Cell as Into<char>>::into(cell));
        // Add the cell to the set of found cells
        self.found_symbols.insert(cell.clone());
        //println!("FOUND SYMBOLS length {}",self.get_found_symbols().len() );
        // Update the count of the cell in the HashMap
        let count = self.symbol_counts.entry(cell.clone()).or_insert(0); //added clone
        *count += 1;
        self.last_visited = Some(cell.clone());
        if cell == Cell::Trash {
            Map::clean(map, (new_xpos, new_ypos)); // fixed xpos to new_xpos ..
        }

        // Update the current position
        self.xpos = new_xpos;
        self.ypos = new_ypos;
    }

    pub fn get_xpos(&self) -> usize {
        self.xpos
    }

    pub fn get_ypos(&self) -> usize {
        self.ypos
    }

    pub fn get_steps(&self) -> usize {
        self.steps
    }

    pub fn get_found_symbols(&self) -> &HashSet<Cell> {
        &self.found_symbols
    }

    pub fn get_symbol_counts(&self) -> &HashMap<Cell, usize> {
        &self.symbol_counts
    }

    pub fn get_last_visited(&self) -> &Option<Cell> {
        &self.last_visited
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let cc = c.to_uppercase().next().unwrap();
        Ok(if cc == '.' {
            Cell::Free
        } else if cc == 'X' {
            Cell::Trash
        } else if cc == 'O' {
            Cell::Buoy 
        } else {
            panic!("Invalid input format") // here also add the 'O' case 
        })
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Free => '.',
            Cell::Trash => 'X', 
            Cell::Buoy => 'O',
        }
    }
}

#[derive(Clone, Debug)]
pub enum Termination {  // other cases
    Steps(usize),
    Symbol(Cell),         // reaching terminal symbol
    Position((usize, usize)),   // reaching a position
}

impl From<String> for Termination {
    fn from(text: String) -> Self {
        // Termination::Steps(text.parse().unwrap())
        let parts: Vec<&str> = text.split(',').collect();

        match parts.len() {
            // Case: <N> (e.g., "3")
            1 => Termination::Steps(parts[0].parse().unwrap()),

            // Case: "S,<X>" (e.g., "S,X")
            2 => {
                let symbol = Cell::try_from(parts[1].chars().next().unwrap()).unwrap();
                Termination::Symbol(symbol)
            }

            // Case: "P,<X>,<Y>" (e.g., "P,2,5")
            3 => {
                let x = parts[1].trim().parse().unwrap();
                let y = parts[2].trim().parse().unwrap();
                Termination::Position((x, y))
            }

            _ => panic!("Invalid input format"),
        }
    }
}

pub(crate) fn terminal(termination: &Termination, _map: &crate::map::Map, state: &State) -> bool {
    
    match termination {
        Termination::Steps(steps) => {
            *steps == State::get_steps(state)
        }
        Termination::Symbol(symb) => {
            State::get_found_symbols(state).contains(symb)
        }
        Termination::Position((xpos, ypos)) => {
            (*xpos == State::get_xpos(state)) && (*ypos == State::get_ypos(state))
        }
    }

}

pub enum Output {  
    Steps, 
    SymbolCount(Cell), 
    TerminalSymbol, 
    Position, 
    Distinct, 
}

impl FromStr for Output {
    type Err = std::str::Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Output::from(s))
    }
}

impl<'a, S> From<S> for Output
where
    S: Into<&'a str>,
{
    fn from(text: S) -> Self {
        let text: &str = text.into();
        let lowercase_text = text.to_lowercase();
        if lowercase_text.starts_with("c,") {
            // Extract the symbol after "C,"
            let symbol = lowercase_text.chars().nth(2).unwrap();
            let cell = Cell::try_from(symbol).unwrap();
            Output::SymbolCount(cell)
        } else {
            match text.to_lowercase().as_str() {
                "p" => Output::Position,
                "n" => Output::Steps,
                "s" => Output::TerminalSymbol,
                "d" => Output::Distinct,
                _ => panic!("{} is not supported", text),
            }
        }
        
    }
}

pub(crate) fn output(output: &Output, end: (crate::map::Map, State)) -> String {
    // todo!();
    let map_end = end.0;
    let state_end: State = end.1;
    match output {
        Output::Steps => {
            //print!("{}",State::get_steps(&state_end));
            State::get_steps(&state_end).to_string()
        }
        Output::SymbolCount(cell) => {
            let hmap = State::get_symbol_counts(&state_end);

            match hmap.get(cell) {
                Some(&value) => {
                //println!("{}", value.to_string());
                    value.to_string()
                },
                None => "0".to_string(),
            }
        }
        Output::TerminalSymbol => {
            let current = Map::get(&map_end, (State::get_xpos(&state_end), State::get_ypos(&state_end)));
            let current_char: char = current.into();
            current_char.to_string()
        }
        Output::Position => {
            format!("({},{})", State::get_xpos(&state_end), State::get_ypos(&state_end))
        }
        Output::Distinct => {
            State::get_found_symbols(&state_end).len().to_string()
        }
    }

}

pub(crate) fn step(map: &mut crate::map::Map, state: &mut State, slope: (usize, usize)) -> () {
    // todo!()
    State::step(state, map, slope);
}
