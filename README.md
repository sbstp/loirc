# loirc
**loirc** is a **lo**w level **irc** client library. It's with built robustness in mind.
Automatic reconnections are built into the core, and
[utilities](https://sbstp.github.io/loirc/loirc/struct.ActivityMonitor.html)
are available to increase reliability. It's the perfect library to use on
fragile network connections such as Wi-Fi, but it's also very useful for any
type of clients, such as bots and loggers, that require high availability.

[Events](https://sbstp.github.io/loirc/loirc/enum.Event.html) are read from a channel, and
communications are sent via [Writers](https://sbstp.github.io/loirc/loirc/struct.Writer.html).
Event processing can be a bit tedious, hence why this is considered low level.

A library named [hiirc](https://github.com/SBSTP/hiirc) built on top of this is in active
development, it will provide the same robustness, but with a much friendlier, event-based API.

The [documentation](https://sbstp.github.io/loirc/loirc/index.html) is pretty good,
please refer to it for more information.
Examples are also available in the `examples` folder.

Server side is not a goal of **loirc** at the moment.

## License
zlib license, see [LICENSE](LICENSE)
