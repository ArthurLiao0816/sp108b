# System Program Course -- Homework Project

Field | Content
-----|--------
Semester | Second semester of 2019
Student |  廖明志
ID number | 110710529
PHD | [陳鍾誠](https://misavo.com/blog/%E9%99%B3%E9%8D%BE%E8%AA%A0)
School&Department | [NQU CSIE](https://www.nqu.edu.tw/educsie/index.php)
Course Homepage | [System Program](https://misavo.com/blog/%E9%99%B3%E9%8D%BE%E8%AA%A0/%E8%AA%B2%E7%A8%8B/%E7%B3%BB%E7%B5%B1%E7%A8%8B%E5%BC%8F)

## Table of Contents 
* [Compiler](#Compiler)
* [Note](#Note)
* [compiler ver 2.0](https://github.com/ArthurLiao0816/sp108b/blob/master/homework/compiler%20ver%202.0/README.md#compiler-ver-20)
* [compiler ver 3.0](https://github.com/ArthurLiao0816/sp108b/blob/master/homework/compiler%20ver%203.0/README.md#compiler-ver-30)
* [PowerJitCall](https://github.com/ArthurLiao0816/sp108b/blob/master/homework/power/README.md#powerjitcall)
* [compiler ver 4.0](https://github.com/ArthurLiao0816/sp108b/tree/master/Project/MtmPrj#compiler-ver-40)

### Compiler

#### grammar

```
PROG = STMTS
BLOCK = { STMTS }
STMTS = STMT*
STMT = IF | WHILE | BLOCK | ASSIGN
WHILE = while (E) STMT
FOE = for ( E ; E ; E ) STMT
IF = if (E) STMT (else if (E) STMT else STMT)?
GOTO = goto [label name]
ASSIGN = id '=' E;
E = F (op E)*
F = (E) | Number | Id
```
### Note
* If "git pull" failed, "git stash" first then git pull again.
* "#define trace printf" is a better way to modularize program if we need.
* ".c" and ".h" files usually contain program and "define", respectively.
* If "git push origin master" failed, "git pull" first then git push again.
