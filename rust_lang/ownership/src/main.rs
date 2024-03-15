fn main() {
    println!("understanding ownership");
    
    let s2 = gives_ownership();
    println!("Got the ownership of this string {s2}");

    let s1 = String::from("Hello");
    let s3 = take_and_give_ownership(s1);
    println!("Got back the ownership of variable s1 in form of s2: {s3}");
    take_ownership(s3);

    let num = 12;
    makes_copy(num);
    println!("{} still i have ownership to this number", num);
}

fn take_ownership(some_string: String)
{
    println!("string ownership must be this, {some_string}");
}

fn makes_copy(some_int:i32)
{
    println!("since int is scalar type it must be copy, {}", some_int);
}

fn gives_ownership() -> String {
    let s = String::from("world");
    s
}

fn take_and_give_ownership(s:String) -> String {
    s
}
