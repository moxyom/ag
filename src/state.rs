use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use tokio::sync::RwLock;

pub struct Counter(AtomicUsize);

impl Counter {
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    pub fn increment(&self) -> usize {
        self.0.fetch_add(1, Ordering::Relaxed)
    }

    pub fn get(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

pub struct Answer {
    pub message: Box<str>,
    pub vote_counter: Counter,
}

impl Answer {
    pub fn new(message: Box<str>) -> Self {
        Self {
            message,
            vote_counter: Counter::new(),
        }
    }
}

pub struct Vote {
    pub title: Box<str>,
    pub answers: Vec<Answer>,
    pub id: usize,
}

pub struct Room {
    pub current_vote: Option<Vote>,
}

impl Room {
    pub fn new() -> Self {
        Room { current_vote: None }
    }

    pub fn set_current_vote(&mut self, title: Box<str>, answers: Vec<Answer>) -> usize {
        let vote_id = match &self.current_vote {
            Some(prev) => prev.id + 1,
            None => 1,
        };

        self.current_vote = Some(Vote {
            title,
            answers,
            id: vote_id,
        });

        vote_id
    }

    pub fn vote_for(&self, vote_id: usize, answer: usize) {
        if let Some(vote) = &self.current_vote {
            if vote_id != vote.id {
                return;
            }

            match vote.answers.get(answer) {
                Some(a) => {
                    a.vote_counter.increment();
                }
                None => {
                    #[cfg(debug_assertions)]
                    println!("current vote {} has no answer {}", vote.id, answer);
                }
            }
        }
    }

    pub fn get_current_stats(&self) -> Vec<usize> {
        match &self.current_vote {
            Some(vote) => vote.answers.iter().map(|a| a.vote_counter.get()).collect(),
            None => vec![],
        }
    }
}

pub struct ClientState {
    pub sender: Option<SplitSink<WebSocket, Message>>,
    pub last_vote_id: AtomicUsize,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            sender: None,
            last_vote_id: AtomicUsize::new(0),
        }
    }
}

pub struct RoomState {
    pub room: Room,
    pub joinable: bool,
    pub clients: Vec<ClientState>
}

impl RoomState {
    pub fn new() -> Self {
        Self {
            room: Room::new(),
            joinable: true,
            clients: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub rooms: Arc<RwLock<HashMap<usize, Arc<RwLock<RoomState>>>>>,
    pub jwt_secret: Box<str>,
}

impl AppState {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            jwt_secret: jwt_secret.into(),
        }
    }
}
