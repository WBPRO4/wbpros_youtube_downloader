/*
Asynchronous(non-blocking) functions that perform things tat might need to be done more than once
*/
use std::path::Path;
use sanitize_filename::sanitize;
use crate::{ParsedArgs, convert_file};
use rusty_ytdl::{Video, VideoInfo, VideoOptions, VideoQuality, VideoSearchOptions};

pub async fn get_video_info(url: &String, args: &ParsedArgs) -> (rusty_ytdl::Video, VideoInfo)
{
  let mut video_options = VideoOptions {
    quality: VideoQuality::LowestAudio,
    filter: VideoSearchOptions::Audio,
    ..Default::default()
  };

    if args.with_video
    {
      video_options.quality = VideoQuality::Highest;
      video_options.filter = VideoSearchOptions::VideoAudio;
    }
    else if args.video_only 
    {
      video_options.quality = VideoQuality::Highest;
      video_options.filter = VideoSearchOptions::Video;
    };

    let video = Video::new_with_options(url, video_options).unwrap();
    let video_info = video.get_basic_info().await.unwrap();
    println!("title: {}\nUpload Date: {}",video_info.video_details.title, video_info.video_details.upload_date);
    return (video, video_info);
}

pub async fn download_video(video: &Video, video_info: &VideoInfo, args: &ParsedArgs)
{
    println!("Downloading Video: {}", video_info.video_details.title);
    // sanitize the title
    let sanitized_title = sanitize(&video_info.video_details.title);
    // do this twice cuz of some weird type conversion issue
    let mut clean_tmp_file_title: String = format!("{}.mp4", sanitized_title);
    // do this thrice cuz command line args
    if args.as_m4a{clean_tmp_file_title = format!("{}.m4a", sanitized_title);}
    // create a path object
    let tmp_file_path: &Path = Path::new(clean_tmp_file_title.as_str());
    // download video
    video.download(tmp_file_path).await.unwrap();
    
    // Yes I know I write if statements like a madman but its 1 thong happening in them I aint wasting 3 line for that
    if args.as_m4a {println!("--as-m4a specified skipping conversion step")}
    else if args.video_only {println!("--video-only specified skipping conversion step")}
    else if args.with_video {println!("--with-video specified skipping conversion step")}
    else{convert_file(&clean_tmp_file_title, &format!("{}.mp3", sanitized_title));}
}