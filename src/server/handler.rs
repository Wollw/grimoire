use actix_web::{post, web};
use async_channel::Sender;
use bevy::prelude::*;
use egui::emath::Float;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::*;
use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
};

use crate::ActixData;

#[derive(Resource, Clone, Serialize, Deserialize, Debug)]
pub struct NetCommand {
    pub cmd: String,
    pub params: Option<HashMap<String, StringOrNumber>>,
}
pub type NetCommandDeque = VecDeque<NetCommand>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde[untagged]]
pub enum StringOrNumber {
    String(String),
    Number(f32),
}

#[post("/")]
pub async fn handler(
    req_body: String,
    data: web::Data<Mutex<ActixData>>,
) -> actix_web::Result<String> {
    let cmd: NetCommand = match serde_json::from_str(req_body.as_str()) {
        Ok(cs) => cs,
        Err(e) => return Ok(e.to_string()),
    };
    info!("Sending Actix Message: {:?}", cmd);
    if let Err(e) = data.lock().tx.try_send(cmd) {
        return Ok(e.to_string());
    }
    info!("Sent Actix Message");
    Ok("Command Received".to_string())
}
