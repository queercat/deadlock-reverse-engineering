# words
im guessing the UDP protocol has a kind of handshake that registers an identifier with a session, i'm betting that's what frame 7 in small_conversations is. if so i bet the server immediately responds with an "ack", investigate this, potentially explore preventing that message from being sent and seeing if server keeps requesting more updates.

do messages have idempotency guarantees? what happens with message replay.

are there any values that grow or shrink with the size of the datagram? e.g. message size?

i should make a tool that is able to sync video footage, overlay keyboard input, overlay timestamps with generating wireshark recordings.

i don't think there is any other way to do this.

# quesitons
what kind of information do they need to transmit?
- position
- rotation
- view matrix
- using ability?
- is shooting?
- velocity?
- what item we're using?
- buying / selling an item? (is this maybe TCP?)
