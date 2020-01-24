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

    fn update(&mut self, mut quakes: Vec<Quake>) {
        quakes.retain(|quake| !self.quakes.contains(quake) );
        debug!("Will add the following quakes to the cache:\n{:#?}", &quakes);
        self.quakes.extend(quakes)
    }

    pub fn new(quakes: Vec<Quake>) -> CacheActor {
        let sessions = Vec::new();
        CacheActor { quakes, sessions }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UpdateCache(pub Vec<Quake>);

impl Handler<UpdateCache> for CacheActor {
    type Result = ();

    fn handle(&mut self, msg: UpdateCache, _: &mut Self::Context) -> Self::Result {
        debug!("Received cache updates:\n{:#?}", &msg.0);
        let quakes = msg.0;
        self.update(quakes.clone());
        let list = QuakeList::new(quakes);
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



