use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    env, fs::{self, OpenOptions}, io::{Read, Write}, path::PathBuf
};

fn get_data_file_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not find $HOME environment variable");

    let config_dir = PathBuf::from(home_dir).join(".config/project-tracker");

    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    config_dir.join("data.json")
}

#[derive(Parser)]
#[command(name = "Project Tracker")]
#[command(about = "A simple CLI tool to keep track of your projects")]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new project.
    AddProject {
        /// Name of the project.
        name: String,
    },
    /// List all projects.
    ListProjects,
    /// Add a task to a project.
    AddTask {
        project: String,
        description: String,
    },
    /// List all tasks in a project.
    ListTasks { project: String },
    /// Mark a task as complete
    CompleteTask { project: String, task_id: u32 },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    tasks: Vec<Task>,
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::AddProject { name }) => add_project(&name),
        Some(Commands::ListProjects) => list_projects(),
        Some(Commands::AddTask {
            project,
            description,
        }) => add_task(&project, &description),
        Some(Commands::ListTasks { project }) => list_tasks(&project),
        Some(Commands::CompleteTask { project, task_id }) => complete_task(project, *task_id),
        None => list_all_projects_and_tasks(),
    }
}

fn add_project(name: &str) {
    let mut data = load_data();

    if data.iter().any(|p| p.name == name) {
        println!("Project with name '{}' already exists.", name);
        return;
    }

    let project = Project {
        name: name.to_string(),
        tasks: Vec::new(),
    };

    data.push(project);
    save_data(&data);

    println!("Project '{}' added", name);
}

fn load_data() -> Vec<Project> {
    let data_file = get_data_file_path();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&data_file)
        .unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read data file.");

    if content.is_empty() {
        return Vec::new();
    } else {
        serde_json::from_str(&content).expect("Unable to parse data file.")
    }
}

fn save_data(data: &Vec<Project>) {
    let data_file = get_data_file_path();

    let content = serde_json::to_string_pretty(data).expect("Unable to serialize data.");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&data_file)
        .expect("Unable to open data file.");

    file.write_all(content.as_bytes())
        .expect("Unable to write data file.");
}

fn list_projects() {
    let data = load_data();

    if data.is_empty() {
        println!("{}", "No projects found");
    } else {
        println!("{}", "Projects:");
        for project in data {
            println!(" - {}", project.name);
        }
    }
}

fn add_task(project_name: &str, description: &str) {
    let mut data = load_data();

    if let Some(project) = data.iter_mut().find(|p| p.name == project_name) {
        let new_id = project.tasks.last().map_or(1, |t| t.id + 1);
        let task = Task {
            id: new_id,
            description: description.to_string(),
            completed: false,
        };

        project.tasks.push(task);
        save_data(&data);
        println!("Task {} added to project: '{}'.", description, project_name);
    } else {
        println!("Project '{}' not found.", project_name);
    }
}

fn list_tasks(project_name: &str) {
    let data = load_data();

    if let Some(project) = data.iter().find(|p| p.name == project_name) {
        println!("Tasks in project: {}:", project_name);

        if project.tasks.is_empty() {
            println!("    {}", "No tasks yet")
        } else {
            for task in &project.tasks {
                let checkbox = if task.completed { "[x]" } else { "[ ]" };
                println!("    {} {}: {}", checkbox, task.id, task.description);
            }
        }
    } else {
        println!("Project '{}' not found.", project_name);
    }
}

fn complete_task(project_name: &str, task_id: u32) {
    let mut data = load_data();
    if let Some(project) = data.iter_mut().find(|p| p.name == project_name) {
        if let Some(task) = project.tasks.iter_mut().find(|t| t.id == task_id) {
            if task.completed {
                println!("Task {} is already completed!", task_id);
                return;
            }
            task.completed = true;
            save_data(&data);
            println!(
                "Task {} in project '{}' is now completed!",
                task_id, project_name
            );
        } else {
            println!("Task {} not found in project '{}'.", task_id, project_name);
        }
    } else {
        println!("Project '{}' not found.", project_name);
    }
}

fn list_all_projects_and_tasks() {
    let data = load_data();
    if data.is_empty() {
        println!("No projects found.");
        return;
    }

    println!("Projects:");

    for project in data {
        println!("Project: \"{}\"", project.name);

        // Calculate concluded %.
        let total_tasks = project.tasks.len();
        let completed_tasks = project.tasks.iter().filter(|t| t.completed).count();
        let progress = if total_tasks > 0 {
            completed_tasks as f64 / total_tasks as f64
        } else {
            0.0
        };

        // Build progress bar.
        let bar_width = 20;
        let filled = (progress * bar_width as f64).round() as usize;
        let filled_bar = "█".repeat(filled).green();
        let empty_bar = " ".repeat(bar_width - filled);
        let percentage = (progress * 100.0) as u8;
        let progress_bar = format!(
            "[{}{}] {}%",
            filled_bar,
            empty_bar,
            percentage.to_string().bold().yellow()
        );

        println!("Progress: {}", progress_bar);

        if project.tasks.is_empty() {
            println!("    No tasks yet.");
        } else {
            for task in &project.tasks {
                let checkbox = if task.completed {
                    "[x]".green()
                } else {
                    "[ ]".red()
                };
                println!("    {} {}: {}", checkbox, task.id, task.description);
            }
        }
        println!();
    }
}
