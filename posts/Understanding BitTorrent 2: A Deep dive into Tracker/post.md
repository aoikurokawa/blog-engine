## Introduction 
BitTorrent is one of the pioneering and most popular peer-to-peer (P2P) file sharing protocols. In the [last blog](https://www.nxted.co.jp/blog/blog_detail?id=40), I wrote about [Bencode](https://en.wikipedia.org/wiki/Bencode). Bencode is the encoding method used by BitTorrent for storing and transmitting loosely structured data. It's a binary format that serializes data types like integers, strings, lists, and dictionaries (key-value pairs). In this blog, I will parse a torrent file and then try to interact with a tracker.


## Bittorrent file
A .torrent file describes the contents of a torrentable file and information for connecting to a tracker. For example, Debian’s .torrent file looks like this:

```
d
  8:announce
    41:http://bttracker.debian.org:6969/announceDebian’s .torrent file looks like this:
  7:comment
    35:"Debian CD from cdimage.debian.org"
  13:creation date
    i1573903810e
  4:info
    d
      6:length
        i351272960e
      4:name
        31:debian-10.2.0-amd64-netinst.iso
      12:piece length
        i262144e
      6:pieces
        26800:�����PS�^�� (binary blob of the hashes of each piece)
    e
e

```

First of all, we need to define the struct of Torrent. According to the [spec](https://www.bittorrent.org/beps/bep_0003.html), there are `announce` attribute and some information in `info` attribute. 

> metainfo files
>
> Metainfo files (also known as .torrent files) are bencoded dictionaries with the following keys:
>
> announce
>    The URL of the tracker.
> info
>    This maps to a dictionary, with keys described below.
>
> All strings in a .torrent file that contains text must be UTF-8 encoded.

```rs
// torrent.rs

use serde::{Deserialize, Deserializer, Serialize};

/// A Metainfo files (also known as .torrent files).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Torrent {
    /// The URL of the tracker
    pub announce: String,

    pub info: Info,
}
```

Inside `info` attribute, there are some attributes more. So I need to define struct of `Info`.

> info dictionary
>
> The name key maps to a UTF-8 encoded string which is the suggested name to save the file (or directory) as. It is purely advisory.
> 
> piece length maps to the number of bytes in each piece the file is split into. For the purposes of transfer, files are split into fixed-size pieces which are all the same length except for possibly the last one which may be truncated. piece length is almost always a power of two, most commonly 2 18 = 256 K (BitTorrent prior to version 3.2 uses 2 20 = 1 M as default).
> 
> pieces maps to a string whose length is a multiple of 20. It is to be subdivided into strings of length 20, each of which is the SHA1 hash of the piece at the corresponding index.
> 
> There is also a key length or a key files, but not both or neither. If length is present then the download represents a single file, otherwise it represents a set of files which go in a directory structure.
> 
> In the single file case, length maps to the length of the file in bytes.
> 
> For the purposes of the other keys, the multi-file case is treated as only having a single file by concatenating the files in the order they appear in the files list. The files list is the value files maps to, and is a list of dictionaries containing the following keys:
> 
> length - The length of the file, in bytes.
> 
> path - A list of UTF-8 encoded strings corresponding to subdirectory names, the last of which is the actual file name (a zero length list is an error case).
> 
> In the single file case, the name key is the name of a file, in the muliple file case, it's the name of a directory.


```rs
// torrent.rs

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    /// The suggested name to save the file (or directory) as. It is purely advisory.
    ///
    /// In the single file case, the name key is the name of a file, in the multiple file case,
    /// it's the nmae of a directory.
    pub name: String,

    /// The number of bytes in each piece the file is split into.
    ///
    /// For the purposes of transfer, files are split into fixed-size pieces which are all the same
    /// length except for possibly the last one which may be truncated. piece length is almost
    /// always a power of two, most commonly 2^18 = 256K
    /// (BitTorrent prior to version 3.2 uses 2^20 = 1 M as default).
    #[serde(rename = "piece length")]
    pub plength: usize,

    /// Each of which is the SHA1 hash of the piece at the corresponding index.
    pub pieces: Hashes,

    #[serde(flatten)]
    pub keys: Keys,
}
```

`pieces` are a bit tricky to serialize and deserialize.  

```rs
// torrent.rs

#[derive(Debug, Clone)]
pub struct Hashes(pub Vec<[u8; 20]>);
struct HashesVisitor;

impl<'de> Visitor<'de> for HashesVisitor {
    type Value = Hashes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a byte string whose length is multiple of 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!("length is {}", v.len())));
        }

        Ok(Hashes(
            v.chunks_exact(20)
                .map(|slice_20| slice_20.try_into().expect("guaranteed to be length 20"))
                .collect(),
        ))
    }
}

impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> Result<Hashes, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashesVisitor)
    }
}

impl Serialize for Hashes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let single_file = self.0.concat();
        serializer.serialize_bytes(&single_file)
    }
}
```

Because of the spec of keys, we need to use [untagged](https://serde.rs/variant-attrs.html#untagged) attribute. 

> There is also a key length or a key files, but not both or neither. If length is present then the download represents a single file, otherwise it represents a set of files which go in a directory structure.


```rs
/// There is also a key `length` or a key `files`, but not both or neither.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Keys {
    /// If `length` is present then the download represents a single file,
    SingleFile {
        /// The length of the file in bytes.
        length: usize,
    },

    /// Otherwise it represents a set of files which go in a directory structure.
    /// For the purposes of the other keys in `Info`, the multi-file case is treated as only having
    /// a single file by concatenating the files in the order they appear in the files list.
    MultiFile {
        /// The files list is the value files maps to, and is a list of dictionaries containing the
        /// following keys:
        files: Vec<File>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    /// The length of the file, in bytes
    pub length: usize,

    /// Subdirectory names, the last of which is the actual file name
    /// (a zero length list is an error case).
    pub path: Vec<String>,
}
```

I use the code that we explored in the last blog. To handle command line easily, I use [clap crate](https://docs.rs/clap/latest/clap/). 

```rs
// main.rs/// A Metainfo files (also known as .torrent files).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Torrent {
    /// The URL of the tracker
    pub announce: String,

    pub info: Info,
}
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Decode {
        value: String,
    },
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  match args.command {
    Commands::Decode { value } => {
      let v = decode_bencode_value(&value).0;
      println!("{v}");
    }
  }

  Ok(())
}

```




## References
- https://news.ycombinator.com/item?id=37941075
- https://blog.jse.li/posts/torrent/
