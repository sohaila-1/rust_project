use std::net::TcpStream;
use std::io::{Write, Read};

use ciborium::{ser::into_writer, de::from_reader};

mod protocol;
use protocol::{Request, Response};

fn main() {
    println!("🚀 Connexion au serveur...");

    let mut stream = TcpStream::connect("127.0.0.1:8000")
        .expect("❌ Impossible de se connecter");

    println!("✅ Connecté !");

    // 🔥 Message
    let request = Request::Order {
        recipe_name: "Margherita".to_string(),
    };

    // 🔥 Sérialisation CBOR
    let mut send_buffer = Vec::new();
    into_writer(&request, &mut send_buffer).unwrap();

    println!("📤 Envoi de la requête...");
    stream.write_all(&send_buffer).unwrap();

    // 🔥 Lecture NON bloquante
    println!("📥 Attente de la réponse...");

    // 🔹 Lire la taille (4 bytes)
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();

    let len = u32::from_be_bytes(len_buf);
    println!("📏 Taille du message: {}", len);

    // 🔹 Lire le message CBOR
    let mut data = vec![0u8; len as usize];
    stream.read_exact(&mut data).unwrap();

    println!("📦 Données CBOR: {:?}", data);

    // 🔹 Décoder
    match from_reader::<Response, _>(&data[..]) {
        Ok(response) => {
            println!("🍕 Réponse décodée : {:#?}", response);
        }
        Err(e) => {
            println!("❌ Erreur décodage: {:?}", e);
        }
    }
}