extern crate telecat;

extern crate futures;
extern crate tokio;
extern crate futures_retry;

use std::sync::Arc;
use std::env;

use futures::future;
use futures::{Future, Stream};

use futures_retry::{RetryPolicy, StreamRetryExt};

use telecat::StreamFlatExt;

fn main() {
    let token = env::var("TELECAT_TOKEN").expect("You must provide the bot token.");

    let bot = Arc::new(telecat::Bot::new([telecat::TG_API_URL, &token].concat()).unwrap());
    let raw_stream = telecat::RawStream::new(&bot);

    let running = raw_stream
        .retry(|_| RetryPolicy::Repeat)
        .flat_iter()
        .for_each(move |upd| {
            tokio::spawn(process(&*bot, upd));

            Ok(())
        });

    tokio::run(running);
}

fn process(bot: &telecat::Bot, upd: telecat::types::Update) -> impl Future<Item=(), Error=()> {
    println!("hello world, {:?}", upd);
    match bot.reply_to_message(&upd, "slap") {
        Ok(_) => future::ok(()),
        Err(_) => future::err(()),
    }
}
