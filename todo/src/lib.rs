use std::process;
use rand:: Rng;

pub struct Task
{
    task : String,
    done_status : bool,
    id : u64
}

impl Task
{
    fn update_status(&mut self)
    {
        self.done_status = true;
    }
    fn update_task(&mut self, new_task: String)
    {
        self.task = new_task;
    }
}

pub fn run(args : Vec<&str>, todo : &mut Vec<Task>)
{
    parse_arguments(args, todo);
}

fn parse_arguments(args : Vec<&str>, todo : &mut Vec<Task>)
{
    let operation = args[0];
    match operation 
    {
        "add" => {
            if let Some(value) = args.get(1)
            {
                let new_task = value;
                add_new_task(todo, new_task);
                display_todo(todo);
            }
            else {println!("No task has been entered")};
        },
        "show" => display_todo(todo),
        "update" => {
            let id:u64 = args[1].parse().expect("Failed to pass a number");
            if let Some(value) = args.get(1)
            {
                let updated_task = value;
                if let Ok(task) = get_task(todo, id)
                {
                    if let Some(update_task) = args.get(2)
                    {
                        let new_task = update_task;
                        task.update_task(new_task.to_string());
                    }
                    else {println!("enter the task to be updated")};
                }
                else {println!("No task is found in todo list with the id:{}", id)};
            }
            else {println!("No task id has been entered")};
        },
        "done" => {
            let id: u64 = args[1].parse().expect("Failed to parse id");
            if let Ok(task) = get_task(todo, id)
            {
                task.update_status();
            }
            else {println!("No task is found in todo list with id:{}", id)};
        },
        "delete" => {
            if let Some(value) = args.get(1)
            {
                let id = value.parse().expect("Failed to pass a id");
                remove_task(todo, id);
                display_todo(todo);
            }
            else {println!("id is not given")};
        }
        "exit" => {process::exit(0)},
        _ => {println!("Default process")},
    }
}

fn remove_task(todo: &mut Vec<Task>, id: u64)
{
    todo.retain(|task| task.id != id);
}

fn get_task(todo: &mut Vec<Task>, id: u64) -> Result <&mut Task, &str>
{
    for task in todo
    {
        if task.id == id {return Ok(task)};
    }
    return Err("Task not found in todo list"); 
}

fn display_todo(todo: &mut Vec<Task>)
{
    if todo.len() < 1 {println!("Empty Todo list")};
    for task in todo
    {
        println!("id: {}, name: {}, status: {}", task.id, task.task, task.done_status);
    }
}

fn add_new_task(todo : &mut Vec<Task>, str: &str)
{
    let mut rng = rand::thread_rng();
    let random_number:u64 = rng.gen();
    let task:Task = Task 
    {
        task: str.to_string(),
        done_status: false,
        id: random_number
    };
    todo.push(task);
    println!("New task has been added: {}", str);
}
