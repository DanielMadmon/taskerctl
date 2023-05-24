use clap::{Parser,Subcommand,Args};
use tasker::taskerctl::{read_tasks_db,read_logs_db,rm_task,Task};

fn main(){
    
    let input = ArgsData::parse();
    match input.input{
        ArgsInput::Add(options) => {
            //TODO:Unwarp safely and add task
           let name = options.name;         
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
    #[arg(short='h',long="hour")]
    hour: Option<i32>,
    ///add minute of execution (optional)
    #[arg(short='n',long="minute")]
    minute: Option<i32>,
}
#[derive(Debug,PartialEq,Args,Clone)]
struct TaskName{
    name:String
}

fn show_list(){
    let task_db = read_tasks_db();

}
fn help(){

}
fn remove_task(task_name:String){
    rm_task(task_name);
    println!("task deleted");
}