fn main() {
    let name: &str = "Alice";
    let age = 24;
    print_name(name, age)
}

fn print_name(name: &str, age: i32){
    println!("My name is {} and I am {}", name, age);
}