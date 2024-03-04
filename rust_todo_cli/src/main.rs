use std::io;

// this is the struct to store hte task content and the task id
struct Task {
    id: usize,
    content: String,
}



fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("1: Add A Task");
        println!("2: List Tasks");
        println!("3: Remove A Task");
        println!("Q: Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter the task content:");
                let mut task_content = String::new();
                io::stdin().read_line(&mut task_content).expect("Failed to read line");
                add_task(&mut tasks, task_content.trim().to_string());
            },
            "2" => {
                list_tasks(& tasks)
            },
            "3" => {
                println!("Enter the task id you want to remove:");
                let mut id_to_remove = String::new();
                io::stdin().read_line(&mut id_to_remove).expect("Failed to read line");
                let id: usize = id_to_remove.trim().parse().expect("Please type a number!");

                if remove_task(&mut tasks, id) {
                    println!("Task {} removed successfully.", id);
                } else {
                    println!("No task found with ID {}.", id);
                }
            },
            "Q" => {
                break;
            },
            _ => println!("Invalid option, please  try again."),
        }
    }

}

// add one task to the task list
fn add_task(tasks: &mut Vec<Task>, content: String) {
    let id = tasks.len() + 1; // get the current task id by increase the current max id by 1
    tasks.push(Task {id, content}); // push the new task to the task list
}

// list all the tasks in the task list (by printing on the console)
fn list_tasks(tasks: & Vec<Task>) {
    for task in tasks {
        println!("{}: {}", task.id, task.content);
    }
}

// remove a task with a given id
fn remove_task(tasks: &mut Vec<Task>, id: usize) -> bool {

    if let Some(index) // if there is a match, index will be the first element in the 
                       // task list that matches the condition; otherwise, it returns 'None'
    = 
    tasks.iter() // create a new iterator over vector 'tasks'
    .position // called on the iterator, returns 'true' or 'false depending on whether the 
              // element satisfies the condition
    (|task| // take a reference   
         task.id == id) {
        tasks.remove(index);
        true
    } else {
        false
    }
}


