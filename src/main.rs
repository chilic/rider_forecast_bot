pub mod wather;

use dotenv::dotenv;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    payloads::SendMessageSetters,
    prelude::*,
    types::{ButtonRequest, KeyboardButton, KeyboardMarkup},
};

type BotDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveLocation,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: BotDialogue, msg: Message) -> HandlerResult {
    match msg.location() {
        Some(location) => {
            bot.send_message(msg.chat.id, location.longitude.to_string())
                .await?;
            dialogue.exit().await?;
        }
        None => {
            let button = KeyboardButton::new("Share location").request(ButtonRequest::Location);
            let markup = KeyboardMarkup::new([[button]])
                .one_time_keyboard(true)
                .resize_keyboard(true);

            bot.send_message(msg.chat.id, "Location?")
                .reply_markup(markup)
                .await?;
        }
    }
    Ok(())
}
