# ACKNOWLDGEMENT

This project is under development. It will not be usable for productiion and also
development until alpha release. Contributions are welcome and available on
the github repo located in <https://github.com/m-epasta/synq/>.
ALSO, the syntax is not fully covered, the syntax were replaced with a script
that is incorrect. The initial release does not have the complete correct
syntax.

# SYNQ - A data transporting framework above UDP and QUIC

synQ is an alternative of gRPC. It aims to be blazingly fast and with near zero overhead.
It achieves its 2 goals by using modern protocols(UDP & QUIC) with a totally new way of handshaking.

## DESIGN

The library is on 2 distincted but required part (2 different code): the client and the sender.

### sender

The sender should create a topic (e.g: Block) and then start an UDP connection, of course, you can
modify some behaviours but the default is not really more complicated then that. You can decide else to expose the port locally or globally.

### client

The client check a specified port. It then check the exposed content, if it is a topic, where it is
found to be interested in, he start a connection and then send the hash of the codec. The sender
sender then check if the given hash is correct and then exchange ALL the required data and CLOSES
the socket. The client then has to deal with this data. If the given data is corrupted or false you
can target the closed socket with a special byte indicating data corruption, if the sender does not
respond back, the sender will be shadowed by others instances by an other magi byte than the sender
can not access.

# LICENSE

The project is licensed under MIT license.
