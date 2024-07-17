// fn main() -> io::Result<()> {
//     let server_address = "127.0.0.1:7878";
//     let mut stream = TcpStream::connect(server_address)?;
//
//     let command = Command::CreateAccount("Alice".to_string());
//
//     // Сериализация операции в JSON
//     let serialized = serde_json::to_string(&command).unwrap();
//
//     // Отправка сериализованных данных на сервер
//     stream.write_all(serialized.as_bytes())?;
//     println!("{}", serialized);
//
//     // Чтение ответа от сервера
//     let mut buffer = [0; 512];
//     let n = stream.read(&mut buffer).unwrap();
//
//     // Десериализация полученных данных
//     let received_data = &buffer[..n];
//     let response: Response = serde_json::from_slice(received_data).unwrap();
//
//     println!("{:?}", response);
//     Ok(())
// }
