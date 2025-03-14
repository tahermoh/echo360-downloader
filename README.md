# Echo360 Lecture Video Downloader
<p align="center">
    <img src="res/echo360.png" alt="Echo360 Logo" width="25%">
</p>

Pretty much what is says in the title. A GUI application that allows you to log in to your Echo360 account, view your courses, and selected videos to quickly download, because it's insanely slow to do it on their website.

## Getting Started
### Dependencies
So far the the downloader requires you to be running your own Webdriver instance on `http://localhost:9515`

If you're building locally, you're going to require a `Rust` and `cargo` installation.

### Installation
#### Local Compilation
`git clone git@github.com:tahermoh/echo360-downloader.git`

`cd echo360-downloader`

`cargo build --release`

### Running
`./target/release/echo360-downloader`

### Optional Config
If you already have valid cookies to use and know your local echo360 domain, you can add them to a `.env` file at the root of this project, for example:

```
PLAY_SESSION_COOKIE="PLAY_SESSION=..."
DOMAIN="https://echo360.net.au"
```
This will avoid the need for login in on application startup.


