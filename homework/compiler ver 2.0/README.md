### compiler ver 2.0
***
#### version introduction
* This is an updated version. Compared to previous versions, we added the initial functionality of IF (). Currently, this function can only be used to compile conditional statements consisting of an "if" and an "else".
* Please note that this version of the IF () function does not yet contain an "else if" conditional statement, we will add it as soon as possible.
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
