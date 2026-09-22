mod transport;

use screenseal_core::protocol::message::Message;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use transport::tcp::{
    enviar_mensaje,
    iniciar_servidor,
    recibir_mensaje,
};

const VERSION_PROTOCOLO: u8 = 1;

fn servidor() {
    let listener =
        iniciar_servidor("127.0.0.1:7878")
            .expect("No se pudo iniciar el servidor");

    println!("Host escuchando en 127.0.0.1:7878");

    let (mut stream, direccion) =
        listener.accept()
            .expect("No se pudo aceptar la conexión");

    println!("Cliente conectado: {}", direccion);

    // 1. Enviar saludo al cliente
    let hello = Message::Hello {
        version: VERSION_PROTOCOLO,
    };

    enviar_mensaje(&mut stream, &hello)
        .expect("No se pudo enviar Hello");

    println!("Host -> Cliente: {:?}", hello);

    // 2. Esperar respuesta del cliente
    let respuesta =
        recibir_mensaje(&mut stream)
            .expect("No se pudo recibir HelloAck");

    println!("Cliente -> Host: {:?}", respuesta);

    // 3. Validar respuesta
    match respuesta {
        Message::HelloAck { version } if version == VERSION_PROTOCOLO => {
            println!("Handshake aceptado");
        }

        Message::HelloAck { version } => {
            println!(
                "Versión incompatible. Host: {}, Cliente: {}",
                VERSION_PROTOCOLO,
                version
            );
        }

        otro_mensaje => {
            println!(
                "Se esperaba HelloAck, pero llegó: {:?}",
                otro_mensaje
            );
        }
    }
}

fn cliente() {
    thread::sleep(Duration::from_millis(500));

    let mut stream =
        TcpStream::connect("127.0.0.1:7878")
            .expect("No se pudo conectar al host");

    println!("Cliente conectado al host");

    // 1. Recibir saludo del host
    let mensaje =
        recibir_mensaje(&mut stream)
            .expect("No se pudo recibir Hello");

    println!("Cliente recibió: {:?}", mensaje);

    // 2. Revisar la versión recibida
    match mensaje {
        Message::Hello { version } => {
            if version == VERSION_PROTOCOLO {
                let respuesta = Message::HelloAck {
                    version: VERSION_PROTOCOLO,
                };

                enviar_mensaje(&mut stream, &respuesta)
                    .expect("No se pudo enviar HelloAck");

                println!("Cliente -> Host: {:?}", respuesta);
                println!("Handshake del cliente aceptado");
            } else {
                println!(
                    "Versión incompatible. Host: {}, Cliente: {}",
                    version,
                    VERSION_PROTOCOLO
                );
            }
        }

        otro_mensaje => {
            println!(
                "Se esperaba Hello, pero llegó: {:?}",
                otro_mensaje
            );
        }
    }
}

fn main() {
    let servidor_hilo = thread::spawn(servidor);
    let cliente_hilo = thread::spawn(cliente);

    servidor_hilo.join().unwrap();
    cliente_hilo.join().unwrap();

    println!("Comunicación finalizada correctamente");
}