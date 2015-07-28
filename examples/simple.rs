extern crate irc;

use std::thread;

fn main() {
    // Connect to freenode and use default reconnection settings.
    let (writer, reader) = irc::connect("irc.freenode.net:6667", None).unwrap();
    writer.user("myuser", "my real name");
    writer.nick("mynick");

    // Receive events.
    for event in reader.iter() {
        println!("{:?}", event);
        match event {
            irc::Event::Message(msg) => {
                if msg.code == irc::Code::RplWelcome {
                    // join #mychannel, no password
                    writer.join("#mychannel", None);
                }
            }
            _ => {}
        }
    }
}
