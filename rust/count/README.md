# let 和 println!

## 說明
這裡把let和println放在一起實作，是因為我覺得let和println!容易理解!

## 程式碼：

C:

```
int main()
{
    int x = 1;
    x = x+1 ;
    x = x*2 ;
    printf("%d", x);
}
```
RUST:

```
fn main() {
    let x = 1;
    let x = x+1;
    let x = x*2;
    println!("ans:{}",x);
}

```
#### 說明：
(l)

let => 在rust裡let為宣告變數，相當於C的int

但是let後的變數具有不可變性，所以必須在let後加上mut

mut => 這是一個可變的（mutable）變數綁定。 當一個綁定是可變的，代表你被允許改變它所指向的內容。

程式碼：
```
fn main() {
    let mut x = 5;
    let y=2;
    x = x+y;
    println!("{}",x);
}
```
結果：7

(2)println!() or print!()=> 列印

#### 語法：
println!("The value of x is: {}", x);

#### 解釋：

{} => 相當於c語言裡的 % + 字母




