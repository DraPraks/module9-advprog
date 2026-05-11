# Tutorial A: Event-Driven Architecture - Publisher

## Answers to Questions

### a. How much data will the publisher send to the message broker in one run?

The publisher sends **5 messages** to the message broker in one run. Each `p.publish_event()` call sends one message containing a `UserCreatedEventMessage` with:
- A unique `user_id` (from 1 to 5)
- A unique `user_name` (with the format "YOUR_NPM-UserN")

So a total of 5 events are published in a single execution of the publisher program.

### b. Why is the URL the same in both subscriber and publisher?

The URL `"amqp://guest:guest@localhost:5672"` is identical in both the publisher and subscriber because:

1. **They must connect to the same message broker**: The publisher and subscriber need to communicate through a single RabbitMQ instance
2. **Same credentials**: Both use the default RabbitMQ guest account for authentication
3. **Same location**: Both connect to the same RabbitMQ server running on localhost (your machine) on port 5672
4. **Different roles, same infrastructure**: The difference between publisher and subscriber is not in the connection details but in what they do:
   - The **publisher** creates events and sends them to the message broker
   - The **subscriber** receives events from the message broker and processes them

This separation allows the publisher and subscriber to operate independently while maintaining reliable communication through the central message broker.

## Publisher Implementation

This publisher creates and sends 5 user creation events to the "user_created" queue on RabbitMQ. Each event contains a user ID and user name, simulating new users being registered in the system.

---

**Note**: Replace `[YOUR_NPM]` in the publisher code with your actual NPM (student ID).
