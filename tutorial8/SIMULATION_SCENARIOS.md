# Tutorial A: Extended Simulation Guide

## Scenario 1: Basic Publishing and Subscribing

Follow the Testing Guide to:
1. Run subscriber: `cargo run` (in subscriber folder)
2. Run publisher: `cargo run` (in publisher folder)
3. Observe 5 messages printed on subscriber console

---

## Scenario 2: Slow Subscriber Simulation

### Step 1: Enable slow processing
Edit `subscriber/src/main.rs` and uncomment this line:
```rust
thread::sleep(ten_millis);  // Uncomment this line
```

This makes each message processing take ~1 second.

### Step 2: Increase message volume
Edit `publisher/src/main.rs` and add 5 more `p.publish_event()` calls (copy the existing 5 events):
```rust
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "6".to_owned(), 
        user_name: "YOUR_NPM-User6".to_owned() 
    });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "7".to_owned(), 
        user_name: "YOUR_NPM-User7".to_owned() 
    });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "8".to_owned(), 
        user_name: "YOUR_NPM-User8".to_owned() 
    });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "9".to_owned(), 
        user_name: "YOUR_NPM-User9".to_owned() 
    });
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage { 
        user_id: "10".to_owned(), 
        user_name: "YOUR_NPM-User10".to_owned() 
    });
```

### Step 3: Rebuild and Test
```bash
# In publisher directory
cargo build

# In subscriber directory
cargo build
```

### Step 4: Run the Slow Subscriber Test
1. Run subscriber: `cargo run`
2. Run publisher: `cargo run`
3. **Observe the RabbitMQ Dashboard** (http://localhost:15672):
   - Navigate to "Queues and Streams" → "user_created" queue
   - Watch the message count spike
   - Messages are consumed slowly (1 second per message) creating a queue backlog
   - Take a screenshot of the queue graph showing the spike

### What You'll See:
- Publisher quickly sends all 10 messages (almost instantly)
- Subscriber processes them slowly (1 per second)
- This creates a visible queue in RabbitMQ
- The queue gradually empties as the subscriber processes messages

---

## Scenario 3: Multiple Concurrent Subscribers

This demonstrates how multiple subscribers solve the bottleneck.

### Step 1: Run Multiple Subscribers
Open **3 separate terminal windows** and in each run:
```bash
cd c:\Users\prako\OneDrive\Documents\Projects\module9-advprog\tutorial8\subscriber
cargo run
```

### Step 2: Publish Messages
In another terminal:
```bash
cd c:\Users\prako\OneDrive\Documents\Projects\module9-advprog\tutorial8\publisher
cargo run
```

You can run this multiple times to simulate repeated message publishing.

### Step 3: Observe Load Distribution
- Each subscriber window shows messages it's processing
- Work is distributed across all 3 subscribers
- Some might process "User1, User4, User7" while others process different users
- The queue clears much faster than with a single subscriber

### What You'll See:
- Messages are load-balanced across subscribers
- Total processing time is reduced by ~3x
- The RabbitMQ queue stays smaller because multiple subscribers drain it simultaneously
- Take a screenshot showing multiple subscribers running and the distributed processing

### Why This Works:
- RabbitMQ distributes messages round-robin across connected subscribers
- Multiple consumers can process messages in parallel
- This is the power of event-driven architecture - horizontal scaling

---

## Expected Observations

### Single Slow Subscriber (10 messages):
- Queue spike to ~9-10 messages
- Takes ~10 seconds to clear
- Heavy queue backlog visible on dashboard

### Three Concurrent Slow Subscribers (10 messages):
- Queue spike is much smaller (typically 1-3 messages)
- Clears in ~3-4 seconds
- Much smoother processing

---

## Conclusion

This demonstrates key event-driven architecture benefits:
1. **Decoupling**: Publisher doesn't wait for subscriber
2. **Scalability**: Add more subscribers to handle more load
3. **Resilience**: If one subscriber crashes, others keep processing
4. **Responsiveness**: Publisher returns immediately; processing happens asynchronously

The message broker handles distribution and ensures no messages are lost!
