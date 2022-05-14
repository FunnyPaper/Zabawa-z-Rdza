trait ToInt {
    fn to_int(&self) -> i32;
}

impl ToInt for f64 {
    fn to_int(&self) -> i32 { *self as i32 }
}

fn main() {
    println!("5.1 to int is {}", 5.1.to_int());   
}
