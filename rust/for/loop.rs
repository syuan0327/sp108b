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