/**
@author 郑春宏 - 906 - Team4
@date 2022-04-10
@description 程序设计参考文档：https://doc.rust-lang.org/1.0.0/std/net/index.html 实现功能如下：
1、程序通过 `cargo run` 编译并启动后，Mac 机上通过 `lsof -i:8866` 可以看到 `TCP localhost:8866 (LISTEN)` 表示监听启动成功。
2、客户端通过 telnet 进行测试，测试命令 : `telnet localhost 8866` 输入 ASCCII 字符返回响应字符测试通过。
3、为了可以连接多个客户端，需要使用 thread 类库对链接使用线程进行处理。
4、输入`bye`程序退出，并提示 `Bye bye and see you soon.`
*/
// 引用类库 io、net 用来完成TCP监听读取，
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
// 引入 thread 类库用来多线程处理
use std::thread;
// 引入 str 库，用来转换输入的 buf 到 str 类型。
use std::str;
// 程序入口函数
fn main() {
    println!("Hello, world!");
    // 定义一个请求地址 IP:端口 的形式
    let addr = "127.0.0.1:8866".to_string();
    // 创建一个Tcp监听，通过字符串切片将addr 传入，代码执行后（MacOS）可以通过 lsof -i:8866 进行验证端口启动

    let listener = TcpListener::bind(&addr).unwrap();
    // 调用 incoming() 方法接收客户端的链接信息，如果有新的信息进来就会返回一个Result枚举，OK(T:TcpStream)
    for stream in listener.incoming() {
        // 如果有客户端链接比如通过： telnet 127.0.0.1 8866
        println!("DEBUG::有新的链接进入，这行字就会打印 -------");
        // 模式匹配
        match stream {
            // 当Result 枚举类型匹配Ok时
            Ok(stream) => {
                // 如果链接成功，开启一个新的线程，之所以用多线程的原因是TCP客户请求可能有多个。
                thread::spawn(move|| {
                    // 将客户端处理信息解耦到 handle_client 函数中，并移交 stream 变量所有权
                    handle_client(stream);
                });
            }
            // 当Result 3枚举匹配错误时
            Err(e) => {
                // 直接通过panic!宏输出错误信息，并终止程序运行。
                panic!("妈蛋！遇到错误 {:?}", e)
            }
        }
    }
    // 关闭Tcp监听链接
    drop(listener);
}
// 线程调用的处理函数。
/**
* @param stream: TcpStream  传入的输入流
*/

fn handle_client(mut stream: TcpStream){

    println!("DEBUG::客户链接处理中。");
    // 定义一个存储用的数组，因为需要后续进行填充值所以声明为可变的 `mut`
    let mut buf = [0; 512];
    // 建立一个循环，来反复读取客户的输入信息
    loop {
        // 通过read方法
        let bytes_read = stream.read(&mut buf).expect("读取出现错误，这里直接中断程序运行");
        // 输出调试信息
        println!("DEBUG::byte size: {}", bytes_read);
        // 如果输入流的字符长度是直接退出循环。
        if bytes_read == 0 {
            // 退出loop，实际上这里退出后整个方法也就退出了。
            break;
        }

        // 为了后面对比方便，将byte[] 转换为str 类型。
        let s = match str::from_utf8(&buf[..bytes_read]) {
            // 如果转换成功返回字符串值。
            Ok(v) => v,
            // 遇到转换错误输出错误信息，终止程序运行。
            Err(e) => {
                // 输出调试信息。
                stream.write(b"Need utf-8 sequence.").unwrap();
                // 继续监听，虽然本次输入的字符流格式不是utf8 字符，但是不影响下次输入所以不需要 panic!
                continue;
            },
        };
        // 如果输入的前3个字符串是 bye 则程序终止，为了防止越界所以需要先判断 s.len() >= 3
        if s.len() >= 3 && s[0..3] == "bye".to_string() {
            // 输出终止前的消息。
            stream.write(b"Bye bye and see you soon.\n").unwrap();
            // 直接跳出 loop 循环，实际上这个链接也就终止了
            break;
        }
        // 如果程序没有终止，返回输入的消息，也就是输入什么返回什么，unwrap() 表示不处理错误，遇到错误直接出错退出程序。
        stream.write(&buf[..bytes_read]).unwrap();
    }
}



