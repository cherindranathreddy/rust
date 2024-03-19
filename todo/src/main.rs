use std::io;

fn main() 
{
  runprompt();
}

fn runprompt()
{
    loop
    {
        println!("Enter Todo list:");
        
        let mut buffer = String :: new();
        io::stdin().read_line(&mut buffer).expect("cannot read");
        
        let args : Vec<&str> = buffer.split(" ").collect();
        run(args);
        break;
    }
}

fn run(args : Vec<&str>)
{
    for s in args
    {
        println!("{s}");
    }
}
