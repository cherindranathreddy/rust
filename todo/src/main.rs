use std::io;
mod lib;
use lib::Task as Task;
use lib::run as run;

fn main() 
{
  let mut todo : Vec<Task> = Vec::new();
  runprompt(&mut todo);
}

fn runprompt(todo : &mut Vec<Task>)
{
    loop
    {
        println!("Enter Todo list:");
        
        let mut buffer = String :: new();
        io::stdin().read_line(&mut buffer).expect("cannot read");
        
        let args : Vec<&str> = buffer.trim().split(" ").collect();
        run(args, todo);
    }
}
