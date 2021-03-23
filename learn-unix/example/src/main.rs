use structopt::StructOpt;
use ls::display_file;

#[derive(Debug, StructOpt, Clone)]
pub struct Ls {
    /// 路径参数
    pub path: Option<String>,
}

impl Ls {
    pub fn get_path(&self) -> Option<&str>{
        self.path.as_deref()
    }
}




#[derive(Debug, StructOpt, Clone)]
pub enum Command {
    /// 显示当前的文件，绝对路径或者相对路径
    #[structopt(name = "ls")]
    Ls(Ls),
}

#[derive(Debug, StructOpt, Clone)]
/// 应用程序命令行解析
pub struct ApplicationArguments  {
    #[structopt(subcommand)]
    pub command: Command,
}

impl ApplicationArguments {
    fn run(&self) {
        match self.command.clone() {
            Command::Ls(ls) => {
                let path = ls.get_path().unwrap_or("");
                let path = String::from(path);
                display_file(path);
            }
            // _ => println!("error args"),
        }
    }
}


fn main() {
    let opt : ApplicationArguments = ApplicationArguments::from_args();
    println!("{:?}", opt);
    opt.run();
}
