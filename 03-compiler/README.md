# 為編譯器加上IF語法

## IF語法程式碼
```
// IF = if (E) STMT ( else STMT )?else是可選所以用括號和問號
void IF() {
  int ifbegin = nextLabel();//產生標記用nextLabel()
  int ifend = nextLabel();
  emit("(L%d)\n", ifbegin);
  skip("if");
  skip("(");
  int e = E();
  emit("if not t%d goto L%d\n", e, ifend);
  skip(")");
  STMT();
  emit("(L%d)\n", ifbegin);
  if(isNext("else")){//如果後面是else比對else
    skip("else");
    STMT();
  }
  emit("(L%d)\n", ifend);
}
```
## 範例程式
```
a = 3;
b = 5;
if (a > b)
t = a+b;
else
t = a-b;
```
## 執行檔案方式
1.開啟Terminal在下方輸入mingw32-make來執行C語言的檔案

![image](https://github.com/syuan0327/sp108b/blob/master/03-compiler/23.JPG)

2.執行完後，會跑出一個exe檔

3.輸入想要執行的檔案

![image](https://github.com/syuan0327/sp108b/blob/master/03-compiler/34.JPG)

4.即可執行

_*mingw32-make用來執行 Makefile, 呼叫 gcc 去建置專案的一種工具。_

## 執行結果
```
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
token=t
token==
token=a
token=+
token=b
token=;
token=else
token=t
token==
token=a
token=-
token=b
token=;
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
14:t
15:=
16:a
17:+
18:b
19:;
20:else
21:t
22:=
23:a
24:-
25:b
26:;
============ parse =============
t0 = 3
a = t0
t1 = 5
b = t1
(L0)
t2 = a
t3 = b
t4 = t2 > t3
if not t4 goto L1
t5 = a
t6 = b
t7 = t5 + t6
t = t7
(L0)
t8 = a
t9 = b
t10 = t8 - t9
t = t10
(L1)
```
## 遇到的問題
在執行檔案的過程中，前面都很順利，但是到最後面會跑出這行文字

![image](https://github.com/syuan0327/sp108b/blob/master/03-compiler/5.JPG)

後來再次輸入mingw32-make後出現了以下狀況

![image](https://github.com/syuan0327/sp108b/blob/master/03-compiler/6.JPG)

這個Error代表著我定義了兩次While所以只要將多定義的那個刪除就可以了
