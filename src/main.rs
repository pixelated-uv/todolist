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
    println!("\nAvailable Functions: new / add, remove / delete, done / complete, quit / close, list / ls");
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
        let mut userinput = userinput.split_once(" ").unwrap_or_else(|| (&userinput, ""));


        if (userinput.0).to_lowercase() == "new" || (userinput.0).to_lowercase() == "add" {

            add_test(&mut task, &mut (userinput.1).to_string());
            save_tasks(filename, &mut task);
            println!("");

        } else if (userinput.0).to_lowercase() == "quit" || (userinput.0).to_lowercase() == "cancel" || (userinput.0).to_lowercase() == "clear" || (userinput.0).to_lowercase() == "close"{
            break;
        } else if (userinput.0).to_lowercase() == "list" || (userinput.0).to_lowercase() == "ls"{

            println!("\n--------------------------------------------------------------");
            for (index, entry) in task.iter().enumerate() {
            println!("{}) task: {:?}, complete: {:?}", index+1, entry.task, entry.status)
            }
            println!("--------------------------------------------------------------\n");

        } else if (userinput.0).to_lowercase() == "remove" || (userinput.0).to_lowercase() == "delete" || (userinput.0).to_lowercase() == "rm"{

            remove_task(&mut task, &mut userinput);
            save_tasks(filename, &mut task);
            println!("");

        } else if (userinput.0).to_lowercase() == "done" || (userinput.0).to_lowercase() == "complete"{

            mark_done(&mut task, &mut userinput);
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
    
fn remove_task(task_list: &mut Vec<Task>, args: &mut (&str, &str)){

    if (args.1) != "" {
        if let Ok(index) = (args.1).parse::<usize>() { 
            if index > 0 && index <= task_list.len() {
                task_list.remove(index-1);
            } else {
                println!("Please enter a valid index.");
            }
        } else {
            println!("Please enter a valid index.");
        }
    } else if (args.1) == "" {
        print!("Enter the index of task you wish to remove: ");
        io::stdout().flush().unwrap();

        let mut del_index = String::new();
        io::stdin()
            .read_line(&mut del_index)
            .expect("Unable to read your input.");

        match del_index.trim().parse::<usize>() {
            Ok(i) if i > 0 && i <= task_list.len() => {
                task_list.remove(i-1);
            }
            Ok(i) if i > task_list.len() => {
                println!("The item at index {} does not exist.", i);
            }
            _ => println!("Please enter a valid index.")
        };
    }
    else {
        println!("Please enter a valid index.")
    }
}

fn mark_done(task_list: &mut Vec<Task>, args: &mut (&str, &str)) {
    if (args.1) != "" {
        if let Ok(index) = args.1.parse::<usize>() {
            if index > 0 && index <= task_list.len() {
                task_list[index-1].status = true;
            } else {
                println!("Please enter a valid index.");
            }
        } else {
            println!("Please entere a valid index.");
        }

     } else if (args.1) == "" {
        print!("Enter the index of task you wish to mark as complete: ");
        io::stdout().flush().unwrap();

        let mut q_index = String::new();
        io::stdin()
            .read_line(&mut q_index)
            .expect("Unable to read your input.");

        match  q_index.trim().parse::<usize>() {
            Ok(i) if i > 0 && i <= task_list.len() => {
                task_list[i-1].status = true
            }
            Ok(i) if i > task_list.len() => {
                println!("The item at index {} does not exist.", i);
            }
            _ => println!("Please enter a valid index."), 
        };
    } else {
        println!("Please enter a valid index.");
    }
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

fn add_test(task_list: &mut Vec<Task>, task: &mut String) {
    if task != "" {
        task_list.push(
            Task {
                task: task.clone(),
                status: false,
            }
        );
    } else {
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
}   
