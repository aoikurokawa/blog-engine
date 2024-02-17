## Introduction

Welcome back to our exploration of the BitTorrent protocol! In [our previous blog](https://www.nxted.co.jp/blog/blog_detail?id=49), we tried to communicate with a tracker. We broke down the components of this request, understanding each element's purpose and how they collectively contribute to successful peer discovery. In this time, we try to handshake with one of the peers from the information that get from the tracker. 

## Handshake

First of all, we have to define a handshake struct. 

```rust

```


## Command

To make life easier, we are going to do step by step. 

1. Define the handshake command

We are going to take 2 arguments `torrent`, `peer`. `torrent` is the path of torrent file and `peer` is address of the peer that we are try to connect and handshake with. 

```rust
Commands::Handshake { torrent, peer } => {

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
let mut handshake = Handshake::new(info_hash, *b"00112233445566778899");
{
  let handshake_bytes =
  &mut handshake as *mut Handshake as *mut [u8; std::mem::size_of::<Handshake>()];
  // Safety: Handshake is POD with repr(c)
  let handshake_bytes: &mut [u8; std::mem::size_of::<Handshake>()] =
  unsafe { &mut *handshake_bytes };
  peer.write_all(handshake_bytes)
    .await
    .context("write handshake")?;

  peer.read_exact(handshake_bytes)
    .await
    .context("read handshake")?;
}

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






