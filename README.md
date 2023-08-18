# **WBPRO's Youtube downloader**
Simple but powerful youtube downloader written in Rust.
Based on the [rusty_ytdl crate](https://crates.io/crates/rusty_ytdl).
I built this as a first project to learn rust so if you know Rust well constructive criticism and pull requests are welcome.
### NOTE: THIS PROJECT REQUIRES AN FFMPEG INSTALLATION TO PERFORM CONVERSION TO MP3 (AS LONG AS ITS IN YOUR PATH IT SHOUD WORK)
## **Usage Guide**
To download a video as mp3 simply invoke the binary with the url
like this:
``` 
    wbpros_youtube_downloader.exe https://youtu.be/dQw4w9WgXcQ
``` 
the program will automatically download the video and convert it to mp3.

To download as mp4 simply specify the ``` --with-video``` switch
``` 
wbpros_youtube_downloader.exe https://youtu.be/dQw4w9WgXcQ --with-video
``` 
To download a list of videos simply put the URLs or VideoIDs in a file like this
``` 
https://youtu.be/dQw4w9WgXcQ
https://youtu.be/QQOkDB2KWCE
https://youtu.be/24E-3B6VE1Q
https://youtu.be/zTCXAEQeog0
https://youtu.be/42iQKuQodW4
``` 
then invoke the downloader with the ``` --file``` switch like this
``` 
wbpros_youtube_downloader.exe --file <the files name> <any other switches>
``` 
the downloader will go down the list and download all the videos
NOTE: The file must be in the current path(fix for this coming later)

## All switches
NOTE: not all switches can be used together
|    Switch    |                           Effect                           |
|:------------:|:----------------------------------------------------------:|
| --file       | reads URLs from specified file instead of the command line |
| --with-video | skips converting video to mp3                              |
| --video-only | downloads video without sound                              |
| --as-m4a     | downloads video as an m4a                                  |
