use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Channel {
    Local,
    Public,
    AIPrivate,          // Freq 144.7
    AlienHivemind,
    Binary,
    Command,            // Freq 135.3
    CentCom,            // Freq 133.7
    Engineering,        // Freq 135.7
    Medical,            // Freq 135.5
    Science,            // Freq 135.1
    Security,           // Freq 135.9
    Service,            // Freq 134.9
    Supply,             // Freq 134.7
    Syndicate,          // Freq 121.3
    OOC,                // Out of character
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    ClientConnected { client_id: usize, username: String },
    ClientDisconnected { client_id: usize },
    ChatMessage { client_id: usize, channel: Channel, message: String },
}
