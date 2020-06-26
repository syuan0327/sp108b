# 迴圈
這裡利用所學的迴圈程式寫一些範例，和C與RUST之間的對比

## 程式碼
## while
#### C:
```
int main() {
    int num = 10;
    while(num != 5){
        printf("%d\n", num);
        num -= 1;
    }
    printf("end:%d\n",num);
}
```
#### RUST:
```
fn main() {
    let mut number = 10;
    while number != 5 {
        println!("{}", number);
        number -= 1;
    }
    println!("end:{}",number);
}
```
#### 執行結果：
<img src ="https://github.com/syuan0327/sp108b/blob/master/rust/for/1.JPG" width=50% height=50%>

## for
#### C:
```
int main()
{
    int a[] = {1, 2, 3};
    for(int i=0;i<=2;i++)
    {
        printf("a[%d] : %d\n", i,a[i]);
    }
}
```
#### RUST:
```
fn main() {
    let a = [1, 2, 3];
    for i in 0..3 {
        println!("a[{}] : {}", i,a[i]);
    }
}
```
#### 執行結果：
<img src ="https://github.com/syuan0327/sp108b/blob/master/rust/for/2.JPG" width=50% height=50%>

## loop
#### RUST:
```
fn main() {
    let a= [1,2,3,4,5];
    let mut i = 0;
    loop {
        let ch = a[i];
        if ch == 0 {
            break;
        }
        println!("{}", ch);
        i += 1;
    }
}
```
#### 執行結果：
<img src ="https://github.com/syuan0327/sp108b/blob/master/rust/for/3.JPG" width=50% height=50%>

## 說明:

這裡比較不一樣的地方是

1.在RUST裡while後面不用()

2.for的部分和python的寫法較相近(※註：這裡只提出python是因為目前只比較熟悉python)

3.Rust有原生的無限循環結構—— loop，可以通過 break類似於return一樣使整個迴圈退出並給予外部一個返回值。
