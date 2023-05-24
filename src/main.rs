use clap::{Parser,Subcommand,Args};
use tasker::taskerctl::{read_tasks_db,read_logs_db,rm_task,Task,add_task};

fn main(){
    
    let input = ArgsData::parse();
    let mut new_task = Task::new();
    match input.input{
        ArgsInput::Add(options) => {
         if let Some(name) = options.name{
            new_task.name = Some(name);
         }
         if let Some(shell) = options.shell{
            new_task.shell = Some(shell);
         }
         if let Some(command) = options.command{
            new_task.command = Some(command);
         }
         new_task.comment = options.comment;
         new_task.month = options.month;
         new_task.day_of_month = options.month;
         new_task.day_of_week = options.day_of_week;
         new_task.hour = options.hour;
         new_task.minute = options.minute;
         add_task(new_task);
         println!("new task added successfully.");
        }
        ArgsInput::Status => {
            help();
        }
        ArgsInput::List => {
            show_list();
        }
        ArgsInput::Remove(task_name) =>{
            remove_task(task_name.name);
        }
        ArgsInput::Logs =>{
            
        }
    }
}
    
#[derive(Debug,Parser,PartialEq)]
#[clap(author,version,about)]
#[command(author, version, about, long_about = None)]
struct ArgsData{
    #[clap(subcommand)]
    input:ArgsInput
}

#[derive(Debug, Subcommand,PartialEq,Clone)]
#[command(author, version, about, long_about = None)]
enum ArgsInput{
    ///show tasker-service status
    Status,
    ///show all the tasks
    List,
    ///Add new task
    Add(AddOptions),
    ///remove task by passing task's name
    Remove(TaskName),
    ///show logs
    Logs
}

#[derive(Args,Debug,PartialEq,Clone)]
struct  AddOptions{
    ///add task name (required)
    #[clap(short = 'n',long = "name")]
    name:Option<String>,
    ///add shell to run the command with (required)
    #[clap(short = 's', long = "shell")]
    shell:Option<String>,
    ///add task command (required)
    #[arg(short='c',long="command")]
    command: Option<String>,
    ///add comment
    #[arg(long = "comment")]
    comment: Option<String>,
    ///add month of execution (optional)
    #[arg(short='m',long="month")]
    month: Option<i32>,
    ///add day of month for execution (optional)
    #[arg(short='d',long="day_month")]
    day_of_month: Option<i32>,
    ///add day of week for execution (optional)
    #[arg(short='w',long="day_week")]
    day_of_week: Option<i32>,
    ///add hour of execution (optional)
    #[arg(short='u',long="hour")]
    hour: Option<i32>,
    ///add minute of execution (optional)
    #[arg(short='t',long="minute")]
    minute: Option<i32>,
}
#[derive(Debug,PartialEq,Args,Clone)]
struct TaskName{
    name:String
}

fn show_list(){
    let task_db = read_tasks_db();
    for task in task_db{
        println!("{task:?}");
    }
}
fn help(){

}
fn remove_task(task_name:String){
    rm_task(task_name);
    println!("task deleted");
}