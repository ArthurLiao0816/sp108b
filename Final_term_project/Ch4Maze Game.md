# [Maze Game](README.md#rust-learning)
---

## process
1. create a new project directory
    * `cargo new maze-game`
    * `cd maze-game`
    * `tree . /f`
    ![cargo_screenshot_maze-game_cargo-new](Picture/cargo_screenshot_maze-game_cargo-new.png)<br><br>

2. write a maze game with **C** first
    * If you wanna make a maze game, you need a maze. This is my maze.
        ```
        int maze[row][column] = {  {1, 1, 1, 1},    
                                    {0, 0, 1, 1},
                                    {1, 0, 0, 0},
                                    {1, 1, 1, 1}    };
        ```

    * Then, I need to know where is my player, so I save its position within a matrix.
        ```
        int player_position[2] = {1, 0};
        ```

    * So far, both of my maze and player are just datas. If people wanna play it, they won't know how the status in the maze after they make a movement. So, I make the game visualized by using '+', '*', and ' ', as obstacles, player, and path, respectively.
        ```
        void maze_status(){
            for(i = 0; i < row; i++){
                for(j = 0; j < column; j++){
                    if((i == player_position[0]) && (j == player_position[1])){
                        printf("*");
                    }
                    else if(maze[i][j] == 1){
                        printf("+");
                    }
                    else printf(" ");
                }
                printf("\n");
            }
            printf("\n");

            return 0;
        }
        ```
    
    * If people wanna play this game, they need to make movement by entering specific instructions. So I make an instruction function to let them know options they have.
        ```
        void instruction(){
            printf("=================\n");
            printf("direction | code\n");
            printf("=================\n");
            printf("upward    | up\n");
            printf("downward  | down\n");
            printf("rightward | right\n");
            printf("leftward  | left\n");
            printf("-----------------\n");
            printf("quitting  | -1\n");
            printf("=================\n\n");

            return 0;
        }
        ```
    
    * When player makes a movement, its position needs to be changed.
        ```
        void player_move(){
            if(strstr(movement, "up") && maze[player_position[0] - 1][player_position[1]] != 1){
                player_position[0] -= 1;
            }
            else if(strstr(movement, "down") && maze[player_position[0] + 1][player_position[1]] != 1){
                player_position[0] += 1;
            }
            else if(strstr(movement, "right") && maze[player_position[0]][player_position[1] + 1] != 1){
                player_position[1] += 1;
            }
            else if(strstr(movement, "left") && maze[player_position[0]][player_position[1] - 1] != 1){
                player_position[1] -= 1;
            }
            else return 0;
            printf("%d %d\n", player_position[0], player_position[1]);

            for(i = 0; i < 5; i++){
                movement[i] = "";
            }

            return 0;
        }
        ```
        <br>

3. then we transfer this game from **C** to **Rust**
    * maze
        ```
        let maze : [[usize; 4]; 4] = [[1, 1, 1, 1],
                                        [0, 0, 1, 1],
                                        [1, 0, 0, 0],
                                        [1, 1, 1, 1]];
        ```

    * player
        ```
        let mut player_position : [usize; 2] = [1, 0];
        ```
    
    * direction that player wanna head to
        ```
        let mut movement : &str = "";
        ```

    * instruction function
        ```
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
        ```

    * status of current maze
        ```
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
        ```

    * I find out the string I input is always two-bits longer than codes such as `up`, `down`, ...<br>
    so I use `input` to read the input string, and then store wanted part to `movement`
        ```
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Fail to read input.");
        movement = &input[0..(input.len() - 2)];
        ```
    
    * condition statements about player's movement
        ```
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
        ```

    * error message under the circumstance that player enter wrong codes
        ```
        else {
            println!("{}", movement);
            println!("please enter the right code");
            continue;
        }
        ```

    * success message shows up when player arrives the exit
        ```
        if player_position[0] == 2 && player_position[1] == 3 {
            print!("SUCCESS!\n");
            break;
        }
        ```
    
    * main function
        ```
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
                println!("movement.len() = {}", movement.len());

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
        ```

## result
* normal playing process<br>
    * initial maze<br><br>
    ![cargo_screenshot_maze-game_initial_maze](Picture/cargo_screenshot_maze-game_initial_maze.png)

    * first right<br><br>
    ![cargo_screenshot_maze-game_1st_right](Picture/cargo_screenshot_maze-game_1st_right.png)

    * second down<br><br>
    ![cargo_screenshot_maze-game_2ed_down](Picture/cargo_screenshot_maze-game_2ed_down.png)

    * third right<br><br>
    ![cargo_screenshot_maze-game_3rd_right](Picture/cargo_screenshot_maze-game_3rd_right.png)

    * 4th right and success<br><br>
    ![cargo_screenshot_maze-game_4th_right_and_success](Picture/cargo_screenshot_maze-game_4th_right_and_success.png)

* enter wrong code<br>
    ![cargo_screenshot_maze-game_wrong_code](Picture/cargo_screenshot_maze-game_wrong_code.png)

* quit game before success<br>
    ![cargo_screenshot_maze-game_quit_game](Picture/cargo_screenshot_maze-game_quit_game.png)


## [References](References.md#Ch5.)