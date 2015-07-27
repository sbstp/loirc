extern crate irc;

use std::thread;

fn main() {
    let (writer, reader) = irc::connect("irc.freenode.net:6667").unwrap();
    writer.user("scarlet", "scarlet");
    writer.nick("scarlet-irc");

    let pinger = writer.clone();
    thread::spawn(move || {
        thread::sleep_ms(15 * 1000);
        pinger.raw("PING irc.freenode.net\n".into());
        println!("sent ping");
    });

    for event in reader.iter() {
        println!("{:?}", event);
        match event {
            irc::Event::Message(msg) => {
                if msg.code == irc::Code::RplWelcome {
                    writer.close();
                }
                //println!("{:?}", msg);
            }
            irc::Event::ParseError(err) => {
                println!("{:?}", err);
            }
            _ => {}
        }
    }
    println!("we done heere");
}
