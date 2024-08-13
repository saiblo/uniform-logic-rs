//! # Using Uniform Protocol to Write Game Logic
//!
//! The Uniform Protocol has 4 basic structs: [`InitInput`], [`RoundOutput`], [`RoundInput`], 
//! [`FinishOutput`]. The suffix "Input" means the message is sent from Judger(Agent) to Logic, 
//! while "Output" means the message is sent from Logic to Judger(Agent).
//!
//! You also need to design your custom protocol and define your structs to specify the general 
//! types defined in these basic structs.

mod protocol;
mod serialize_map;

pub use protocol::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{to_string, Error};
pub use serialize_map::SerializableMap;
use std::{collections::HashMap, io::stdin};

/// Receive init message from Judger.
pub fn receive_init_message<InitData>() -> Result<InitData, Error>
where
    InitData: DeserializeOwned,
{
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).unwrap();
    serde_json::from_str::<InitInput<InitData>>(&input_str)
        .map(|init_input| init_input.initdata)
}

/// Send round message to Judger.
pub struct RoundMessageSender<Request, Display> {
    content: HashMap<String, Request>,
    display: Option<Display>,
}

impl<Request, Display> RoundMessageSender<Request, Display>
where
    Request: Serialize,
    Display: Serialize,
{
    /// Create a new `RoundMessageSender`.
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
            display: None,
        }
    }

    /// Prepare to send a request to an agent.
    pub fn send_agent(&mut self, name: String, request: Request) {
        self.content.insert(name, request);
    }

    /// Prepare to send a display message to web player.
    pub fn send_display(&mut self, display: Display) {
        self.display = Some(display);
    }

    /// Send the round message.
    pub fn end(self) -> Result<(), Error> {
        let round_output = RoundOutput {
            content: self.content.into(),
            display: self.display.unwrap(),
        };
        println!("{}", to_string(&round_output)?);
        Ok(())
    }
}

/// Receive round message from Judger.
pub fn recieve_round_message<Response>() -> Result<HashMap<String, AgentMessage<Response>>, Error>
where
    Response: DeserializeOwned,
{
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).unwrap();
    serde_json::from_str::<RoundInput<Response>>(&input_str)
        .map(|round_input| round_input.log.0)
}

/// Send finish message to Judger.
pub struct FinishMessageSender<Display> {
    content: HashMap<String, FinishMessage>,
    display: Option<Display>,
}

impl<Display> FinishMessageSender<Display>
where
    Display: Serialize,
{
    /// Create a new `FinishMessageSender`.
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
            display: None,
        }
    }

    /// Prepare to send a finish message to an agent.
    pub fn send_agent(&mut self, name: String, score: f32, state: String) {
        self.content.insert(name, FinishMessage { score, state });
    }

    /// Prepare to send a display message to web player.
    pub fn send_display(&mut self, display: Display) {
        self.display = Some(display);
    }

    /// Send the finish message.
    pub fn end(self) -> Result<(), Error> {
        let finish_output = FinishOutput {
            content: self.content.into(),
            display: self.display.unwrap(),
        };
        println!("{}", to_string(&finish_output)?);
        Ok(())
    }
}
