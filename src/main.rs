fn main() {
    let some_number = 3;
    println!("some_number: {}", some_number);

    let some_number  = something(&some_number);
    println!("number * 2: {}", &some_number);
}

fn something(x: &i32) -> i32 {
      x * 2
}
