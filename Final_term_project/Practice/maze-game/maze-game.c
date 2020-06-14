//made by Arthur 2020.06.12
#include <stdio.h>
#define row 4
#define column 4

int i, j;   //for loops

int maze[row][column] = {  {1, 1, 1, 1},    //this is my maze
                            {0, 0, 1, 1},
                            {1, 0, 0, 0},
                            {1, 1, 1, 1}    };

int player_position[2] = {1, 0};    //player's position
char movement[] = "";               //this variable's value will be decided by player

void instruction(); //it shows the method to play this game
void maze_status(); //it prints the current status in the first place and every time player make movement
void player_move(); //it implements the movement that player wants to make

int main(){ //main

    instruction();
    maze_status();

    while(1){                           //this loop will end as long as player wants to
        printf("your next move -> ");
        scanf("%s", &movement);
        printf("\n");
        if(strstr(movement, "-1")) break;

        if(!(strstr(movement, "up") || strstr(movement, "down") || strstr(movement, "right") || strstr(movement, "left"))){
            printf("%s\n", movement);
            printf("please enter the right code\n");
            for(i = 0; i < 5; i++){
                movement[i] = "";
            }
            continue;
        }

        player_move();
        instruction();
        maze_status();
        if(player_position[0] == 2 && player_position[1] == 3){ //if player reach the exit, the game will stop
            printf("SUCCESS!\n");
            break;
        }
    }

    return 0;
}

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

void player_move(){ //using strstr() to know which movement that player wants to make, and change his/her position
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
