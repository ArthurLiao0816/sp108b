# [Interesting Things](README.md#rust-learning)

## Program reads input before printing
* there are 3 situations ...
    1. `println!("your next move -> ");`<br><br>
    * program reads printing before input ...
    ![cargo_screenshot_maze_game_println!](Picture/cargo_screenshot_maze_game_println!.jpg)
    2. `print!("your next move -> ");`<br>
        `print!("\n");`<br><br>
    * program reads printing before input ...
    ![cargo_screenshop_maze_game_print!_print!](Picture/cargo_screenshop_maze_game_print!_print!.jpg)
    3. `print!("your next move -> ");`<br><br>
    * Program reads input before printing ...
    ![cargo_screenshot_maze_game_print!](Picture/cargo_screenshot_maze_game_print!.jpg)

## [References](References.md#Ch5.)