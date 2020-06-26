# cargo 
Cargo 是 Rust 的構建系統和包管理器。大多數 Rustacean 們使用 Cargo 來管理他們的 Rust 專案，因為它可以為你處理很多工，比如構建代碼、下載依賴庫並編譯這些庫。

（代碼所需要的庫叫做依賴（dependencies））。
## 使用方法
#### 一、
```
cargo new hi
```
(*註：此處的hi為RUST的工程目錄名稱，所以可以任意取名)

打好後會出現名為hi的工程目錄

目錄：

<img src = "https://github.com/syuan0327/sp108b/blob/master/rust/hi/1.JPG" width=30% height=30%>

#### 二、
```
cd ./hi
cargo build 
cargo run 
```
cd ./hi => 進到hi目錄

cargo build => 系統在創建工程時會生成一個 Hello, world 來源程式 main.rs

目錄：

<img src = "https://github.com/syuan0327/sp108b/blob/master/rust/hi/2.JPG" width=30% height=30%>

終端機：

<img src = "https://github.com/syuan0327/sp108b/blob/master/rust/hi/3.JPG" width=50% height=50%>

cargo run => 編譯並運行

<img src = "https://github.com/syuan0327/sp108b/blob/master/rust/hi/4.JPG" width=50% height=50%>
