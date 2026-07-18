pub mod handler;
use actix_web::dev::ServerHandle;
pub use handler::*;

pub use crate::handler::*;

use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
};

use actix_web::{App, HttpServer, middleware, web};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::*;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ServerState {
    #[default]
    StopServer,
    RunServer,
}

#[derive(Resource)]
pub struct NetChannels {
    pub tx: Sender<ServerState>,
    pub rx: Receiver<NetCommand>,
}

pub fn stop_actix(channels: Res<NetChannels>) {
    info!("Sending Stop");
    if let Err(e) = channels.tx.try_send(ServerState::StopServer) {
        error!("in stop_actix: {:?}", e)
    }
}

pub fn setup_actix(mut commands: Commands) {
    let (tx_control, rx_control) = async_channel::unbounded();
    let (tx_update, rx_update) = async_channel::unbounded();

    IoTaskPool::get()
        .spawn(async move { net_code(rx_control, tx_update).await })
        .detach();

    commands.insert_resource(NetChannels {
        tx: tx_control,
        rx: rx_update,
    })
}

pub struct ActixData {
    tx: Sender<NetCommand>,
}

async fn net_code(rx_control: Receiver<ServerState>, tx_update: Sender<NetCommand>) {
    IoTaskPool::get()
        .spawn(async move { init_actix(rx_control, tx_update) })
        .detach()
}

#[actix_web::main]
async fn init_actix(rx: Receiver<ServerState>, tx: Sender<NetCommand>) -> std::io::Result<()> {
    info!("init_actix");
    // Start Actix Instance with channels
    let data = web::Data::new(Mutex::new(ActixData { tx: tx }));
    let srv = HttpServer::new({
        let data = web::Data::clone(&data);
        move || {
            App::new()
                .app_data(data.clone())
                .wrap(middleware::Logger::default())
                .service(handler::handler)
        }
    })
    .workers(4)
    .bind(("localhost", 4875))?
    .run();

    let h = srv.handle();

    IoTaskPool::get()
        .spawn(async move {
            while let Ok(msg) = rx.recv().await {
                info!("MSG RECV: {:?}", msg);
                if msg == ServerState::StopServer {
                    h.stop(true).await;
                }
            }
        })
        .detach();

    srv.await
}
