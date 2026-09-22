use screenseal_core::protocol::message::Message;

#[test]
fn mensaje_se_serializa_y_deserializa_correctamente() {
    let mensaje_original = Message::Hello {
        version: 1,
    };

    let bytes = mensaje_original
        .encode()
        .expect("No se pudo serializar el mensaje");

    let mensaje_recibido = Message::decode(&bytes)
        .expect("No se pudo deserializar el mensaje");

    assert_eq!(mensaje_original, mensaje_recibido);
}