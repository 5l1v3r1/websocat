extern crate websocket_lowlevel;

use futures::future::Future;
use futures::stream::Stream;

use std::cell::RefCell;
use std::rc::Rc;

use super::{box_up_err, peer_err, peer_strerr, BoxedNewPeerFuture, Peer, Result};

use super::ws_peer::{Mode1, PeerForWs, WsReadWrapper, WsWriteWrapper};
use super::{once, ConstructParams, Options, PeerConstructor, Specifier};

use self::websocket_lowlevel::codec::ws::Context as WsLlContext;

#[derive(Debug, Clone)]
pub struct WsLlClient<T: Specifier>(pub T);
impl<T:Specifier> Specifier for WsLlClient<T> {
    fn construct(&self, p: ConstructParams) -> PeerConstructor {
        let inner = self.0.construct(p.clone());
        let opts = p.program_options;
        inner.map(move |q, _| get_ws_lowlevel_peer(
            WsLlContext::Client,
            q,
            opts.clone(),
        ))
    }
    specifier_boilerplate!(noglobalstate singleconnect has_subspec);
}
specifier_class!(
    name = WsLlClientClass,
    target = WsLlClient,
    prefixes = ["ws-lowlevel-client:","ws-ll-client:","ws-ll-c:"],
    arg_handling = subspec,
    overlay = false,
    MessageOriented,
    SingleConnect,
    help = r#"
[A] Low-level HTTP-independent WebSocket client connection without associated HTTP upgrade.

Example: TODO
"#
);

#[derive(Debug, Clone)]
pub struct WsLlServer<T: Specifier>(pub T);
impl<T:Specifier> Specifier for WsLlServer<T> {
    fn construct(&self, p: ConstructParams) -> PeerConstructor {
        let inner = self.0.construct(p.clone());
        let opts = p.program_options;
        inner.map(move |q, _| get_ws_lowlevel_peer(
            WsLlContext::Server,
            q,
            opts.clone(),
        ))
    }
    specifier_boilerplate!(noglobalstate singleconnect has_subspec);
}
specifier_class!(
    name = WsLlServerClass,
    target = WsLlServer,
    prefixes = ["ws-lowlevel-server:","ws-ll-server:","ws-ll-s:"],
    arg_handling = subspec,
    overlay = false,
    MessageOriented,
    SingleConnect,
    help = r#"
[A] Low-level HTTP-independent WebSocket server connection without associated HTTP upgrade.

Example: TODO
"#
);

pub fn get_ws_lowlevel_peer(mode: WsLlContext, inner: Peer, opts: Rc<Options>) -> BoxedNewPeerFuture {
    info!("get_ws_lowlevel_peer");
    unimplemented!()
}