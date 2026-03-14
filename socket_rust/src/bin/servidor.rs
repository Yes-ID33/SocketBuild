use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let direction: &str = "127.0.0.1:7086";

    let listener = TcpListener::bind(direction).expect("El puerto 7086 está ocupado :(");

    println!("El servidor RUST está escuchando en {}...", direction);
    println!("Esperando al cliente...");

    match listener.accept() {
        Ok((mut stream, addr)) => {
            println!("Conexión exitosa, el cliente viene de {}", addr);

            let mut buffer = [0; 1024];

            loop {
                match stream.read(&mut buffer){
                    Ok(0)=>{
                        println!("El cliente se desconectó manualmente.");
                        break;
                    }
                    Ok(bytes_leidos) => {
                        let mensaje = String::from_utf8_lossy(&buffer[..bytes_leidos]);
                        let msj_limpio = mensaje.trim();

                        println!("Cliente dice: {}", msj_limpio);

                        let longitud = msj_limpio.len();
                        let respuesta = if longitud > 15{
                            format!("Que mensaje tan grande: '{}'\n", msj_limpio)
                        }else if longitud >8{
                            format!("Mensaje de tamaño promedio: '{}'\n", msj_limpio)
                        }else{
                            format!("Que mensaje tan pequeño: '{}'\n", msj_limpio)
                        };
                        if let Err(e) = stream.write_all(respuesta.as_bytes()) {
                            println!("Error respondiéndole al cliente: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("No se pudo leer el mensaje del cliente: {}", e);
                        break;
                    }
                }

            }
        }
        Err(e) => {
            println!("Alguien intentó conectarse pero falló: {}", e);
        }
    }

}