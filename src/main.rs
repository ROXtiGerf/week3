use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::io::{self, Read, Write};

//处理流
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0;512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0{
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    //设置一个listener用于接收ip中发出的信息
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //创建一个装线程的容器
    let mut thread_vec: Vec<thread::JoinHandle<()>> =Vec::new();
    //用for循环持续监听流里的内容
    for stream in listener.incoming() {
        //如果流有问题则返回failed
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        thread_vec.push(handle);
    }

    //处理线程，等待结束
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}
