function setup_websocket(url){
    let ws_url = "ws://" + url;
    console.info("trying to connect to ", url);
    var socket = new WebSocket(ws_url);
    socket.onmessage = (event)=> {
        console.log( JSON.parse(event.data) )
    }

    //console.warn("not setting up a WebSocket");

}
