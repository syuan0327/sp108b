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
    println!{"ans:{}",x};
}

```
let => 在rust裡let為宣告變數，相當於C的int

println!{} or print!{}=> 列印

#### 語法：
println!{"The value of x is: {}", x};

#### 解釋：

{} => 相當於c語言裡的 % + 字母

※必須注意的點是printf在C裡後面是加()，但在RUST裡println!是加{}




