use crate::state::{SharedState};
use tokio::sync::mpsc;
use warp::{filters::ws::{Message, WebSocket}};
use futures::{SinkExt, StreamExt};

pub async fn spawn_client(websocket: WebSocket, state: SharedState){
   let (mut sender, mut receiver) = websocket.split();

   let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let user_id = uuid::Uuid::new_v4().to_string();

    state.clients.insert(user_id.clone(), tx);

   //sender task
   tokio::spawn(async move{
        while let Some(msg) = rx.recv().await{
                let _ = sender.send(msg).await;

        };
   });

   while let Some(msg) = receiver.next().await{
        //search the hashmap for whom this message is and send to that tx
        if let Ok(message) = msg{
            if message.is_text(){
                println!("{} sent: {}", user_id,message.to_str().unwrap());

                if let Some(sender_tx) = state.clients.get(&user_id){
                    let _ = sender_tx.send(message);
                };
            }
        } 
   }

    println!("Client {} disconnected.", user_id);
    state.clients.remove(&user_id);

}