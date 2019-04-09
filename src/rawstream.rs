use futures::{Async, Stream};
use teleborg as tg;
use std::sync::Arc;

use teleborg::error::Error::JsonError;

// RawStreamer is streaming updates from telegram api as is
pub struct RawStream {
    bot: Arc<tg::Bot>,
    last_update_id: Option<i32>,
}

impl RawStream {
    pub fn new(bot: &Arc<tg::Bot>) -> RawStream {
        RawStream {
            last_update_id: None,
            bot: bot.clone(),
        }
    }
}

impl Stream for RawStream {
    type Item = Vec<tg::objects::Update>;
    type Error = (); // io::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {

        let _poll_interval = Some(0);
        let timeout = Some(10);
        let network_delay = Some(0.0);

        loop {
            let pending_updates = self.bot.get_updates(
                self.last_update_id.unwrap_or(0),
                None,
                timeout,
                network_delay
            );

            println!("pending_updates: {:?}", pending_updates);

            match pending_updates {
                Ok(Some(v)) => {
                    self.last_update_id = v.iter().map(|upd| upd.update_id as i32 + 1).max();
                    return Ok(Async::Ready(Some(v)));
                },
                Ok(None) => {
                    return Ok(Async::Ready(Some(vec![])))
                }
                Err(e) => {
                    // DIRTY HACK
                    match e {
                        _ => {
                            if let Some(ref mut x) = self.last_update_id {
                                *x = -1;
                            }
                            return Ok(Async::Ready(Some(vec![])))
                        }
                    }
                }
            }
        }
    }
}
