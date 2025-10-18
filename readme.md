# Shorten Video Tool

A simple GUI application built with Rust and egui for trimming video files.

## Features

- **Simple GUI**: Clean and intuitive user interface
- **Video Trimming**: Extract specific time ranges from MP4 videos
- **Real-time Preview**: See output path before processing
- **Cross-platform**: Built with Rust for broad compatibility

## Requirements

- **FFmpeg**: Must be installed and available in your system PATH
- **Supported Formats**: Currently supports MP4 files

## Usage

1. Launch the application
2. Click "Open file..." to select an MP4 video
3. Adjust the start and end time sliders to select the desired segment
4. Enter an output filename
5. Click "Shorten!" to process the video

## Building

```bash
cargo build --release
```

## Dependencies

- [eframe](https://github.com/emilk/egui): GUI framework
- [mp4](https://crates.io/crates/mp4): MP4 file parsing
- [rfd](https://crates.io/crates/rfd): File dialogs

## License

This project is open source and available under the MIT License.