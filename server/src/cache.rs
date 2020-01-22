use actix::prelude::*;
use log::*;

use quakes_api::*;
use crate::websocket;

pub(crate) struct CacheActor {
    quakes: Vec<Quake>,
    sessions: Vec<Recipient<websocket::CacheUpdates>>
}

impl Actor for CacheActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        debug!("Cache actor started...");
    }
}

impl CacheActor {

    fn update(&mut self, quake: Quake) {
        self.quakes.push(quake)
    }

    pub fn new(quakes: Vec<Quake>) -> CacheActor {
        let sessions = Vec::new();
        CacheActor { quakes, sessions }
    }
}

impl Default for CacheActor {
    fn default() -> Self {
        CacheActor::new(Vec::new())
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UpdateCache(pub Quake);

impl Handler<UpdateCache> for CacheActor {
    type Result = ();

    fn handle(&mut self, msg: UpdateCache, _: &mut Self::Context) -> Self::Result {
        debug!("Received cache update for 1 quake...");
        let quake = msg.0;
        self.update(quake.clone());
        let list = QuakeList::new(vec![quake]);
        for session in self.sessions.iter() {
            session.do_send(websocket::CacheUpdates(list.clone())).unwrap();
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<websocket::CacheUpdates>,
}

impl Handler<Connect> for CacheActor {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        debug!("New websocket connection to cache!");
        let addr = msg.addr;
        let list = QuakeList::new(self.quakes.clone());
        addr.do_send(websocket::CacheUpdates(list)).unwrap();
        self.sessions.push(addr);
    }
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub addr: Recipient<websocket::CacheUpdates>,
}

impl Handler<Disconnect> for CacheActor {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        debug!("Disconnecting websocket from cache...");
        self.sessions.retain(|session| !session.eq(&&msg.addr));
        debug!("Updated sessions: {:?}", &self.sessions);
    }
}



