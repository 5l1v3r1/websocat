extern crate tokio_stdin_stdout;

use super::{BoxedNewPeerFuture, Peer};

use super::{once, ConstructParams, PeerConstructor, Specifier};

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ThreadedStdio;
impl Specifier for ThreadedStdio {
    fn construct(&self, _: ConstructParams) -> PeerConstructor {
        once(get_stdio_peer())
    }
    specifier_boilerplate!(globalstate singleconnect no_subspec);
}

specifier_class!(
    name = ThreadedStdioClass,
    target = ThreadedStdio,
    prefixes = ["threadedstdio:"],
    arg_handling = noarg,
    overlay = false,
    StreamOriented,
    SingleConnect,
    help = r#"
Stdin/stdout, spawning a thread. [A]

Like `-`, but forces threaded mode instead of async mode

Use when standard input is not `epoll(7)`-able or you want to avoid setting it to nonblocking mode.
"#
);
#[cfg(not(all(unix, feature = "unix_stdio")))]
specifier_class!(
    name = ThreadedStdioSubstituteClass,
    target = ThreadedStdio,
    prefixes = ["-", "stdio:"],
    arg_handling = noarg,
    overlay = false,
    StreamOriented,
    SingleConnect,
    help = r#"
Read input from console, print to console (threaded version).

This specifier can be specified only one time.
"#
);

#[cfg(not(all(unix, feature = "unix_stdio")))]
specifier_class!(
    name = InetdClass,
    target = ThreadedStdio,
    prefixes = ["inetd:"],
    arg_handling = noarg,
    overlay = false,
    StreamOriented,
    SingleConnect,
    help = r#"
Alias of stdio: (threaded version).
"#
);

pub fn get_stdio_peer() -> BoxedNewPeerFuture {
    info!("get_stdio_peer (threaded)");
    Box::new(::futures::future::ok(Peer::new(
        tokio_stdin_stdout::stdin(0),
        tokio_stdin_stdout::stdout(0),
        None,
    ))) as BoxedNewPeerFuture
}
