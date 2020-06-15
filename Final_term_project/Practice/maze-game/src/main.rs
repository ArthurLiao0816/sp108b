///made by Arthur
///2020.06.15

use std::io::stdin;

fn main() {
    //setups
    let mut i : i32 = 0;
    let mut j : i32 = 0;

    let mut maze : [[usize; 4]; 4] = [[1, 1, 1, 1],
                                    [0, 0, 1, 1],
                                    [1, 0, 0, 0],
                                    [1, 1, 1, 1]];

    let mut player_position : [usize; 2] = [1, 0];
    let code : [&str; 6] = ["up", "down", "right", "left", "-1", ""];
    let mut movement = String::new();

    //game
    instruction();
    maze_status(maze, player_position);

    loop {
        println!("your next move -> ");
        stdin().read_line(&mut movement).expect("Fail to read input.");
        //let movement = readline::readline("");
        if movement == "-1" {
            break;
        }

        if !right_code(&movement, code) {
            println!("{}", movement);
            println!("please enter the right code");
            movement = String::from("");
            continue;
        }

        player_move(maze, player_position, movement, code);
        movement = String::from("");
        instruction();
        maze_status(maze, player_position);
        if player_position[0] == 2 && player_position[1] == 3 {
            print!("SUCCESS!\n");
            break;
        }
    }
}

fn right_code(movement : &String, code : [&str; 6]) -> bool {
    for i in code.iter() {
        match &movement {
            i => return true,
                
            _ => return false,
        }
    }

    return false;
}

fn instruction() -> i32 {
    println!(
"=================
direction | code
=================
upward    | up
downward  | down
rightward | right
leftward  | left
-----------------
quitting  | -1
=================");

    return 0;
}

fn maze_status(maze : [[usize; 4]; 4], player_position : [usize; 2]) -> i32 {
    for i in 0..4 {
        let i = i as usize;
        for j in 0..4 {
            let j = j as usize;
            if (i == player_position[0]) && (j == player_position[1]) {
                print!("*");
            }
            else if maze[i][j] == 1 {
                print!("+");
            }
            else {
                print!(" ");
            }
        }
        print!("\n");
    }
    print!("\n");

    return 0;
}

fn player_move(maze : [[usize; 4]; 4], mut player_position : [usize; 2], mut movement : String, code : [&str; 6]) -> i32{
    
    
    /*match &*movement {
        "up" => if (maze[player_position[0] - 1][player_position[1]] != 1) {
                        player_position[0] -= 1;
                },
        "down" => if (maze[player_position[0] + 1][player_position[1]] != 1) {
                        player_position[0] += 1;
                },
        "right" => if (maze[player_position[0]][player_position[1] + 1] != 1) {
                        player_position[1] += 1;
                },
        "left" => if (maze[player_position[0]][player_position[1] - 1] != 1) {
                        player_position[1] -= 1;
                },
        _ => println!("player_move() -> been there"),
    }*/
    
    /*
    if (&movement == code.iter(0)) && (maze[player_position[0] - 1][player_position[1]] != 1) {
        player_position[0] -= 1;
    }
    else if (&movement == code[1]) && (maze[player_position[0] + 1][player_position[1]] != 1) {
        player_position[0] += 1;
    }
    else if (&movement == code[2]) && (maze[player_position[0]][player_position[1] + 1] != 1) {
        player_position[1] += 1;
    }
    else if (&movement == code[3]) && (maze[player_position[0]][player_position[1] - 1] != 1) {
        player_position[1] -= 1;
    }
    else {
        return 0;
    }
    */
    println!("{} {}\n", player_position[0], player_position[1]);

    return 0;
}