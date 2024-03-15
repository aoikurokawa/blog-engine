## Introduction

In [our last exploration](https://www.nxted.co.jp/blog/blog_detail?id=55), we delved into discovering peers and tried to handshake with one of the peer, then we successfully got the peer id.


## Flow

Before diving into the section of downloading a piece, we want to check the flow.
To download a piece, our program need to send [peer messages](https://www.bittorrent.org/beps/bep_0003.html#peer-messages) to a peer. The overall flow look like this:

1. Read the torrent file to get the tracker file. 

check out [here](https://www.nxted.co.jp/blog/blog_detail?id=48)

2. Perform the tracker GET request to get a list of peers

check out [here](https://www.nxted.co.jp/blog/blog_detail?id=49)

3. Establish TCP connection with a peer, and perform a handshake

check out [here](https://www.nxted.co.jp/blog/blog_detail?id=55)

4. Exchange multiple peer messages to download the file

later in this blog...


## Peer Messages

Peer messages consists of a message length prefix (4 bytes), message id (1 byte) and a payload (variable size). Here are the peer messages you'll need to exchange once the handshake is complete.

1. Wait for a `bitfield` message from the peer indicating which pieces it has

- The message id for this message type is `5`.

2. Send an `interested` message

- The message id for `interested` is `2`.
- The payload for this message is empty.

3. Wait until you receive an `unchoke` message back

- The message id for `unchoke` is `1`.
- The payload for this message is empty.

4. Break the piece into blocks of 16 kiB (16 * 1024 bytes) and send a `request` message for each block

- The message id for `request` is `6`.
- The payload for this message consists of:

    - `index`: the zero-based piece index
    - `begin`: the zero-based byte offset within the piece
