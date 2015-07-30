extern crate irc;

use std::env;

/// Say "peekaboo" in a channel on freenode and then quit.
/// target/debug/examples/peekaboo "#mychannel"
fn main() {
    let args: Vec<String> = env::args().collect();

    let channel = args.get(1).unwrap_or_else(|| {
        println!("Channel must be given as an argument.");
        panic!();
    });

    // Connect to freenode and use no not reconnect.
    let (writer, reader) = irc::connect("irc.freenode.net:6667",
                                        Some(irc::ReconnectionSettings::DoNotReconnect)).unwrap();
    writer.user("peekaboo", "peekaboo bot");
    writer.nick("peekaboo");

    // Receive events.
    for event in reader.iter() {
        println!("{:?}", event);
        match event {
            irc::Event::Message(msg) => {
                if msg.code == irc::Code::RplWelcome {
                    // join channel, no password
                    writer.join(channel, None);
                }
                // JOIN is sent when you join a channel.
                if msg.code == irc::Code::Join {
                    // If there is a prefix...
                    if let Some(prefix) = msg.prefix {
                        match prefix {
                            // And the prefix is a user...
                            irc::Prefix::User(user) => {
                                // And that user's nick is peekaboo, we've joined the channel!
                                if user.nick == "peekaboo" {
                                    writer.privmsg(channel, "peekaboo");
                                    // Note that if the reconnection settings said to reconnect,
                                    // it would. Close would "really" stop it.
                                    writer.quit(Some("peekaboo"));
                                    // writer.close();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}