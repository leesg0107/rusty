mod hello_capnp {
    include!(concat!(env!("OUT_DIR"), "/hello_capnp.rs"));
}

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Fruit {
    name: String,
    price: u32,
}

fn encode() -> Result<Vec<u8>> {
    // make Person
    let mut message = capnp::message::Builder::new_default();
    {
        let mut person = message.init_root::<hello_capnp::person::Builder>();
        person.set_name("apple computer");
        person.set_id(10);
        person.set_email("abc@apple.com");
    }
    let mut output: Vec<u8> = vec![];
    capnp::serialize::write_message(&mut output, &message)?;
    println!("{}", hex::encode(&output));
    Ok(output)
}

fn decode(data: &[u8]) -> Result<()> {
    let mut cursor = std::io::Cursor::new(data);
    let message_reader =
        capnp::serialize::read_message(&mut cursor, capnp::message::ReaderOptions::new())?;
    let person = message_reader.get_root::<hello_capnp::person::Reader>()?;
    println!("name: {}", person.get_name()?.to_str()?);
    println!("id: {}", person.get_id());
    println!("email: {}", person.get_email()?.to_str()?);
    Ok(())
}
fn main() -> Result<()> {
    let encoded: Vec<u8> = encode()?;
    // print length of encoded
    println!("encoded length: {}", encoded.len());
    decode(&encoded)?;
    Ok(())
}
