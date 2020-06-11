//made by Arthur 2020.06.12
#include <stdio.h>
#define row 4
#define column 4

int i, j;

int maze[row][column] = {  {1, 1, 1, 1},
                            {0, 0, 1, 1},
                            {1, 0, 0, 0},
                            {1, 1, 1, 1}    };

int player_position[2] = {1, 0};
char movement[] = "";

void instruction();
void maze_status();
void player_move();

int main(){

    instruction();
    maze_status();

    while(1){
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
        if(player_position[0] == 2 && player_position[1] == 3){
            printf("SUCCESS!\n");
            break;
        }
    }
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
}

void maze_status(){
    for(i = 0; i < row; i++){
        for(j = 0; j < column; j++){
            if((i == player_position[0]) && (j == player_position[1])){
                printf("��");
            }
            else if(maze[i][j] == 1){
                printf("��");
            }
            else printf("�@");
        }
        printf("\n");
    }
    printf("\n");

    return 0;
}

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
