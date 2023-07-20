# Socket Programing in Rust

This is a TCP server client program written in Rust. The server is a simple echo server that echoes back whatever the client sends to it. The client sends a message to the server and waits for the server to echo back the message. The client then prints the message to the console.

## How to run
```bash
# Run the server
cargo run -p tcpserver

# Run the client (in a different terminal) [don't close the terminal running the server]
cargo run -p tcpclient
```
## The TCP Layer

The provides reliable end-to-end communication. While the application layer Transport layer
deals with messages that have specific semantics (such as sending a GET request to get shipment
details), the transport protocols deal with sending and receiving raw bytes. (Note: all application
layer protocol messages eventually get converted into raw bytes for transmission by the transport
layer). TCP and UDP are the two main protocols used in this layer, with QUIC (Quick UDP
Internet Connection) also being a recent entrant. TCP is a connection-oriented protocol that
allows data to be partitioned for transmission and reassembled in a reliable manner at the
receiving end. UDP is a connectionless protocol and does not provide guarantees on delivery,
unlike TCP. UDP is consequently faster and suitable for certain class of applications eg DNS
lookups, voice or video applications.In this book, we will focus on the TCP protocol for transport
layer.