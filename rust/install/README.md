# 安裝RUST

(*註：由於之前安裝RUST時未截圖，所以以下的圖皆來自[傻瓜教程](https://www.runoob.com/rust/rust-setup.html))

## 第一步

1.下載 [RUST](https://www.rust-lang.org/zh-CN/tools/install)

2.下載 [Visual Studio Code](https://code.visualstudio.com/Download)

*請下載這個

<img src = "https://github.com/syuan0327/sp108b/blob/master/rust/install/1.JPG" width=50% height=50%>

## 第二步

1.執行 rustup-init 文件

<img src = "https://github.com/syuan0327/sp108b/blob/master/rust/install/2.JPG" width=70% height=70%>

2.安裝

(1)如果你已經安裝 MSVC，輸入 1 ，直接進入第二步。

(2)如果你安裝的是 MinGW，輸入 2 （自訂安裝），然後系統會詢問你 Default host triple? ，請將 default host triple 改成以下的樣子

<img src = "https://github.com/syuan0327/sp108b/blob/master/rust/install/3.JPG" width=70% height=70%>

剩下的屬性請按照上圖去更改，更改後請按1

## 第三步

1.確認安裝完畢

輸入：rustc -V  

如果有跑出跟以下一樣，就代表安裝成功

<img src = "https://github.com/syuan0327/sp108b/blob/master/rust/install/4.JPG" width=50% height=50%>



