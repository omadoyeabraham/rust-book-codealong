#[derive(Debug)]
struct Task {
    id: String,
    title: String,
    description: String,
    completed: bool,
}

fn main() {
    let mut task_1 = Task {
        id: String::from("1"),
        description: String::from("Read to lead"),
        title: String::from("Man's search for meaning"),
        completed: true
    };

    task_1.title = String::from("Schrodinger's cat and other uncertainties");
    println!("{:?}", task_1);
}