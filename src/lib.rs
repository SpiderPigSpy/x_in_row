#[cfg(not(test))] pub const WIDTH: usize = 5;
#[cfg(not(test))] pub const HEIGHT: usize = 5;
#[cfg(not(test))] pub const X_ROW: usize = 4;

#[cfg(test)] pub const WIDTH: usize = 4;
#[cfg(test)] pub const HEIGHT: usize = 3;
#[cfg(test)] pub const X_ROW: usize = 2;

pub struct Game {
    turns: u32,
    current_turn: Player,
    winner: Option<Player>,
    field: Field,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turns: 0,
            current_turn: Player::Red,
            winner : None,
		    field: Field::new(),
		}
    }
    
    pub fn current_turn(&self) -> Player {
        self.current_turn
    }
    
    pub fn winner(&self) -> Option<Player> {
        self.winner
    }
    
    pub fn make_turn(&mut self, column: usize) -> Result<(), GameError> {
        if self.winner.is_some() { return Err(GameError::AlreadyEnded); }
        try!(self.field.throw_token(column, self.current_turn));
        self.winner = self.field.win_check();
        self.turns += 1;
        self.current_turn = self.current_turn.next();
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameError {
    NoSuchColumn,
    ColumnIsFull,
    AlreadyEnded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Red,
    Blue,
}

impl Player {
    fn next(self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}

struct Field {
    cells: [Option<Player>; WIDTH * HEIGHT],
}

impl Field {
    fn new() -> Field {
        Field {
		    cells: [None; (WIDTH * HEIGHT) as usize],
		}
    }
    
    fn throw_token(&mut self, column_index: usize, player: Player) -> Result<(), GameError> {
        if column_index > WIDTH { return Err(GameError::NoSuchColumn); }
        let Column { cells } = Columns::index(&self.cells, column_index).next().unwrap();
        let row_index = try!( cells.iter().position(|x| x.is_none() ).ok_or(GameError::ColumnIsFull) );
        let position = row_index * WIDTH + column_index;
        self.cells[position] = Some(player);
        Ok(())
    }
    
    fn win_check(&self) -> Option<Player> {
        for Row { cells } in self.rows() {
            match Field::max_series(&cells[..]) {
                (Some(player), length) => {
                	if length >= X_ROW {
                	    return Some(player);
                	}
                },
                _ => {}
            }
        }
        
        for Column { cells } in self.columns() {
            match Field::max_series(&cells[..]) {
                (Some(player), length) => {
                	if length >= X_ROW {
                	    return Some(player);
                	}
                },
                _ => {}
            }
        }
        
        for Diagonal { cells, length } in self.diagonals() {
            match Field::max_series(&cells[0..length]) {
                (Some(player), length) => {
                	if length >= X_ROW {
                	    return Some(player);
                	}
                },
                _ => {}
            }
        }
        
        None
    }
    
    fn rows<'a>(&'a self) -> Rows<'a>{
        Rows::new(&self.cells)
    }
    
    fn columns<'a, 'b>(&'a self) -> Columns<'a> {
        Columns::new(&self.cells)
    }
    
    fn diagonals<'a>(&'a self) -> Diagonals<'a> {
        Diagonals::new(&self.cells)
    }
    
    fn max_series(cells: &[Option<Player>]) -> (Option<Player>, usize) {
        const RED: Option<Player> = Some(Player::Red);
        const BLUE: Option<Player> = Some(Player::Blue);
        let mut red_max = 0;
        let mut blue_max = 0;
        let mut prev_player = None;
        let mut series = 0;
        
        for curr_player in cells.iter() {
            match (*curr_player, prev_player) {
                (Some(_), None) => { series = 1; },
                (RED, RED) | (BLUE, BLUE) => { series += 1; },
                (None, RED) | (BLUE, RED) => { 
					if series > red_max { red_max = series; }
					series = 1;
				},
                (None, BLUE) | (RED, BLUE) => {
					if series > blue_max { blue_max = series; }
					series = 1;
				},
                _ => {}
            }
            prev_player = *curr_player;
        }
        
        match prev_player {
            BLUE => if series > blue_max { blue_max = series; },
            RED  => if series > red_max { red_max = series; },
            _ => {},
        }
        
        if red_max == blue_max { return (None, red_max); }
        if red_max > blue_max { return (RED, red_max); } else { return (BLUE, blue_max); }
    }
}

