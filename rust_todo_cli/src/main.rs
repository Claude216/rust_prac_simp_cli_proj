use std::io;

// this is the struct to store hte task content and the task id
struct Task {
    id: usize,
    content: String,
}



fn main() {
    println!("Hello, world!");
}

// add one task to the task list
fn add_task(tasks: &mut Vec<Task>, content: String) {
    let id = tasks.len() + 1; // get the current task id by increase the current max id by 1
    tasks.push(Task {id, content}); // push the new task to the task list
}

// list all the tasks in the task list (by printing on the console)
fn list_tasks(tasks: &mut Vec<Task>) {
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


