mod utils;
mod structs;
mod async_utils;
use structs::ParsedArgs;
use std::{env, process::exit};
use async_utils::{get_video_info, download_video};
use utils::{read_download_list, parse_args, convert_file};

#[tokio::main]
async fn main() {
    println!("WBPRO's Youtube Downloader\nCopyright 2023 All rights reserved.");

    let args: ParsedArgs = parse_args(env::args().collect());

    if args.filemode
    {
      // File hosts.txt must exist in the current path
      if let Ok(lines) = read_download_list(args.filename.to_string()) 
      {
        // Consumes the iterator, returns an (Optional) String
        for line in lines 
        {
          if let Ok(url) = line 
          {
            let (video, video_info) = get_video_info(&url, &args).await;
            download_video(&video, &video_info, &args).await;
            println!("Moving onto next link in list...")
          }
        }
        println!("Done!\nExiting!");
        exit(0) 
      }
    }
    else
    {
      let (video, video_info) = get_video_info(&args.url, &args).await;
      download_video(&video, &video_info, &args).await;
      println!("Done!")
    }

}