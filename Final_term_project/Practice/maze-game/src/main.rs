///made by Arthur
///2020.06.15

use std::io::stdin;

fn main() {

    //setups
    let maze : [[usize; 4]; 4] = [[1, 1, 1, 1],
                                    [0, 0, 1, 1],
                                    [1, 0, 0, 0],
                                    [1, 1, 1, 1]];

    let mut player_position : [usize; 2] = [1, 0];
    let mut movement : &str = "";

    //game
    instruction();
    maze_status(maze, player_position);

    loop {

        movement = "";
        println!("your next move -> ");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Fail to read input.");
        movement = &input[0..(input.len() - 2)];

        //debugging block
        //println!("movement.len() = {}", movement.len());

        //player_move(maze, player_position, movement, code);
        if (movement == "up") && ((player_position[0] - 1) <= 3) {
            if maze[player_position[0] - 1][player_position[1]] != 1 {
                player_position[0] -= 1;
            }
        }
        else if (movement == "down") && ((player_position[0] + 1)) <= 3 {
            if maze[player_position[0] + 1][player_position[1]] != 1 {
                player_position[0] += 1;
            }
        }
        else if (movement == "right") && ((player_position[1] + 1) <= 3) {
            if maze[player_position[0]][player_position[1] + 1] != 1 {
                player_position[1] += 1;
            }
        }
        else if (movement == "left") && ((player_position[1] - 1) <= 3) {
            if maze[player_position[0]][player_position[1] - 1] != 1 {
                player_position[1] -= 1;
            }
        }
        else if movement == "-1" {
            break;
        }
        else {
            //println!("{}", movement);
            println!("you entered {}, please enter the right code", movement);
            continue;
        }

        instruction();
        maze_status(maze, player_position);

        if player_position[0] == 2 && player_position[1] == 3 {
            print!("SUCCESS!\n");
            break;
        }
    }
}

fn instruction() {
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
}

fn maze_status(maze : [[usize; 4]; 4], player_position : [usize; 2]) {

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
}