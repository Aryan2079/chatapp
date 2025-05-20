use warp::Filter;

pub async fn start(){
    let filter = warp::path("ws")
        .and(warp::ws())
        .map(|ws:warp::ws::Ws|{
            ws.on_upgrade(|_websocket:warp::filters::ws::WebSocket|async move{
                // handle_connection(ws);
                println!("connection is established");
            })
 
        });
   
    println!("server starting in 127.0.0.1:3030");

    warp::serve(filter)
        .run(([127, 0, 0, 1], 3030))
        .await;

}

