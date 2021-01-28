use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::{Error, Read, Write};

// Handle a single client
// 这里必须使用一个无限循环从每个客户机读取数据，但是在主线程中运行一个无线循环会阻塞它，
// 并且没有其他客户端能够连接。这种行为是不可取的。因为这里派生出一个辅助线程来处理每个
// 客户端的连接 
// 
// 读取每个流并将其写会的逻辑封装在名为handleclient的函数中，每个线程接受一个闭包来调用这个函数
// 这个闭包必须是一个move 闭包， 因为它必须从封闭范围中读取一个变量流。
// 在该函数中，我们打印远程端点地址和端口，然后定义一个缓冲区来临时保存数据。
// 还要确保将缓冲区归零。然后我们运行一个无限循环，在其中读取流中的所有数据。
// 流中的read方法返回它已读的数据的长度。 
// 有两种情况返回零，如果已经到达流的末尾，或者如果给定的缓冲区长度为零。
// 第二种情况是不对的，所以当read方法返回零时，我们中断循环，返回一个OK(())
// 使用write方法写会数据
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incomming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        // 使用?操作符来处理操作中的错误，如果一样正常，该操作符将结果返回在OK中，
        // 否则将错误提前返回给调用函数，
        // 在这种情况下，函数的返回类型必须是处理成功返回的空类型，或者处理错误返回的io::Error类型
        
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    // 创建了一个TcpListener， 在Rust中，表示一个正在侦听来自客户端的连接的Tcp套接字，
    // 这里硬编码的了本地地址和端口
    // 将本地地址设置为0.0.0.0 告诉内核将此套接字绑定主机上的所有可用的接口
    // 调用本地的IP和端口用bind方法来创建本地监听套接字
    // 这里选择的是将Ip把这个套接字绑定到主机上的所有可用接口
    // 任何能够到达连接到这个主机网络的客户端都能与这个主机通信。
    // 如果没有错误将返回TcpListener,如果发生错误会使用给定的信息产生恐慌
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");

    // TcpListener的incoming 方法返回一个遍历已连接到服务器的流的迭代器，
    // 循环遍历他们并检查是否其中任何一个遇到的错误， 
    // 出错时，打印出错信息，并移动到下一个连接的客户端。
    // 这里恐慌是不合适的， 因为如果某些客户端由于某些原因出现错误，服务器可以正常运行。
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("failed: {}", e);
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