struct Rows<'a> {
    field: &'a [Option<Player>; WIDTH * HEIGHT],
    current_row: usize,
}

impl<'a> Rows<'a> {
    fn new(field: &'a [Option<Player>; WIDTH * HEIGHT]) -> Rows<'a> {
        Rows {
		    field: field,
		    current_row: 0,
		}
    }
    
    fn index(field: &'a [Option<Player>; WIDTH * HEIGHT], index: usize) -> Rows<'a> {
        Rows {
		    field: field,
		    current_row: index,
		}
    }
}

impl<'a> Iterator for Rows<'a> {
    type Item = Row;
    
    fn next(&mut self) -> Option<Row> {
        if self.current_row < HEIGHT {
            let from = self.current_row * WIDTH;
            let to = from + WIDTH;
            let mut cells = [None; WIDTH];
            for (val, index) in (&self.field[from .. to]).iter().zip(0..WIDTH) {
                cells[index] = *val;
            }
            self.current_row += 1;
            Some( Row { cells: cells} )
        } else {
            None
        }
    }
}

struct Row {
    cells: [Option<Player>; WIDTH],
}

struct Columns<'a> {
    field: &'a [Option<Player>; WIDTH * HEIGHT],
    current_column: usize,
}

impl<'a> Columns<'a> {
    fn new(field: &'a [Option<Player>; WIDTH * HEIGHT]) -> Columns<'a> {
        Columns {
		    field: field,
			current_column: 0,
		}
    }
    
    fn index(field: &'a [Option<Player>; WIDTH * HEIGHT], index: usize) -> Columns<'a> {
        Columns {
		    field: field,
			current_column: index,
		}
    }
}

impl<'a> Iterator for Columns<'a> {
    type Item = Column;
    
    fn next(&mut self) -> Option<Column> {
        if self.current_column < WIDTH {
            let mut cells = [None; HEIGHT];
            for i in 0..HEIGHT {
                let index: usize = (i * WIDTH as usize) + self.current_column as usize;
                cells[i] = self.field[index];
            }
            self.current_column += 1;
            Some( Column { cells: cells} )
        } else {
            None
        }
    }
}

struct Column {
    cells: [Option<Player>; HEIGHT],
}

struct Diagonals<'a> {
    field: &'a [Option<Player>; WIDTH * HEIGHT],
    current_diagonal : usize,
    total_diagonals: usize,
}

impl<'a> Diagonals<'a> {
    fn new(field: &'a [Option<Player>; WIDTH * HEIGHT]) -> Diagonals<'a> {
        Diagonals {
		    field: field,
		    current_diagonal: 0,
		    total_diagonals: WIDTH + HEIGHT - 3,
		}
    }
}

impl<'a> Iterator for Diagonals<'a> {
    type Item = Diagonal;
    
