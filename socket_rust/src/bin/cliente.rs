use std::net::TcpStream;
use std::io::{self, Read, Write};

fn main(){
    let direction: &str = "127.0.0.1:7086";
    println!("Intentando conectar al servidor en {}", direction);

    match TcpStream::connect(direction){
        Ok(mut stream) => {
            println!("Conexión establecida!");

            chat_con_servidor(&mut stream);
        }
        Err(e) => {
            println!("No se pudo establecer conexión: {}", e);
            println!("Está encendido el server??");
        }
    }
}

fn chat_con_servidor(stream: &mut TcpStream){
    loop {
        print!("Escribe tu mensaje: ");
        io::stdout().flush().expect("Error al forzar la pantalla");

        let mut msj = String::new();
        io::stdin().read_line(&mut msj).expect("Error leyendo el teclado");

        if msj.trim().is_empty(){
            continue; //evitar que el usuario envíe un msj vacío
        }

        if let Err(e) = stream.write_all(msj.as_bytes()){
            println!("Error enviando el mensaje: {}", e);
            break;
        }

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer){
            Ok(0) => {
                println!("El servidor cerró la conexión.");
                break;
            }
            Ok(bytes_leidos) => {
                let respuesta= String::from_utf8_lossy(&buffer[..bytes_leidos]) ;
                println!("El servidor respondió: {}", respuesta.trim());
            }
            Err(e) => {
                println!("Error esperando respuesta: {}", e);
                break;
            }
        }

        print!("Cerrar canal? (s/n): ");
        io::stdout().flush().expect("Error al forzar la pantalla");

        let mut salir = String::new();
        io::stdin().read_line(&mut salir).expect("Error al leer el teclado");

        if salir.trim().to_lowercase() == "s"{
            println!("Cerrando el cliente....");
            break;
        }
        println!("------------------------")
    }
}