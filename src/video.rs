use std::{fs::File, io::BufReader, path::Path, process};

/// Get the duration of a video file in seconds
pub fn get_video_duration(file_path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);

    let mp4 = mp4::Mp4Reader::read_header(reader, size)?;
    Ok(mp4.duration().as_secs())
}

/// Shorten a video file by extracting a specific time range
pub fn shorten_video(
    file_path: &str,
    output_name: &str,
    start_time: f64,
    end_time: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the directory path of the original file
    let original_path = Path::new(file_path);
    let parent_dir = original_path.parent().unwrap_or_else(|| Path::new("."));

    // Build the full output path
    let output_path = parent_dir.join(output_name);
    let output_path_str = output_path
        .to_str()
        .ok_or("Invalid output path: contains invalid UTF-8")?;

    let start_time_str = seconds_to_time_string(start_time as u64);
    let end_time_str = seconds_to_time_string(end_time as u64);

    let status = process::Command::new("ffmpeg.exe")
        .args([
            "-i",
            file_path,
            "-ss",
            &start_time_str,
            "-to",
            &end_time_str,
            output_path_str,
        ])
        .status()?;

    if !status.success() {
        return Err(format!("FFmpeg process failed with exit code: {}", status).into());
    }

    Ok(())
}

/// Convert seconds to a time string in HH:MM:SS format
pub fn seconds_to_time_string(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
