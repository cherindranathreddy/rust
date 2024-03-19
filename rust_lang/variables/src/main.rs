fn main() {
    let mut x = 5;
    println!("The Value of x is: {x}");
    x = 6;
    println!("The value of x is: {x} and value of x+1 is:{}. The value of x+2 is:{}", x+1, x+2);

    const THREE_HOURS_IN_SECONDS : i32 = 60*60*3;
    println!("The constant value is: {THREE_HOURS_IN_SECONDS}");

    {
        const THREE_HOURS_IN_SECONDS : u32 = 60*60*3;
        println!("The constant value is=> {THREE_HOURS_IN_SECONDS}")
        println!("My first rust commit");
    }

    println!("Shadowing -> ability to bind different value or different type of value to varibale which has a same name within the scope");
    println!("difference between mut & shadowing is: in shadowing you always have to use let because we are assigned a new vairable & because of this the new variable has different type");
    println!("assigning variables which are not in scope can be done like shown in above const example. like other languages");
}
