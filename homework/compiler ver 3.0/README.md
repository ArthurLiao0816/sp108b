### compiler ver 3.0
***
#### version introduction
* This is an updated version. Compared to previous versions, compiler ver 3.0 could not only compile intermediate representation, but also run program!!!
* Please note that this version of the IF () function does not yet contain an "else if" conditional statement, we will add it as soon as possible.
* [Home](https://github.com/ArthurLiao0816/sp108b#system-program-course----homework-project)
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
1. After u done programming, type "make" in terminal to compile multiple ".C" files at the same time.
    This is what ya ganna get-> gcc -std=c99 -O0 lexer.c compiler.c main.c -o compiler
2. Type "./compiler test/homework_t.c -ir -run" to demo ur code.

#### Result:
```
PS C:\Users\ldhsi\Desktop\系統程式\sp108b\homework\compiler ver 3.0> make
gcc -std=c99 -O0 ir.c irvm.c map.c util.c lexer.c compiler.c main.c -o compiler
PS C:\Users\ldhsi\Desktop\系統程式\sp108b\homework\compiler ver 3.0> ./compiler test/homework_t.c -ir -run
=======irDump()==========
00: t1 = 3
01: a = t1
02: t1 = 5
03: b = t1
04: t2 = a
05: t3 = b
06: t4 = t2 > t3
07: ifnot t4 goto L1
08: t1 = a
09: t = t1
10: goto L2
11: (L1)
12: t1 = b
13: t = t1
14: (L2)
===================irRun()=======================
00: t1 = 3 (3)
01: a = t1 (3)
02: t1 = 5 (5)
05: t3 = b (5)
06: t4 = t2 > t3 (0)
07: ifnot t4 (0) goto L1 (11)
12: t1 = b (5)
13: t = t1 (5)
14: (L2) (14)
```
