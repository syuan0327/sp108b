# 為 05-compiler-run 加上 IF 的語法

## IF語法程式碼
```
void IF() {
  int ifbegin = nextLabel();
  int ifend = nextLabel();
  skip("if");
  skip("(");
  int e = E();
  //此處是把emitnotgoto改成iremitnotgoto
  irEmitIfNotGoto(e, ifbegin);
  skip(")");
  STMT();
  //此處是把emit改成iremitgoto
   irEmitGoto(ifend);
  //標記部分改為 irEmitLabel
  irEmitLabel(ifbegin);
  if (isNext("else")) {
    skip("else");
    STMT();
  }
  irEmitLabel(ifend);
}
```
## 執行檔案方式
1.開啟Terminal在下方輸入mingw32-make來執行C語言的檔案

2.執行完後，會跑出一個exe檔

3.輸入想要執行的檔案
=>./compiler test/if.c -ir -ru   
4.即可執行

_*mingw32-make用來執行 Makefile, 呼叫 gcc 去建置專案的一種工具。_

## 執行結果
```
=======irDump()==========
00: t1 = 3
01: a = t1
02: t1 = 5
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
PS C:\Users\Owner\Desktop\110710520-1\sp108b\05-compiler-run> ./compiler test/if.c -ir -ru   
=======irDump()==========
00: t1 = 3
01: a = t1
02: t1 = 5
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
```
