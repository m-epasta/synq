//! This file shows how to create a synq connection with a service/device

// The serialization can else be written in code as:
// ```rust
// // Make the strict serializable and deserializable
//  #[derive(Photon)]
//  struct MyFrame {
//      age: i32
//      name: String
//  }
// ```
// But you can call external proto/grpc/synq files.
// This example will define the frame/message as the first given method
use synq_codec::serialize;
use synq_core::{connect, send};

#[derive(Photon)]
struct Block {
    // Addres
    receiver: u64,
    sender: u64,
    amount: f64,
}

fn main() {
    let simp = Block{
        receiver: 0x06deaff0e0b5157e9a060b0e740f297107fd4cb35ff86672e1004e5954059fc837b835f62e04c0ec91ec7ee4791956a25e6a56cf4915672330adacab37671d7e2,
        sender: 0xa0e8116eb39f09ff2790085116d716a8006198ea5a3408d2baf4753949a7d7315e9b1696d2f04572c03215e3e1824530a8fadc561c3cc56bf019a4ec34d9d633,
        amount: 1.75372727538
    };

    let frame = serialize(simp);

    // Photon send over the local app/device network the message, it is up the client to pick it up
    // NOTE: OOP (method chaining) also works as frame.send() - so you could rewrite it as
    // ```rust
    //  simp.serialize.send();
    // ```
    send(frame)

    // Thats basically it, you can also wait for response or do post send operations. Just remember
    // that the server side is just:
    // build Frame/Photon/Messafe -> serialize it -> Send it
    // Of course, PHOTON can use differents way of sending/posting data, you can else configure it
    // via config files, custom synq files or with using specific functions/structs/macros from
    // library
}
