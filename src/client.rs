use crate::state::{SharedState};
use crate::message::{Messageformat};
use tokio::sync::mpsc;
use warp::{filters::ws::{Message, WebSocket}};
use futures::{SinkExt, StreamExt};

pub async fn spawn_client(websocket: WebSocket, state: SharedState){
   let (mut sender, mut receiver) = websocket.split();

   let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let user_id = uuid::Uuid::new_v4().to_string();
    state.clients.insert(user_id.clone(), tx);

    println!("Client {} connected", user_id);

   //sender task
   tokio::spawn(async move{
        while let Some(msg) = rx.recv().await{
                let _ = sender.send(msg).await;

        };
   });

   //receiver task
   while let Some(msg) = receiver.next().await{
        //search the hashmap for whom this message is and send to that tx
        if let Ok(message) = msg{
            if message.is_text(){
                let client_message  = serde_json::from_str::<Messageformat>(message.to_str().unwrap()).unwrap();

                if let Some(sender_tx) = state.clients.get(&client_message.to){
                    let _ = sender_tx.send(message);
                };
            }
        } 
   }

    println!("Client {} disconnected.", user_id);
    state.clients.remove(&user_id);

}