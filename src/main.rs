use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task: String,
    status: bool,
}

fn main(){
    let filename = &get_path();
    let mut task = load_tasks(filename);

    println!("Available Functions: add / add task, remove / delete, done / complete, quit / close, list");
    println!("\n--------------------------------------------------------------");
    for (index, entry) in task.iter().enumerate() {
        println!("{}) task: {:?}, complete: {:?}", index+1, entry.task, entry.status)
    }
    println!("--------------------------------------------------------------\n");

    loop {
        print!("Enter a query: ");
        io::stdout().flush().unwrap();

        let mut userinput = String::new();

        io::stdin()
            .read_line(&mut userinput)
            .expect("Unable to process your request.");

        let userinput = userinput.trim().to_string();

        if userinput.to_lowercase() == "add task" || userinput.to_lowercase() == "add" {

            add_task(&mut task);
            save_tasks(filename, &mut task);
            println!("");

        } else if userinput.to_lowercase() == "quit" || userinput.to_lowercase() == "cancel" || userinput.to_lowercase() == "clear" || userinput.to_lowercase() == "close"{
            break;
        } else if userinput.to_lowercase() == "list" {

            println!("\n**************************************************************");
            for (index, entry) in task.iter().enumerate() {
            println!("{}) task: {:?}, complete: {:?}", index+1, entry.task, entry.status)
            }
            println!("**************************************************************\n");

        } else if userinput.to_lowercase() == "remove" || userinput.to_lowercase() == "delete"{

            remove_task(&mut task);
            save_tasks(filename, &mut task);
            println!("");

        } else if userinput.to_lowercase() == "mark done" || userinput.to_lowercase() == "done" || userinput.to_lowercase() == "complete"{

            mark_done(&mut task);
            save_tasks(filename,&mut task);
            println!("");

        }
    }
}

fn load_tasks(filename: &Path) -> Vec<Task>{
    if Path::new(filename).exists() {
       let data = fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
       serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_tasks(filename: &Path, tasks: &mut Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Unable to save the changes");
    fs::write(filename, data).expect("Unable to save the changes.");
}
    

fn add_task(task_list: &mut Vec<Task>) {
    print!("\nAdd Task: ");
    io::stdout().flush().unwrap();

    let mut task = String::new();      
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line please try again later!");
    let task = task.trim().to_string();
    
    task_list.push(
        Task {
            task: format!("{}", task),
            status: false 
        } 
    );
}

fn remove_task(tasks: &mut Vec<Task>){
    print!("Enter the index of task you wish to remove: ");
    io::stdout().flush().unwrap();

    let mut del_index = String::new();
    io::stdin()
        .read_line(&mut del_index)
        .expect("Unable to read your input.");

    match del_index.trim().parse::<usize>() {
        Ok(i) if i > 0 && i <= tasks.len() => {
            tasks.remove(i-1);
        }
        Ok(i) if i > tasks.len() => {
            println!("The item at index {} does not exist.", i);
        }
        _ => println!("Please enter a valid index.")
    };
}

fn mark_done(tasks: &mut Vec<Task>) {
    print!("Enter the index of task you wish to mark as complete: ");
    io::stdout().flush().unwrap();

    let mut q_index = String::new();
    io::stdin()
        .read_line(&mut q_index)
        .expect("Unable to read your input.");

    match  q_index.trim().parse::<usize>() {
        Ok(i) if i > 0 && i <= tasks.len() => {
            tasks[i-1].status = true
        }
        Ok(i) if i > tasks.len() => {
            println!("The item at index {} does not exist.", i);
        }
        _ => println!("Please enter a valid index."), 
    };
}

fn get_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");

    let mut path = PathBuf::from(home_dir);
    path.push(".local/share/todolist");

    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create data directory");
    }

    path.push("list.json");
    path
}
