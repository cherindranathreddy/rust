fn main() {
    println!("Referencing and borrowing");

    let s = String::from("hello");
    let len = length(&s);
    println!("The length of {s} is {len}");

    let mut ms = String::from("Cheri");
    append_string(&mut ms);
    println!("Mutable Reference: {}", ms);

    println!("Multiple mmutable references(&mut ms) of same variable are not allowed, because it may cause a race condition");
    println!("Mutiple references are a variable, but combination of references and mutable references are not allowed");

    println!("At any given point of time you can have either one mutable reference or many immutable references.");
    println!("References must be valid(No dangling pointers)");
}

fn length(s:&String) -> usize
{
    s.len()
}

fn append_string(s:&mut String)
{
   s.push_str(", world");
}
