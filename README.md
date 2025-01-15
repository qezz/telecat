# ⚠️ DEPRECATION NOTICE

This crate is deprecated. Use awesome
[carapax](https://github.com/tg-rs/carapax) for a modern take on
Telegram Bots and Rust futures.

# Telecat

Telecat is a simple library for making Telegram bots

Telecat is young, that's why it is based on the third-party library
teleborg. This may change in future.

Contributions are welcome!

## Getting started

Quick dive

```bash
export TELECAT_TOKEN=yourbot_token
cargo run --example simple
```

And now your bot will replay to every message it receives

## More deeper

Telecat is made with support of futures in mind (futures
0.1). However, you can simply not use the futures, and simply process
the update within `for_each(|| ... )` pipeline.

```rust
extern crate telecat;

// ...

fn main() {
    let token = env::var("TELECAT_TOKEN").expect("You must provide the bot token.");

    let bot = Arc::new(telecat::Bot::new([telecat::TG_API_URL, &token].concat()).unwrap());
    let raw_stream = telecat::RawStream::new(&bot);

    let running = raw_stream
        .retry(|_| RetryPolicy::Repeat)
        .flat_iter()
        .for_each(move |upd| {
            tokio::spawn(process(&*bot, &upd));
            sequential(&*bot, &upd);
            Ok(())
        });

    tokio::run(running);
}

fn process(bot: &telecat::Bot, upd: &telecat::types::Update) -> impl Future<Item=(), Error=()> {
    println!("hello world, {:?}", upd);
    match bot.reply_to_message(&upd, "slap") {
        Ok(_) => future::ok(()),
        Err(_) => future::err(()),
    }
}

fn sequential(bot: &telecat::Bot, upd: &telecat::types::Update) -> Result<telecat::types::Message, telecat::error::Error> {
    bot.reply_to_message(upd, "sequential handler")
}
```

## License

MIT or Apache 2.0, at your choice.

## Thanks!

Big thanks to [@mexus](http://github.com/mexus) for the great
introduction to futures and tokio; and also for a large help and
patience!
