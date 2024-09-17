use sails_rs::{calls::*, gtest::calls::*};

use delayed_message_client::traits::*;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn do_something_works() {
    let remoting = GTestRemoting::new(ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(delayed_message::WASM_BINARY);

    let program_factory = delayed_message_client::DelayedMessageFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = delayed_message_client::DelayedMessage::new(remoting.clone());

    let result = service_client
        .send_delayed_message() // Call service's method (see app/src/lib.rs:14)
        .send_recv(program_id)
        .await
        .unwrap();

    remoting.system().spend_blocks(10);

}
