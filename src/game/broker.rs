use anyhow::anyhow;
use serde::{Serialize, Deserialize};

use crate::{Coord, Game, CoordPair};

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

impl Game {
    pub fn broker_post_move(&self, action_move: CoordPair) -> Result<(),anyhow::Error> {
        let turn = self.total_moves();
        let from = action_move.from;
        let to = action_move.to;
        let data = BrokerData{ from, to, turn };
        let client = reqwest::blocking::Client::new();
        let broker_url = self.options().broker.as_ref().ok_or(anyhow!("no broker"))?.clone();
        let res = client.post(broker_url)
            .body(serde_json::to_string(&data)?)
            .send()?;
        let status = res.status().as_u16();
        match status {
            200 | 404 => {
                let broker_reply : BrokerReply = serde_json::from_str(res.text()?.as_str())?;
                if status == 404 && broker_reply.error.is_some() {
                    return Err(anyhow!("Broker error: {}", broker_reply.error.unwrap()));
                }
                if status == 200 && broker_reply.success && broker_reply.data.is_some() {
                    let check_data = broker_reply.data.unwrap();
                    if data == check_data {
                        return Ok(());
                    }
                }
                Err(anyhow!("Broker error: unknown"))
            },
            status => {
                Err(anyhow!("Broker error: http status {status}"))
            }
        }
    }
    pub fn broker_get_move(&self) -> Result<Option<CoordPair>,anyhow::Error> {
        let broker_url = self.options().broker.as_ref().ok_or(anyhow!("no broker"))?.clone();
        let res = reqwest::blocking::get(broker_url)?;
        let status = res.status().as_u16();
        match status {
            200 | 404 => {
                let broker_reply : BrokerReply = serde_json::from_str(res.text()?.as_str())?;
                if status == 404 && broker_reply.error.is_some() {
                    return Err(anyhow!("Broker error: {}", broker_reply.error.unwrap()));
                }
                if status == 200 && broker_reply.success {
                    if let Some(data) = broker_reply.data {
                        // broker has some data for us
                        if data.turn == self.total_moves()+1 {
                            // the broker has our next move
                            return Ok(Some(CoordPair::new(data.from, data.to)))
                        } else {
                            // return Err(anyhow!("Broker error: expecting data for turn {}, got turn {}",self.total_moves()+1,data.turn))
                            return Ok(None)
                        }
                    } else {
                        // broker has no data yet
                        // return Err(anyhow!("Broker error: no data available yet"));
                        return Ok(None)
                    }
                } else {
                    Err(anyhow!("Broker error: unknown"))
                }
            },
            status => {
                Err(anyhow!("Broker error: http status {status}"))
            }
        }
    }
}
