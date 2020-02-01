use actix::prelude::*;
use actix::dev::{MessageResponse, ResponseChannel};
use log::*;

use quakes_api::*;
use crate::websocket;

const CACHE_LIMIT: usize = 20_000;

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
        let mut quakes = msg.0.clone();
        debug!("Current cache size: {}", &self.quakes.len());
        debug!("Updates:\n{:#?}", &quakes);
        quakes.retain(|quake| !self.quakes.contains(quake) );
        if !quakes.is_empty() {

            debug!("Will add the following quakes to the cache:\n{:#?}", &quakes);
            self.quakes.extend(quakes.clone());

            // Remove old quakes exceeding cache limit
            if self.quakes.len() > CACHE_LIMIT {
                let number_to_remove = self.quakes.len() - CACHE_LIMIT;
                debug!("Cache limit exceeded, dropping {}.", &number_to_remove);
                self.quakes.drain(0..number_to_remove);
            }

            // Send new quakes to subscribers
            let list = QuakeList::new(quakes);
            debug!("Sending to clients:\n{:#?}", &list);
            for session in self.sessions.iter() {
                session.do_send(websocket::CacheUpdates(list.clone())).unwrap();
            }
        }
    }
}


pub struct GetQuakes;
pub struct GetQuakesResponse(pub QuakeList);

impl<A, M> MessageResponse<A, M> for GetQuakesResponse
    where
        A: Actor,
        M: Message<Result = GetQuakesResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut <A as Actor>::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl Message for GetQuakes {
    type Result = GetQuakesResponse;
}

impl Handler<GetQuakes> for CacheActor {
    type Result = GetQuakesResponse;

    fn handle(&mut self, _: GetQuakes, _: &mut Self::Context) -> Self::Result {
        let quakes = QuakeList::new(self.quakes.clone());
        GetQuakesResponse(quakes)
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



