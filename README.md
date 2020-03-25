# 系統程式課程 -- 習題專案

欄位 | 內容
-----|--------
學期 | 108 學年度下學期
學生 |  廖明志
學號末兩碼 | 29
教師 | [陳鍾誠](https://misavo.com/blog/%E9%99%B3%E9%8D%BE%E8%AA%A0)
學校科系 | [金門大學資訊工程系](https://www.nqu.edu.tw/educsie/index.php)
課程首頁 | [系統程式](https://misavo.com/blog/%E9%99%B3%E9%8D%BE%E8%AA%A0/%E8%AA%B2%E7%A8%8B/%E7%B3%BB%E7%B5%B1%E7%A8%8B%E5%BC%8F)

## Table of Contents 
* [Compiler](#Compiler)
* [Note](#Note)
* [compiler ver 2.0](https://github.com/ArthurLiao0816/sp108b/blob/master/homework/compiler%20ver%202.0/README.md#compiler-ver-20)
* [compiler ver 3.0](#compiler-ver-30)

### Compiler

#### grammar

```
PROG = STMTS
BLOCK = { STMTS }
STMTS = STMT*
STMT = IF | WHILE | BLOCK | ASSIGN
WHILE = while (E) STMT
IF = if (E) STMT (else if (E) STMT else STMT)?
ASSIGN = id '=' E;
E = F (op E)*
F = (E) | Number | Id
```
### Note
* If "git pull" failed, "git stash" first then git pull again.
* "#define trace printf" is a better way to modularize program if we need.
* ".c" and ".h" files usually contain program and "define", respectively.

### compiler ver 3.0

#### testing sample:(path->test/homework_t2.c)
```
a = 3;
b = 5;
if (a > b){
    t = a;
}
else if(a == b){
    printf("There is no bigger one.");
}
else{
   t = b;
}
```
