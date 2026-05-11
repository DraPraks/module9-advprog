# Tutorial A: Event-Driven Architecture - Subscriber

## Answers to Questions

### a. What is AMQP?

AMQP (Advanced Message Queuing Protocol) is an open-source standard protocol used for message-oriented middleware. It defines a way for applications to communicate asynchronously by sending messages through a message broker. AMQP ensures reliable, secure, and interoperable messaging between different systems and applications.

### b. What does `guest:guest@localhost:5672` mean?

- **First `guest`**: This is the **username** used to authenticate with RabbitMQ
- **Second `guest`**: This is the **password** for that user account
- **`localhost:5672`**: This specifies:
  - `localhost`: The address where RabbitMQ is running (your local machine)
  - `5672`: The port number on which RabbitMQ is listening for AMQP connections

So the full connection string `amqp://guest:guest@localhost:5672` means: "Connect to RabbitMQ at localhost on port 5672 using the credentials guest/guest"

## Subscriber Implementation

This subscriber listens to the "user_created" queue and processes `UserCreatedEventMessage` events using a message handler. The connection string uses the default RabbitMQ credentials and connects to a local RabbitMQ instance.

The handler prints received messages to the console with an identifier of the subscriber instance.

---

**Note**: Replace `[YOUR_NPM]` in the subscriber code with your actual NPM (student ID).
