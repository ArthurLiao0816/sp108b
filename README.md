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
* [compiler ver 2.0](#compiler_ver_2.0)
* [compiler ver 3.0](#compiler_ver_3.0)

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

### compiler ver 2.0
#### version introduction
* This is an updated version. Compared to previous versions, we added the initial functionality of IF (). Currently, this function can only be used to compile conditional statements consisting of an "if" and an "else".
* Please note that this version of the IF () function does not yet contain an "else if" conditional statement, we will add it as soon as possible, so stay tuned for the next version update.
#### testing sample:(path->test/homework_t.c)
```
a = 3;
b = 5;
if (a > b){
    t = a;
}
else{
   t = b;
}
```

#### testing method:
1. After u done programming, type "make" in terminal to compile ur code.
    This is what ya ganna get-> gcc -std=c99 -O0 lexer.c compiler.c main.c -o compiler
2. Type "./compiler test/homework_t.c" to demo ur code.
3. Result:
```
a = 3;     
b = 5;     
if (a > b){
    t = a; 
}
else{      
   t = b;
}
========== lex ==============
token=a
token==
token=3
token=;
token=b
token==
token=5
token=;
token=if
token=(
token=a
token=>
token=b
token=)
token={
token=t
token==
token=a
token=;
token=}
token=else
token={
token=t
token==
token=b
token=;
token=}
========== dump ==============
0:a
1:=
2:3
3:;
4:b
5:=
6:5
7:;
8:if
9:(
10:a
11:>
12:b
13:)
14:{
15:t
16:=
17:a
18:;
19:}
20:else
21:{
22:t
23:=
24:b
25:;
26:}
============ parse =============
t0 = 3  //a = 3
a = t0
t1 = 5  //b = 5
b = t1
t2 = a
t3 = b
t4 = t2 > t3    //t4 is the boolean of (a > b)
if t4 isn't True goto L0    //if t4 is false, goto L0 to do else block
t5 = a
t = t5
goto L1 //if the "if" block is done, end if
(L0)
t6 = b
t = t6
(L1)
```

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