use serde::{Serialize, Deserialize};
use clap::{App, Arg, SubCommand};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    content: String
}

fn todo_path() -> std::path::PathBuf {
    dirs::home_dir().unwrap().join(".pico-todo")
}

fn load_todos() -> Result<Vec<Todo>, std::io::Error> {
    let data = std::fs::read_to_string(&todo_path())?;
    let todos: Vec<Todo> = serde_json::from_str(data.as_str()).unwrap();
    Ok(todos)
}

fn save_todos(todos: &Vec<Todo>) {
    std::fs::write(&todo_path(), serde_json::to_string(todos).unwrap()).unwrap();
}

fn main() {
    let mut todos = load_todos().unwrap_or(Vec::<Todo>::new());
    let matches = 
        App::new("Pico Todo")
            .version("1.0")
            .author("Nathanial Hartman")
            .about("Track Todos, in 50 Lines or Less of Code")
            .subcommand(SubCommand::with_name("add")
                        .arg(Arg::with_name("content").required(true).takes_value(true)))
            .subcommand(SubCommand::with_name("remove")
                        .arg(Arg::with_name("index").required(true).takes_value(true)))
            .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        todos.push(Todo {
            content:matches.value_of("content").unwrap().to_string()
        });
        save_todos(&todos);
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let index:usize = matches.value_of("index").unwrap().parse().unwrap();
        todos.remove(index - 1);
        save_todos(&todos);
    }
    let mut index = 0;
    for todo in todos.iter() {
        println!("{}: {}", index + 1, todo.content);
        index += 1;
    }
}