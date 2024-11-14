# casino_server
`casino_server` is a simple casino server application written in Rust. It allows multiple clients to connect, place bets, and check their balances. The server handles commands from clients and broadcasts results to all connected clients.

## Features

- Add new players
- Place bets
- Check balance
- Broadcast results to all clients

## Getting Started
To run the server, use the following command:
```sh
cargo run
```

The server will start and listen for connections on localhost:8080.

## Connecting to the Server
You can connect to the server using a simple TCP client like telnet or nc (netcat).

For example, using telnet:
```sh
telnet localhost 8080
```

## Command
Once connected, you can use the following commands:
- balance: Check your current balance.
- bet <amount>: Place a bet with the specified amount.
- exit: Disconnect from server.

## Game
Multiple players can connect to the server at the same time.
When player placed bet, it will be broadcasted to all connected players.
When all players placed bet, the dealer will roll dice for everyone and the result will be broadcasted.

