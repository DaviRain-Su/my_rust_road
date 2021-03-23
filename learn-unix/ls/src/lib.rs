use std::fs::read_dir;
use std::io;

pub fn display_file(path : String) -> io::Result<()> {
    // unsafe  {
        // let path_ptr = path.as_ptr() as *const c_char;
        // let dp = opendir(path_ptr);
        // if dp.is_null() {
        //     panic!("can't open file, {}", path);
        // }

        // loop {
        //     let dirp = readdir(dp);
        //     if dirp.is_null() {
        //         break;
        //     }
        //     let file_name = (*dirp).d_name;
        //
        //     println!("{:?}", (*dirp).d_name);
        // }
        // closedir(dp);
        for entry in read_dir(path)? {
            let p = entry?;
            println!("{:?}", p.path());
        }

        Ok(())
    // }
}

#[test]
fn test() {
    let a = [1, 2, 3];
    let mut it = a.iter();
    println!("{:?}", it.next());
    let mut it1 = it.clone();
    println!("{:?}", it1.next());
}