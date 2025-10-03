use std::{fs::File, io::BufReader, path::Path, process};

pub fn get_video_duration(file_path: &String) -> u64 {
    let f = File::open(file_path).expect("Failed to open file");
    let size = f.metadata().unwrap().len();
    let reader = BufReader::new(f);

    let mp4 = mp4::Mp4Reader::read_header(reader, size).unwrap();
    mp4.duration().as_secs()
}

pub fn shorten_video(file_path: &String, output_name: &String, from_time: &f64, to_time: &f64) {
    // 获取原始文件的目录路径
    let original_path = Path::new(file_path);
    let parent_dir = original_path.parent().unwrap_or_else(|| Path::new("."));

    // 构建输出文件的完整路径
    let output_path = parent_dir.join(output_name);
    let output_path_str = output_path.to_str().expect("Invalid output path");

    process::Command::new("ffmpeg.exe")
        .args([
            "-i",
            file_path,
            "-ss",
            seconds_to_time_string(*from_time as u64).as_str(),
            "-to",
            seconds_to_time_string(*to_time as u64).as_str(),
            output_path_str,
        ])
        .spawn()
        .unwrap();
}

pub fn seconds_to_time_string(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
