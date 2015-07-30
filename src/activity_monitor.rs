use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};

use time::{self, Duration, Timespec};

use connection::{Event, Writer};
use message::Prefix;

#[derive(Clone)]
enum MonitorStatus {
    // Last activity, timespec is when.
    Activity(Timespec),
    // Ping was sent, timespec is when.
    Ping(Timespec),
}

#[derive(Clone)]
enum ConnectionStatus {
    // Connection is alive and well.
    Connected(MonitorStatus),
    // Connection was dropped.
    Disconnected,
    // When the monitor receives a close event, it sets the status
    // to this value to let the background thread know it should stop.
    Quit,
}

#[derive(Clone)]
struct State {
    // Holds the status of the connection.
    status: Arc<Mutex<ConnectionStatus>>,
    // When sending a ping message on irc, you need to specify the server.
    // This holds the current server's name, it's grabbed from the event
    // stream by the feed method.
    server: Arc<Mutex<Option<String>>>,
}

impl State {

    fn new(ts: Timespec) -> State {
        let conn_status = ConnectionStatus::Connected(MonitorStatus::Activity(ts));
        State {
            status: Arc::new(Mutex::new(conn_status)),
            server: Arc::new(Mutex::new(None)),
        }
    }

    // Set the last activity's timestamp.
    fn set_activity(&self, ts: Timespec) {
        *self.status.lock().unwrap() = ConnectionStatus::Connected(MonitorStatus::Activity(ts));
    }

    // Set the status to disconnected.
    fn set_disconnected(&self) {
        *self.status.lock().unwrap() = ConnectionStatus::Disconnected;
    }

    // Set the status to quit.
    fn quit(&self) {
        *self.status.lock().unwrap() = ConnectionStatus::Quit;
    }

    // Get the connection status.
    fn connection_status<'a>(&self) -> MutexGuard<ConnectionStatus> {
        self.status.lock().unwrap()
    }

    // Check if the server variable is set.
    fn has_server(&self) -> bool {
        self.server.lock().unwrap().is_some()
    }

    // Get the server's name.
    fn get_server(&self) -> MutexGuard<Option<String>> {
        self.server.lock().unwrap()
    }

    // Set the server's name.
    fn set_server(&self, name: String) {
        *self.server.lock().unwrap() = Some(name);
    }

    // Unsert the server name. Used when a disconnection occurs.
    fn unset_server(&self) {
        *self.server.lock().unwrap() = None;
    }

}

fn periodic_checker(state: State, handle: Writer) {
    loop {
        let mut conn_status = state.connection_status();

        // Clone here, to we can mutate the status inside the match.
        // The type is Copy anyway.
        match conn_status.clone() {
            ConnectionStatus::Connected(ref mon_status) => {
                match *mon_status {
                    // The monitor is in activity mode.
                    // If the timer expires, it will ping the server and set the monitor status
                    // to ping mode.
                    MonitorStatus::Activity(activity_ts) => {
                        let diff = time::get_time() - activity_ts;
                        if diff > Duration::seconds(10) {
                            println!("sending a ping");
                            // Make sure we have a server name.
                            match *state.get_server() {
                                Some(ref server) =>  {
                                    // Set the monitor's status to ping mode.
                                    *conn_status = ConnectionStatus::Connected(MonitorStatus::Ping(time::get_time()));
                                    // Send a ping, which should trigger activity is the connection is still alive.
                                    let _ = handle.ping(server);
                                }
                                None => {
                                    // This should really never happen. The server's name should be grabbed
                                    // almost instantly, when the first message is received from the server
                                    // after establishing a new connection.
                                    panic!("Server is None! This scenario is highly unlikely, please report this issue!");
                                }
                            }

                        }
                    }
                    // The monitor is in ping mode, which means it expects a ping response anytime.
                    // If the timer expires, the connection is set to disconnect mode.
                    MonitorStatus::Ping(ping_ts) => {
                        let diff = time::get_time() - ping_ts;
                        println!("checking {}", diff);
                        if diff > Duration::seconds(5) {
                            println!("no ping reply");
                            // trigger reconnection process
                            let _ = handle.disconnect();
                        }
                    },
                }
            },
            // Do nothing if the socket is disconnected, for now.
            ConnectionStatus::Disconnected => {},
            ConnectionStatus::Quit => break,
        }

        // Drop the lock to avoid a dead lock.
        drop(conn_status);
        thread::sleep_ms(1000);
    }
}

pub struct ActivityMonitor {
    state: State,
}

impl ActivityMonitor {

    pub fn new(handle: &Writer) -> ActivityMonitor {
        let state =  State::new(time::get_time());

        let state_clone = state.clone();
        let handle_clone = handle.clone();

        thread::spawn(move || {
            periodic_checker(state_clone, handle_clone);
        });

        ActivityMonitor {
            state: state,
        }
    }

    pub fn feed(&self, event: &Event) {
        match *event {
            Event::Closed(_) => {
                self.state.quit();
            }
            Event::Disconnected => {
                self.state.set_disconnected();
                self.state.unset_server();
            }
            Event::Reconnected => {
                self.state.set_activity(time::get_time());
            }
            Event::Message(ref msg) => {
                self.state.set_activity(time::get_time());
                if let Some(ref prefix) = msg.prefix {
                    match *prefix {
                        Prefix::Server(ref name) => {
                            if !self.state.has_server() {
                                self.state.set_server(name.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

}
