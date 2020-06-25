# 條件句

這裡是講述條件句if,else if和else在RUST與C之間的差別

## 程式碼：

#### RUST:

```
fn main() {
    let a = 12;
    let b;
    if a > 0 {
       println!("{}>0",a);
    }  
    else if a < 0 {
        b=1;
        println!("b is {}",b);
    }  
}
```
#### C:
```
int main()
{
    int a = 12;
    int b;
    if (a > 0)
    {
       printf("%d>0", a);
    }
    else if (a < 0)
    {
        b=1;
        printf("b is %d", b);
    }
}
```

## 三元條件運算運算式 (A ? B : C)

```
if <condition> { block 1 } else { block 2 } 
```
範例：

```
fn main() {
    let a = 3;
    let b = 2;
    let num = if a > 0 {a} else {b};
    println!("{}", num);
}
```
顯示結果為 3

## 說明：

1.Rust中的 if 不用加 {} 

2.在 Rust 中可以使用 if-else 結構實現類似於三元條件運算運算式 (A ? B : C) 的效果