    fn next(&mut self) -> Option<Diagonal> {
        if WIDTH == 1 || HEIGHT == 1 { return None; }
        if self.total_diagonals >= self.current_diagonal { return None; }
        None
    }
}

struct Diagonal {
    cells: [Option<Player>; WIDTH + HEIGHT],
    length: usize,
}

#[test]
fn test_diagonal_win() {
    let mut game = Game::new();
    game.make_turn(0).unwrap();
    println!("");
    println!("{:?}", &game.field.cells);
    game.make_turn(1).unwrap();
    println!("{:?}", &game.field.cells);
    game.make_turn(1).unwrap();
    println!("{:?}", &game.field.cells);
    assert_eq!(game.win_check(), Some(Player::Red) );
    assert_eq!(game.make_turn(0), Err(GameError::AlreadyEnded) );
}


#[test]
fn test_row_win() {
    let mut game = Game::new();
    game.make_turn(0).unwrap();
    println!("");
    println!("{:?}", &game.field.cells);
    game.make_turn(0).unwrap();
    println!("{:?}", &game.field.cells);
    game.make_turn(1).unwrap();
    println!("{:?}", &game.field.cells);
    assert_eq!(game.win_check(), Some(Player::Red) );
    assert_eq!(game.make_turn(0), Err(GameError::AlreadyEnded) );
}

#[test]
fn test_column_win() {
    let mut game = Game::new();
    game.make_turn(0).unwrap();
    println!("");
    println!("{:?}", &game.field.cells);
    game.make_turn(1).unwrap();
    println!("{:?}", &game.field.cells);
    game.make_turn(0).unwrap();
    println!("{:?}", &game.field.cells);
    assert_eq!(game.win_check(), Some(Player::Red) );
    assert_eq!(game.make_turn(0), Err(GameError::AlreadyEnded) );
}

#[test]
fn test_max_series() {
    let red = Some(Player::Red);
    let blue = Some(Player::Blue);
    assert_eq!(Field::max_series(&[]), (None, 0));
    assert_eq!(Field::max_series(&[None, None]), (None, 0));
    assert_eq!(Field::max_series(&[red]), (red, 1));
    assert_eq!(Field::max_series(&[blue]), (blue, 1));
    assert_eq!(Field::max_series(&[red, blue]), (None, 1));
    assert_eq!(Field::max_series(&[red, red, blue]), (red, 2));
    assert_eq!(Field::max_series(&[red, red, None, red, red, red]), (red, 3));
    assert_eq!(Field::max_series(&[blue, blue, None, red, red, red]), (red, 3));
    assert_eq!(Field::max_series(&[blue, blue, None, red, red, red, blue, blue, blue]), (None, 3));
}

#[test]
fn test_rows_iterator() {
    let red = Some(Player::Red);
    let blue = Some(Player::Blue);
    let cells = &[red,  None, blue, None, 
    			  None, red,  blue, red, 
    			  None, None, None, None,];
    let mut rows = Rows::new(cells);
    let row1 = rows.next().unwrap();
    let row2 = rows.next().unwrap();
    let row3 = rows.next().unwrap();
    assert!(rows.next().is_none());
    
    assert_eq!(row1.cells, [red,  None, blue, None] );
    assert_eq!(row2.cells, [None, red,  blue, red] );
    assert_eq!(row3.cells, [None, None, None, None] );
}

#[test]
fn test_columns_iterator() {
    let red = Some(Player::Red);
    let blue = Some(Player::Blue);
    let cells = &[red,  None, blue, None, 
    			  None, red,  blue, red, 
    			  None, None, None, None,];
    			  
    let mut columns = Columns::new(&cells);
    let column1 = columns.next().unwrap();
    let column2 = columns.next().unwrap();
    let column3 = columns.next().unwrap();
    let column4 = columns.next().unwrap();
    assert!(columns.next().is_none());
    
    assert_eq!(column1.cells, [red,  None, None] );
    assert_eq!(column2.cells, [None, red,  None] );
    assert_eq!(column3.cells, [blue, blue, None] );
    assert_eq!(column4.cells, [None, red,  None] );
}

#[test]
fn test_diagonal_iterator() {
    let red = Some(Player::Red);
    let blue = Some(Player::Blue);
    let cells = &[red,  None, blue, None, 
    			  None, red,  blue, red, 
    			  None, None, None, None,];
    			  
	let mut diagonals = Diagonals::new(&cells);
	let d1 = diagonals.next().unwrap();
	let d2 = diagonals.next().unwrap();
	let d3 = diagonals.next().unwrap();
	let d4 = diagonals.next().unwrap();
	
	let d5 = diagonals.next().unwrap();
	let d6 = diagonals.next().unwrap();
	let d7 = diagonals.next().unwrap();
	let d8 = diagonals.next().unwrap();
	
	assert!( diagonals.next().is_none() );
	
	fn check(d: Diagonal, cells: [Option<Player>; WIDTH + HEIGHT], length: usize) {
	    assert_eq!((d.cells, d.length), (cells, length) );
	}
	
	check(d1, [None, None, None, None, None, None, None], 2);
	check(d2, [blue, red, None, None, None, None, None], 3);
	check(d3, [None, blue, None, None, None, None, None], 3);
	check(d4, [red, None, None, None, None, None, None], 2);
	
	check(d5, [None, None, None, None, None, None, None], 2);
	check(d6, [red, red, None, None, None, None, None], 3);
	check(d7, [None, blue, None, None, None, None, None], 3);
	check(d8, [blue, red, None, None, None, None, None], 2);
	
}
