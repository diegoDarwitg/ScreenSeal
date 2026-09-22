use screenseal_core::protocol::message::Message;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn enviar_mensaje(
    stream: &mut TcpStream,
    mensaje: &Message,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = mensaje.encode()?;

    let tamaño = bytes.len() as u32;

    stream.write_all(&tamaño.to_be_bytes())?;
    stream.write_all(&bytes)?;

    Ok(())
}

pub fn recibir_mensaje(
    stream: &mut TcpStream,
) -> Result<Message, Box<dyn std::error::Error>> {
    let mut buffer_tamaño = [0u8; 4];

    stream.read_exact(&mut buffer_tamaño)?;

    let tamaño = u32::from_be_bytes(buffer_tamaño) as usize;

    let mut buffer_mensaje = vec![0u8; tamaño];

    stream.read_exact(&mut buffer_mensaje)?;

    let mensaje = Message::decode(&buffer_mensaje)?;

    Ok(mensaje)
}

pub fn iniciar_servidor(
    direccion: &str,
) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(direccion)
}