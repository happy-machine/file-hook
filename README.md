# file-hook

Very lightweight Rust program for notifications for files / folders.

### What is file-hook?

file-hook will wait indefinetely and send a request to a specified endpoint for every file or folder recieved, this will contain a body with either a key value for 'file' or 'folder' and the corresponding name. It's fast and lightweight so you can leave it running as a background task or in a persistent container in confidence.

### How to use

To run:
`cargo run` or `cargo build --release`, then `./file-hook`
<br/>

### Environment variables

**ENDPOINT**="https://apimocha.com/test/example"  
Endpoint to sent REST request to on file/folder arrival.

**REQUEST_SENSOR_PATH**="/Users/me/files-to-watch"  
Folder to watch for new files/folders.
