use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
//处理端
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    //定义变量	
    let mut buf = [0; 512];
    //循环字节流	
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
     
        stream.write(&buf[..bytes_read])?;
	//休眠    
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}
//主函数
fn main() -> std::io::Result<()> {
    //绑定IP地址	
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    //循环输入流
    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        let handle = thread::spawn(move || {
            handle_client(stream)
		.unwrap_or_else(|error| eprintln!("{:?}", error));
        });
      
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
