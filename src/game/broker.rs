use std::any;

use anyhow::anyhow;
use serde::{Serialize, Deserialize};

use crate::{Coord, Game, CoordPair};

#[allow(non_upper_case_globals)]
const broker_url : &str = "http://localhost:8001/test";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
struct BrokerData {
    from: Coord,
    to: Coord,
    turn: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BrokerReply {
    success: bool,
    error: Option<String>,
    data: Option<BrokerData>,    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum BrokerResult {

}

impl Game {
    pub fn broker_post_move(&self, action_move: CoordPair) -> Result<(),anyhow::Error> {
        let turn = self.total_moves();
        let from = action_move.from;
        let to = action_move.to;
        let data = BrokerData{ from, to, turn };
        let client = reqwest::blocking::Client::new();
        let res = client.post(broker_url)
            .body(serde_json::to_string(&data)?)
            .send()?;
        let status = res.status().as_u16();
        match status {
            200 | 400 => {
                let broker_reply : BrokerReply = serde_json::from_str(res.text()?.as_str())?;
                println!("{broker_reply:#?}");
                if status == 400 && broker_reply.error.is_some() {
                    return Err(anyhow!("Broker error: {}", broker_reply.error.unwrap()));
                }
                if status == 200 && broker_reply.data.is_some() {
                    let check_data = broker_reply.data.unwrap();
                    if data == check_data {
                        return Ok(());
                    }
                }
                Err(anyhow!("Broker error: unknown"))
            },
            status => {
                Err(anyhow!("http status {status}"))
            }
        }
    }
    pub fn broker_get_move() -> Result<CoordPair,anyhow::Error> {
        let res = reqwest::blocking::get(broker_url)?;
        if res.status() == 200 {
            let broker_reply : BrokerReply = serde_json::from_str(res.text()?.as_str())?;
            return Ok(CoordPair::new(broker_reply.data.unwrap().from, broker_reply.data.unwrap().to))
        };
        Err(anyhow!("broker error"))
    }
}
