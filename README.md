# Media File Downloader

Simple library to download video/audio files through HTTP connection.

# Example

```Rust
fn main() {
    load_media_file(
        "your-url",
        "your_filename",
        "/path/to/your/folder"
    ).unwrap();
}```