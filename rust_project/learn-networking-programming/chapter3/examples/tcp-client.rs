use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    // 使用TcpStream::connect建立到服务器的连接，该连接接受远程端点地址为字符串，
    // 同服务端一样，也是需要IP和端口
    // 如果连接失败，将使用设置的错误消息终止程序。

    let mut stream = TcpStream::connect("0.0.0.0:8888").expect("Could not connect to server");

    // 在一个无限循环中，初始化一个空字符串以本地读入用户输入，并初始化一个Vec<u8>以从服务器读取响应 。
    // readline函数从标准输入中读取一行并将其存储在称为input的变量中。
    // 然后以字节流的形式写入连接，服务端相应，客户端使用BufReader读取内容，
    // 使用BufReader可以减少更多的系统调用
    // read_until 方法读取缓冲区中的数据， 缓冲区根据需要增长，
    // 最后打印出字符串的内容，使用std::str::from_utf8将字节序列转换成字符串，打印
    loop {
        let mut input = String::new();

        let mut buffer: Vec<u8> = Vec::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        stream
            .write(input.as_bytes())
            .expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");

        println!(
            "{}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        );
    }
}
