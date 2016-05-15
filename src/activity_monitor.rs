use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};

use std::time::{Duration, Instant};

use connection::{Event, Writer};
use message::Prefix;

#[derive(Clone)]
enum MonitorStatus {
    // Last activity, timespec is when.
    Activity(Instant),
    // Ping was sent, timespec is when.
    Ping(Instant),
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

    fn new(ts: Instant) -> State {
        let conn_status = ConnectionStatus::Connected(MonitorStatus::Activity(ts));
        State {
            status: Arc::new(Mutex::new(conn_status)),
            server: Arc::new(Mutex::new(None)),
        }
    }

    // Set the last activity's timestamp.
    fn set_activity(&self, ts: Instant) {
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

fn periodic_checker(state: State, handle: Writer, settings: MonitorSettings) {
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
                        let diff = activity_ts.elapsed();
                        if diff > settings.activity_timeout {
                            // Make sure we have a server name.
                            match *state.get_server() {
                                Some(ref server) =>  {
                                    // Set the monitor's status to ping mode.
                                    *conn_status = ConnectionStatus::Connected(MonitorStatus::Ping(Instant::now()));
                                    // Send a ping, which should trigger activity is the connection is still alive.
                                    let _ = handle.raw(format!("PING {}\n", server));
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
                        let diff = ping_ts.elapsed();
                        if diff > settings.ping_timeout {
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
        thread::sleep(Duration::from_secs(1));
    }
}

/// These settings tell the monitor how to behave.
///
/// They allow you to configure the amount of time between the steps.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MonitorSettings {
    /// Amount of time since the last activity.
    ///
    /// When the amount of time since the last activity gets higher than this value,
    /// tt will trigger a ping request. If there is not a lot of activity, this means
    /// that the server will be pinged everytime this duration expires.
    pub activity_timeout: Duration,
    /// Amount of time to wait for a ping reply.
    ///
    /// When the amount of time since the ping was sent gets higher than this value,
    /// and that no activity occured, assume the connection was dropped and trigger
    /// and a disconnect.
    pub ping_timeout: Duration,
}

/// Default values are provided for the settings.
///
/// They are:
///
/// `activity_timeout` = 60 seconds
///
/// `ping_timeout` = 15 seconds
impl Default for MonitorSettings {

    fn default() -> MonitorSettings {
        MonitorSettings {
            activity_timeout: Duration::from_secs(60),
            ping_timeout: Duration::from_secs(15),
        }
    }

}

/// This struct monitors a connection's activity.
///
/// It works in a few simple, steps.
/// First, it monitors activity via the feed method, saving a timestamp for relevant events.
/// If no activity is detected over the given period of time, it will send a ping request to
/// the IRC server. If no ping  reply is received, it will trigger a disconnect which may lead
/// to reconnection depending on the reconnection settings.
///
/// The amount of time to wait, without activity, before sending a ping request, and the amount
/// of time to wait for the ping reply can be configured via the `MonitorSettings` struct.
pub struct ActivityMonitor {
    state: State,
}

impl ActivityMonitor {

    /// Create a new ActivityMonitor.
    ///
    /// The handle to a Writer allows the monitor to notify the connection of disconnects.
    pub fn new(handle: &Writer, settings: MonitorSettings) -> ActivityMonitor {
        let state =  State::new(Instant::now());

        let state_clone = state.clone();
        let handle_clone = handle.clone();

        thread::spawn(move || {
            periodic_checker(state_clone, handle_clone, settings);
        });

        ActivityMonitor {
            state: state,
        }
    }

    /// Give an event received from the connection to the monitor.
    ///
    /// The monitor will process it accordingly. If an Event::Closed event
    /// is received, it will shutdown all of its activities.
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
                self.state.set_activity(Instant::now());
            }
            Event::Message(ref msg) => {
                self.state.set_activity(Instant::now());
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
            // Other events are irrelevant.
            _ => {}
        }
    }

}

/// Drop stops the background thread and clears the monitor's resources.
///
/// If you want the activity monitor to cease its activites, you can simply drop it.
/// It will not affect the connection on which the activity monitor operates.
impl Drop for ActivityMonitor {

    fn drop(&mut self) {
        self.state.quit();
    }

}
