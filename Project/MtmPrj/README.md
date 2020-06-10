### compiler ver 4.0
***
#### version introduction
* "compiler ver 4.0", aka my midterm project, is a HUGELY updated version. Compared to previous versions, we add "FOR ()" and "GOTO ()" functions. Well~ Well~ It sounds like someone have another option to execute ur code repeatedly besides "WHile ()" function. Oh! By the way, "GOTO ()" function is just like a double-edged blade. Don't use it unless it's worth a try. 
* Please note that the "IF ()" function of this version still has not yet contain an "else if" conditional statement, we will add it as soon as possible.
* [Home](https://github.com/ArthurLiao0816/sp108b#system-program-course----homework-project)

#### code:
* [Click Me ^^](https://github.com/ArthurLiao0816/sp108b/blob/master/Project/MtmPrj/compiler.c)

#### testing sample:
1. sample of "IF ()" (path->test/if.c)
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

2. sample of "FOR ()" (path->test/for.c)
```
a = 0;
for ( i = 1; i < 5; i = i + 1){
    a = a + 1;
}
```

3. sample of "GOTO ()" (path->test/goto.c)
```
a = 4;
b = 6;

goto goto1;

a = a + 1;

goto1:
a = a + b;
```

#### testing method:
1. After u done programming, type "make" in terminal to compile multiple ".c" files at the same time.
    This is what ya ganna get-> gcc -std=c99 -O0 ir.c irvm.c map.c util.c lexer.c compiler.c main.c -o compiler
2. Type "./compiler test/if.c -ir -run", "./compiler test/for.c -ir -run", and "./compiler test/goto.c -ir -run" to demo "if.c", "for.c", and "goto.c", respectively.

#### Result:
1. result of "IF ()"
```
PS C:\Users\ldhsi\Desktop\系統程式\sp108b\Project\MtmPrj> ./compiler test/if.c -ir -run 
=======irDump()==========
00: t1 = 3
01: a = t1
02: t1 = 5
03: b = t1
04: t2 = a
05: t3 = b
06: t4 = t2 > t3
07: if not t4 goto L1
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
03: b = t1 (5)
04: t2 = a (3)
05: t3 = b (5)
06: t4 = t2 > t3 (0)
07: ifnot t4 (0) goto L1 (11)
11: (L1) (11)
12: t1 = b (5)
13: t = t1 (5)
14: (L2) (14)
```

2. result of "FOR ()"
```
PS C:\Users\ldhsi\Desktop\系統程式\sp108b\Project\MtmPrj> ./compiler test/for.c -ir -run
=======irDump()==========
00: t1 = 0
01: a = t1    
02: t1 = 1    
03: i = t1    
04: (L1)      
05: t2 = i    
06: t3 = 4    
07: t4 = t2 < t3     
08: if not t4 goto L2
09: t1 = i
10: t2 = 1
11: t3 = t1 + t2     
12: i = t3
13: t1 = a
14: t2 = 1
15: t3 = t1 + t2
16: a = t3
17: goto L1
18: (L2)
===================irRun()=======================
00: t1 = 0 (0)
01: a = t1 (0)
02: t1 = 1 (1)
03: i = t1 (1)
04: (L1) (4)
05: t2 = i (1)
06: t3 = 4 (4)
07: t4 = t2 < t3 (1)
08: ifnot t4 (1)  -- fail
09: t1 = i (1)
10: t2 = 1 (1)
11: t3 = t1 + t2 (2)
12: i = t3 (2)
13: t1 = a (0)
14: t2 = 1 (1)
15: t3 = t1 + t2 (1)
16: a = t3 (1)
17: goto L1 (4)
04: (L1) (4)
05: t2 = i (2)
06: t3 = 4 (4)
07: t4 = t2 < t3 (1)
08: ifnot t4 (1)  -- fail
09: t1 = i (2)
10: t2 = 1 (1)
11: t3 = t1 + t2 (3)
12: i = t3 (3)
13: t1 = a (1)
14: t2 = 1 (1)
15: t3 = t1 + t2 (2)
16: a = t3 (2)
17: goto L1 (4)
04: (L1) (4)
05: t2 = i (3)
06: t3 = 4 (4)
07: t4 = t2 < t3 (1)
08: ifnot t4 (1)  -- fail
09: t1 = i (3)
10: t2 = 1 (1)
11: t3 = t1 + t2 (4)
12: i = t3 (4)
13: t1 = a (2)
14: t2 = 1 (1)
15: t3 = t1 + t2 (3)
16: a = t3 (3)
17: goto L1 (4)
04: (L1) (4)
05: t2 = i (4)
06: t3 = 4 (4)
07: t4 = t2 < t3 (0)
08: ifnot t4 (0) goto L2 (18)
18: (L2) (18)
```

3. result of "GOTO ()"
```
PS C:\Users\ldhsi\Desktop\系統程式\sp108b\Project\MtmPrj> ./compiler test/goto.c -ir -run
=======irDump()==========
00: t1 = 4
01: a = t1
02: t1 = 6
03: b = t1
04: goto L1
05: t1 = a
06: t2 = 1
07: t3 = t1 + t2
08: a = t3
09: (L1)
10: t1 = a
11: t2 = b
12: t3 = t1 + t2
13: a = t3
===================irRun()=======================
00: t1 = 4 (4)
01: a = t1 (4)
02: t1 = 6 (6)
03: b = t1 (6)
04: goto L1 (9)
09: (L1) (9)
10: t1 = a (4)
11: t2 = b (6)
12: t3 = t1 + t2 (10)
13: a = t3 (10)
```
 
```
我的編譯器擴充自老師的課堂範例， if, goto, for 語法則是新加入的。
if, goto, for 是我 100% 原創的。
參考老師的範例之後，我讀懂了，於是完全靠自己寫出 if, goto, for 擴充部分。
```