use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }

    fn get_handler_action(&self) -> String {
        "handle_user_created".to_string()
    }
}

fn main() {
    let mut p = CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "1".to_owned(), 
        user_name: "2406353530-Muhammad Adra Prakoso (User1)".to_owned() 
    });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "2".to_owned(), 
        user_name: "2406353530-Muhammad Adra Prakoso (User2)".to_owned() 
    });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "3".to_owned(), 
        user_name: "2406353530-Muhammad Adra Prakoso (User3)".to_owned() 
    });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "4".to_owned(), 
        user_name: "2406353530-Muhammad Adra Prakoso (User4)".to_owned() 
    });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "5".to_owned(), 
        user_name: "2406353530-Muhammad Adra Prakoso (User5)".to_owned() 
    });
}

