use crate::state::{AppState};
use warp:: Filter;
use crate::client::{spawn_client};

pub async fn start(){

    let state = AppState::new();
    let state_filter = warp::any().map(move || state.clone());

    let filter = warp::path("ws")
        .and(warp::ws())
        .and(state_filter)
        .map(|ws: warp::ws::Ws, state|{
            ws.on_upgrade(move |socket|async move{
                spawn_client(socket, state).await;
            })
 
        });
   
    println!("server starting in 127.0.0.1:3030");

    warp::serve(filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

