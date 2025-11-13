use std::io::{self,Write};
use std::process::Command;

fn main (){
    println!("Youtube Audio Downloader");

    println!("masukan url youtube lu : ");
    io::stdout().flush().unwrap();

    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    let url = url.trim();

    if url.is_empty(){
        println!("lu dongo nyuruh gw download, tapi ga ngasih url nya?");
        return;
    }

    println!("sabar yaa,audionya lagi di download...");

    let status = Command::new("yt-dlp")
    .args([
        "--extract-audio",
        "--audio-format","mp3",
        "--output", "%(title)s.%(ext)s",
        url,
    ])

    .status();

    match status {
        Ok(s) if s.success() => println!("yeeyyyy.. audio lu udah berhasil di download"),
        Ok(_) => println!("yt-dlp returned an error"),
        Err(e) => println!("gagal menjalankan yt-dlp: {}",e ),
    }
}
