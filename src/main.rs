use tokio::net::TcpListener;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt, AsyncReadExt};

#[allow(unused)]

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let message_len = reader.read_i32();
            let mut username = String::new();
            reader.read_line(&mut username);
            let message_type = reader.read_i32();
            let mut message = String::new();
            
            loop {
                let read_byte_size = reader.read_line(&mut message).await.unwrap();
                if read_byte_size == 0 {
                    break;
                }

            }

            writer.write_all(message.as_bytes()).await.unwrap();
        });
    }
}
