## Introduction
In [our last exploration](https://www.nxted.co.jp/hp/blog/blog_detail?id=62), we delved into downloading a piece.
We will try to download a whole file from peers in this time!
If you want to read a whole series, I recommend reading from [here](https://www.nxted.co.jp/blog/blog_detail?id=40).

## CLI

```rs
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download {
        #[arg(short)]
        output: PathBuf,
        torrent: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Download { output, torrent } => {
            let torrent: Torrent = Torrent::read(torrent).await?;
            torrent.print_tree();
            // torrent.download_all_to_file(output).await?;
            let files = download_all(&torrent).await?;
            tokio::fs::write(
                output,
                files.into_iter().next().expect("always one file").bytes(),
            )
            .await?;
        }
    }

    Ok(())
}
```


## Resources
- https://without.boats/
- [Async: What is blocking?](https://ryhl.io/blog/async-what-is-blocking/)
- [Green Threads in Rust](https://stanford-cs242.github.io/f17/assets/projects/2017/kedero.pdf)
- []()
