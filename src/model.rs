use std::sync::Arc;

use serde::Serialize;
use tokio::sync::Mutex;

use crate::Error;

#[derive(serde::Deserialize, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(serde::Deserialize)]
pub struct TicketForCrate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self, ()> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, ticket: TicketForCrate) -> Result<Ticket, ()> {
        let mut store = self.tickets_store.lock().await;
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    //list all tickets
    pub async fn list_tickets(&self) -> Result<Vec<Ticket>, ()> {
        let store = self.tickets_store.lock().await;
        Ok(store.iter().filter_map(|t| t.clone()).collect())
    }

    //delete ticket
    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket, Error> {
        let mut store = self.tickets_store.lock().await;
        let ticket = store.get_mut(id as usize).and_then(|f| f.take());
        ticket.ok_or(crate::Error::TicketDeleteFailIdNotFound { id })
    }
}
