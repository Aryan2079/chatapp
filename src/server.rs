use tokio::sync::mpsc;
use warp::{filters::ws::{Message, WebSocket}, Filter};
use futures::{SinkExt, StreamExt};

pub async fn start(){
    let filter = warp::path("ws")
        .and(warp::ws())
        .map(|ws:warp::ws::Ws|{
            ws.on_upgrade(|ws|async move{
                handle_connection(ws);
                // println!("connection is established");
            })
 
        });
   
    println!("server starting in 127.0.0.1:3030");

    warp::serve(filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_connection(websocket: WebSocket){
   let (mut sender, mut receiver) = websocket.split();

   let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

   //sender task
   tokio::spawn(async move{
        while let Some(msg) = rx.recv().await{
                let _ = sender.send(msg).await;
        };
   });

   while let Some(msg) = receiver.next().await{
        //search the hashmap for whom this message is and send to that tx
        if let Ok(message) = msg{
                let _ = tx.send(message);
        }

   }

}

