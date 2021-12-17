use futures_util::{SinkExt, StreamExt,stream::SplitSink};
use std::{sync::{Arc, Mutex},collections::{HashSet,HashMap}};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{WebSocketStream};
use tokio_tungstenite::tungstenite::{Message, Result};
use chrono::{DateTime, Utc};
//.keys().cloned().collect::<Vec<_>>();

pub struct Board{
    room_id:String,
    owner_user_id:String,
    //Those granted permissions by the owner
    users_with_permission: HashSet<String>
}

pub struct User{
    avatar_url:String,
    display_name:String,
    user_name:String,
    last_online:DateTime<Utc>,
    muted:bool,
    deaf:bool,
    ip:String,
    bio:String,
    banner_url:String
}

pub struct Room{
    room_id:String,
    owner_user_id:String,
    muted:HashSet<String>,
    voice_server_id:String,
    deaf:HashSet<String>,
    user_ids:HashSet<String>,
    public:bool,
    chat_mode:String,
    auto_speaker:bool,
}

//IoTServerConnectionId -> Permissions for the connection(represented as the board)
//Read the docs about the Board concept
pub type IoTServerConnections = Arc<Mutex<HashMap<String,Board>>>;

//user id -> write connection.
//broadcasting requires you to acquire the lock of the mutex
//to access peer connections.
pub type PeerMap =  Arc<Mutex<HashMap<String,
    SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>>>>;

//current connected and authed users
pub type ActiveUsers = Arc<Mutex<HashMap<String,User>>>;

//room collection
pub type ActiveRooms = Arc<Mutex<HashMap<String,Room>>>;