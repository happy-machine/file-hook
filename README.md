##file-hook

Very lightweight Rust program for notifications for files / folders.

####What is file-hook?

We wrote file-hook to containerise and use in our kubernetes orchestrated pipelines.

**wait mode** - file-hook will wait indefinetely for a first file or folder to arrive.

When it recieves a file or folder it will start a timer and wait for n seconds (60 by default) before exiting with a zero exit code.

**default mode** - file-hook will wait indefinetely and send a request to a specified endpoint for every file or folder recieved, this will contain a body with either a key value for 'file' or 'folder' and the corresponding name.

#### How to use

To run:
`cargo run` or `cargo build --release`, then `./file-hook`

To run in 'wait' mode
`cargo run --wait` or `cargo build --release`, then `./file-hook --wait`

#### Environment variables

**ENDPOINT**="https://apimocha.com/test/example"
Endpoint to sent REST request to on file/folder arrival.

**REQUEST_SENSOR_PATH**="/Users/me/files-to-watch"
Folder to watch for new files/folders.

**FILE_SENSOR_DELAY**=20
Seconds to wait in wait mode.
