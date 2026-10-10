use axum::extract::ws::Message;
use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct ClientIn {
    #[serde(rename = "voteId")]
    pub vote_id: usize,
    #[serde(rename = "answerId")]
    pub answer_id: usize,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum AdminIn {
    NewVoteIn {
        title: Box<str>,
        answers: Vec<Box<str>>,
    },
    SetVoteInscription {
        open: bool,
    },
    Terminate {
        terminate: bool,
    },
}

pub enum ParsedMessage<T> {
    Data(T),
    Ping,
    ParseError,
}

pub fn parse<T>(message: Message) -> ParsedMessage<T>
where
    T: for<'a> Deserialize<'a>,
{
    match message {
        Message::Text(text) => {
            if let Ok(answer) = serde_json::from_str(&text) {
                return ParsedMessage::Data(answer);
            }

            ParsedMessage::ParseError
        }
        Message::Ping(_) => ParsedMessage::Ping,
        _ => ParsedMessage::ParseError,
    }
}
