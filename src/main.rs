use argh::FromArgs;
fn main(){
    let input:TopLevel = argh::from_env();
}
#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
struct TopLevel {
    #[argh(subcommand)]
    add: Add,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Add {
    Name(NameStruct),
    Shell(ShellStruct),
}
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "Name")]
#[argh(description = "add new task name")]
struct NameStruct{
    #[argh(option)]
    ///name of the new task
    name:String
}   

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "add shell name for new task")]
#[argh(subcommand, name = "Shell")]
struct ShellStruct{
    #[argh(option)]
    ///name of the shell to run the command on 
    shell:String
}   