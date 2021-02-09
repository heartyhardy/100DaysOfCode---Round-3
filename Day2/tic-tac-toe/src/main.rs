use std::io;

const ROW:u8 = 0;
const COL:u8 = 1;

const PLAYER1:u8 = 0;
const PLAYER2:u8 = 1;

fn main() {
    let mut player_moves: u8 = 0;

    println!();
    println!("TIC TAC TOE");
    println!("___________");
    println!();

    let mut matrix:[[u8; 3]; 3] = [[0,0,0],[0,0,0],[0,0,0]];

    show_board(&mut matrix);

    loop{
        //Player 1 Move
        if !player_move(&mut matrix, PLAYER1){
            continue;
        }
        player_moves+=1;

        match check_win_conditions(&mut matrix, player_moves){
            Err(()) => (),
            Ok(player) => {
                println!();
                println!("Player {} wins!", player);
                show_board(&mut matrix);
                break;           
            }
        }

        show_board(&mut matrix);

        //Player 2 Move
        if !player_move(&mut matrix, PLAYER2){
            continue;
        }
        player_moves+=1;
        
        match check_win_conditions(&mut matrix, player_moves){
            Err(()) => (),
            Ok(player) => {
                println!();
                println!("Player {} wins!", player);
                show_board(&mut matrix);
                break;
            }
        }

        // Handle a draw
        if player_moves == 6{
            println!("It's a draw!");
            show_board(&mut matrix);
        }

        show_board(&mut matrix);
    }


}

/*
    Display the current state of the board
    -- 0 represents Player 1
    -- X represents Player 2
*/
fn show_board(matrix: &mut[[u8; 3]; 3]){
    println!();
    for i in 0..3{
        for j in 0..3{
            match matrix[i][j]{
                0 => print!("{}  ","*"),
                1 => print!("{}  ", "0"),
                2 => print!("{}  ", "X"),
                _ => print!("")
            }
        }
        println!();
    }
    println!();
}

/*
    Player move
*/
fn player_move(matrix: &mut[[u8; 3];3], player: u8) -> bool{
    let row:usize = get_player_input(ROW);
    let col:usize = get_player_input(COL);

    if row == 404 || col == 404{
        return false;
    }
    
    return validate_move(matrix, row, col, player);
}

/*
    Check win conditions
*/
fn check_win_conditions(matrix: &mut[[u8; 3];3], player_moves:u8) -> Result<u8, ()>{
    if player_moves < 5 {
        return Err(());
    }

    let mut is_matching:bool = false;
    let mut player_id = 0;

    //Check ROWS for win conditions
    for i in 0..3{
        for j in 0..2{
            if matrix[j][i] == matrix[j+1][i]{
                is_matching = true;
            }else{
                is_matching = false;
                break;
            }
        }

        if is_matching == true{
            player_id = matrix[0][i];
            break;
        }else{
            is_matching = false;
        }
    }

    if is_matching{
        return Ok(player_id);
    }

    //Check COLS for win conditions
    for i in 0..3{
        for j in 0..2{
            if matrix[i][j] == matrix[i][j+1]{
                is_matching = true;
            }else{
                is_matching = false;
                break;
            }
        }

        if is_matching == true{
            player_id = matrix[i][0];
            break;
        }else{
            is_matching = false;
        }
    }

    if is_matching{
        return Ok(player_id);
    }

    // Diagonal check 1
    for i in 0..2{
        for j in 0..2{
            if i != j{
                continue;
            }            
            if matrix[i][j] == matrix[i+1][j+1]{
                is_matching = true;
            }else{
                is_matching = false;
                break;
            }
        }

        if is_matching == true{
            player_id = matrix[0][0];
        }else{
            is_matching = false;
            break;
        }
    }

    if is_matching{
        return Ok(player_id);
    }

    //Diagonal check 2
    for i in (1..3).rev(){
        for j in 0..3{
            if i + j != 2{
                continue;
            }
            println!("{} {} - {} {} - {} {}", i, j, i-1, j + 1, matrix[i][j], matrix[i-1][j+1]);
            if matrix[i][j] == matrix[i-1][j+1] {
                is_matching=true;
            }else{
                is_matching=false;
                break;
            }
        }

        if is_matching == true{
            player_id = matrix[2][0];
            //break;
        }else{
            is_matching = false;
            break;
        }
    }

    if is_matching{
        return Ok(player_id);
    }else{
        return Err(());
    }
}

/*
    Validate player move
*/
fn validate_move(matrix: &mut[[u8; 3];3], row:usize, col:usize, player: u8) -> bool{
    if (row+1 > 3 || row+1 <= 0) || (col+1 > 3 || col+1 <= 0){
        return false;
    }

    if matrix[row][col] != 0 {
        return false;
    }

    match player{
        PLAYER1 => matrix[row][col] = 1,
        PLAYER2 => matrix[row][col] = 2,
        _ => return false
    }

    return true;
}

/*
    Read input from stdin
*/
fn get_player_input(flags: u8) -> usize{
    let mut player_input = String::new();

    println!();
    println!("Enter your move");

    if flags == ROW{
        println!("ROW: ");
    }else if flags == COL{
        println!("COL: ");
    }

    io::stdin().read_line(&mut player_input)
        .expect("Failed to read your input!");
    
    let player_input:usize = match player_input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!();
            println!("Please input a number!");
            404
        },
    };
    
    return player_input;
}

