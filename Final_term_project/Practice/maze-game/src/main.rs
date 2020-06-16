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
    let code : [&str; 5] = ["up", "down", "right", "left", "-1"];
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
        println!("movement.len() = {}", movement.len());

        //player_move(maze, player_position, movement, code);
        if (movement == "up") && (maze[player_position[0] - 1][player_position[1]] != 1) && ((player_position[0] - 1) <= 3) && ((player_position[0] - 1) >= 0) {
            player_position[0] -= 1;
        }
        else if (movement == "down") && (maze[player_position[0] + 1][player_position[1]] != 1) && ((player_position[0] + 1)) <= 3 && ((player_position[0] + 1) >= 0) {
            player_position[0] += 1;
        }
        else if (movement == "right") && (maze[player_position[0]][player_position[1] + 1] != 1) && ((player_position[1] + 1) <= 3) && ((player_position[1] + 1) >= 0) {
            player_position[1] += 1;
        }
        else if (movement == "left") && (maze[player_position[0]][player_position[1] - 1] != 1) && ((player_position[1] - 1) <= 3) && ((player_position[1] - 1) >= 0) {
            player_position[1] -= 1;
        }
        else if movement == "-1" {
            break;
        }
        else {
            println!("{}", movement);
            println!("please enter the right code");
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

    if player_position[0] > 3 || player_position[0] < 0 {
        println!("y-axis is out of range");
    }
    else if player_position[1] > 3 || player_position[1] < 0 {
        println!("x-axis is out of range");
    } 
    else {
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

    return 0;
}