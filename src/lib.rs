extern crate reqwest;

use std::{fs::File, io::Cursor};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn load_media_file_async(url: &str, filename: &str, path: &str) -> Result<File> {
    let resp = reqwest::get(url).await?;

    let ext = resp.headers()["content-values"]
        .to_str()?
        .split("/")
        .skip(1)
        .next()
        .unwrap();

    let mut file = File::create(format!("{}/{}.{}", path, filename, ext))?;
    let mut content = Cursor::new(resp.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(file)
}

pub fn load_media_file(url: &str, filename: &str, path: &str) -> Result<File> {
    let mut resp = reqwest::blocking::get(url)?;

    let ext = resp.headers()["content-type"]
        .to_str()?
        .split("/")
        .skip(1)
        .next()
        .unwrap();

    let mut file = File::create(format!("{}/{}.{}", path, filename, ext))?;
    resp.copy_to(&mut file)?;
    Ok(file)
}

#[test]
fn test() {
    load_media_file(
        "https://rr4---sn-u5uuxaxjvhg0-31oe.googlevideo.com/videoplayback?expire=1678419421&ei=fVEKZLvuEZGo7QTk0bpY&ip=188.130.155.153&id=o-ABVqVV-prj0vJy8CqcQsaGGuAytNq4dKVuDCKObjXyS3&itag=140&source=youtube&requiressl=yes&mh=2P&mm=31%2C29&mn=sn-u5uuxaxjvhg0-31oe%2Csn-gvnuxaxjvh-bvwz&ms=au%2Crdu&mv=m&mvi=4&pcm2cms=yes&pl=24&gcr=ru&initcwndbps=1353750&vprv=1&mime=audio%2Fmp4&ns=uhKH6SivvVAGRsM2dNhmPLAL&gir=yes&clen=7687478&dur=474.964&lmt=1577687743670005&mt=1678397346&fvip=12&keepalive=yes&fexp=24007246&c=WEB&txp=5531432&n=-Ko6i5v2UDv_qb9yr&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cgcr%2Cvprv%2Cmime%2Cns%2Cgir%2Cclen%2Cdur%2Clmt&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpcm2cms%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIhAJwvhxQ0yI1DsIMHRek6cJvpjdR4R0iTFZWTSqoSgRhtAiBzK9CMSaoA6mWQbptrzofFcQPi0BZ_NHmzt8GeuLLSjQ%3D%3D&sig=AOq0QJ8wRgIhALg8IMQsfp7wHnt0Jeu32Dgq2EMXUmOFsrnp2DZFJLpTAiEAmw_6K3KweMBO3WZW3aL1vYG-OxNpXpkSrZ2LrLJCKVM=",
        "testing",
        "/home/paranid5/PROGRAMMING/Rust/media_file_downloader"
    ).unwrap();
}
