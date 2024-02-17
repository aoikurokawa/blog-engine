## Introduction

Welcome back to our exploration of the BitTorrent protocol! In [our previous blog](https://www.nxted.co.jp/blog/blog_detail?id=49), we tried to communicate with a tracker. We broke down the components of this request, understanding each element's purpose and how they collectively contribute to successful peer discovery. In this time, we try to handshake with one of the peers from the information that get from the tracker. 

## Handshake

First of all, we have to define a handshake struct. 
`Handshake` struct has 5 attributes according to the [spec](https://www.bittorrent.org/beps/bep_0003.html#peer-protocol).

#### length 
length of the protocol string (BitTorrent protocol) which is 19 (1 byte)

#### protocol
the string BitTorrent protocol (19 bytes)

#### reserved
eight reserved bytes, which are all set to zero (8 bytes)

#### info_hash
sha1 infohash (20 bytes) (NOT the hexadecimal representation, which is 40 bytes long)

#### peer_id
peer id (20 bytes) (you can use 00112233445566778899 for this blog)

```rust
#[derive(Debug, Clone)]
pub struct Handshake {
    pub length: u8,
    pub protocol: Vec<u8>,
    pub reserved: Vec<u8>,
    pub info_hash: Vec<u8>,
    pub peer_id: Vec<u8>,
}
```

To initiate the new Handshake, we are going to create new function. 

```rust
impl Handshake {
    pub fn new(info_hash: &[u8; 20]) -> Self {
        Self {
            length: 19,
            protocol: b"BitTorrent protocol".to_vec(),
            reserved: vec![0; 8],
            info_hash: info_hash.to_vec(),
            peer_id: b"00112233445566778899".to_vec(),
        }
    }

   pub fn bytes(&self) -> Vec<u8> {
     let mut bytes = Vec::with_capacity(68);

     bytes.push(self.length);
     bytes.extend(self.protocol.clone());
     bytes.extend(self.reserved.clone());
     bytes.extend(self.info_hash.clone());
     bytes.extend(self.peer_id.clone());

     bytes
   }
}
```


## Command

To make life easier, we are going to do step by step. 

1. Define the handshake command

We are going to take 2 arguments `torrent`, `peer`. `torrent` is the path of torrent file and `peer` is address of the peer that we are try to connect and handshake with. 

```rust
#[derive(Subcommand)]
enum Commands {
    Handshake {
        torrent: PathBuf,
        peer: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  match args.command {
    Commands::Handshake { torrent, peer } => {}
  }
}

```

2. Read the torrent file

Inside handshake comamnd, we going to read the torrent file and deseriaze into actual `Torrent` struct to get the information. 

```rust
let dot_torrent = std::fs::read(torrent).context("read torrent file")?;
let t: Torrent = serde_bencode::from_bytes(&dot_torrent).context("parse torrent file")?;
```

3. Calculate info hash

In order to handshake with the peer, we have to send the info hash. 

```rust
let info_hash = t.info_hash();
```

Inside `info_hash` function, we convert the `info` to bytes, then hash them by `Sha1`. 

```rust
impl Torrent {
  pub fn info_hash(&self) -> [u8; 20] {
    let info_encoded = serde_bencode::to_bytes(&self.info).expect("re-encode info section");
    let mut hasher = Sha1::new();
    hasher.update(&info_encoded);
    hasher
      .finalize()
      .try_into()
      .expect("GenericArray<[u8; 20]>")
  }
}
```

3. Connect with the peer

From the argument of peer info, we try to connect with it. By [tokio](https://docs.rs/tokio/latest/tokio/), connect asynchronously. 

```rust
let mut peer = tokio::net::TcpStream::connect(peer)
  .await
  .context("connect to peer")?;
```

4. Handshake

```rust
let handshake = Handshake::new(info_hash);
{
  let mut handshake_bytes = handshake.bytes();
  stream.write_all(&mut handshake_bytes).await?;

  stream.read_exact(&mut handshake_bytes).await?;
}
```

5. Get peer id from the peer

```rust
assert_eq!(handshake.length, 19);
assert_eq!(handshake.bittorent_protocol, *b"BitTorrent protocol");

println!("Peer ID: {}", hex::encode(handshake.peer_id));
```

When we hit the command like below, we would get the peer id. 

```bash
./build.sh handshake sample.torrent 178.62.82.89:51470
```

## Conclusion
We explored discovering peers and tried to handshake with one of the peer, then we successfully got the peer id.
In the next blog, we will download the piece of file from the peer. 
Thank you for reading.

## Resources
- [The BitTorrent Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html#peer-protocol)

