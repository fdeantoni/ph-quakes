use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::*;

use quakes_api::*;

use crate::cache;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct WsActor {
    cache: Addr<cache::CacheActor>,
    hb: Instant,
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Websocket actor started...");
        self.hb(ctx);
        let addr = ctx.address().recipient();
        self.cache.do_send(cache::Connect {
                addr,
            });
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("Websocket has stopped.");
        let addr = ctx.address().recipient();
        self.cache.do_send(cache::Disconnect {
            addr
        })
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                debug!("WS Text: {:?}", &text);
            },
            Ok(ws::Message::Binary(_)) => {
                debug!("WS Binary");
            },
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WsActor {
    fn new(cache_addr: Addr<cache::CacheActor>) -> Self {
        Self {
            cache: cache_addr,
            hb: Instant::now()
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                error!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CacheUpdates(pub QuakeList);

impl Handler<CacheUpdates> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: CacheUpdates, ctx: &mut Self::Context) -> Self::Result {
        debug!("Received new quakes to send to client...");
        let quakes = msg.0.to_geojson();
        ctx.text(quakes.to_string());
    }
}

pub(crate) async fn index(r: HttpRequest, stream: web::Payload, cache: web::Data<Addr<cache::CacheActor>>) -> Result<HttpResponse, Error> {
    debug!("{:?}", r);
    let cache_addr= cache.get_ref().clone();
    let res = ws::start(WsActor::new(cache_addr), &r, stream);
    debug!("{:?}", res);
    res
}

