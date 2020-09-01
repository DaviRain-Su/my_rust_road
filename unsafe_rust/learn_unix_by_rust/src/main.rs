use nix::unistd;
use nix::sys::stat;
// use tempfile::tempdir;
// use libc;

fn main() {
    println!("pid = {}", unistd::getpid());
    println!("uid = {}", unistd::getuid());
    println!("gid = {}", unistd::getgid());
    // println!("tid = {}", unistd::gettid());
    // println!("sid = {}", unistd::getsid());
    println!("grp = {}", unistd::getpgrp());

    let dir = unistd::getcwd().unwrap();
    println!("The current directory is {:?}", dir);

    let temp_dir = dir.join("hello_world");
    match unistd::mkdir(&temp_dir, stat::Mode::S_IRWXU) {
        Ok(_) => println!("create {:?}", temp_dir),
        Err(err) => println!("Error creating directory: {}", err),
    }

    // let tmp_dir1 = tempdir().unwrap();
    // let tmp_dir2 = tmp_dir1.path().join("new_dir");

    // create new directory and give read, write and execute rights to the owner
    // match unistd::mkdir(&tmp_dir2, stat::Mode::S_IRWXU) {
    //    Ok(_) => println!("created {:?}", tmp_dir2),
    //    Err(err) => println!("Error creating directory: {}", err),
    // }
    // let sleep_number = 10u32;
    // let ret = unistd::sleep(sleep_number);
    // println!("after is ....., ret = {}", ret);
    
    // let mut buf = [0u8; 45];
    // let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    // let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");
    // println!("Hostname: {}", hostname);

    // use nix::unistd::{fork, ForkResult};

    // match fork() {
    //     Ok(ForkResult::Parent { child, .. }) => {
    //         let mut buf = [0u8; 45];
    //         let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    //         let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");
    //         println!("parent: Hostname: {}", hostname);
    //         println!("Continuing execution in parent process, new child has pid: {}", child);
    //     }
    //     Ok(ForkResult::Child) => {
    //         println!("I'm a new child process");
    //         let mut buf = [0u8; 45];
    //         let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    //         let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");
    //         println!("Child Hostname: {}", hostname);
    //     },
    //     Err(_) => println!("Fork failed"),
    // }
}
