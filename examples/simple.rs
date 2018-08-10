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
            tokio::spawn(handle1(&*bot, &upd));
            tokio::spawn(handle2(&*bot, &upd));

            seq_handle(&*bot, &upd);

            Ok(())
        });

    tokio::run(running);
}

fn handle1(bot: &telecat::Bot, upd: &telecat::types::Update) -> impl Future<Item=(), Error=()> {
    match bot.reply_to_message(upd, "async handle1") {
        Ok(_) => future::ok(()),
        Err(_) => future::err(()),
    }
}

fn handle2(bot: &telecat::Bot, upd: &telecat::types::Update) -> impl Future<Item=(), Error=()> {
    match bot.reply_to_message(upd, "async handle2") {
        Ok(_) => future::ok(()),
        Err(_) => future::err(()),
    }
}

fn seq_handle(bot: &telecat::Bot, upd: &telecat::types::Update) -> Result<telecat::types::Message, telecat::error::Error> {
    bot.reply_to_message(upd, "sequential handler")
}
