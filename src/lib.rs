#![forbid(unsafe_code)]
//! omasnap - Wayland screenshot utility for SigmaOS

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum CaptureMode {
    Fullscreen,
    Region {
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    },
    Window {
        title: String,
    },
    ActiveWindow,
}

#[derive(Debug, Clone)]
pub struct ScreenshotConfig {
    pub mode: CaptureMode,
    pub output_path: PathBuf,
    pub format: ImageFormat,
    pub include_cursor: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpg,
    Webp,
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            Self::Png => "png",
            Self::Jpg => "jpg",
            Self::Webp => "webp",
        }
    }
}

#[derive(Debug)]
pub struct ScreenshotResult {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub file_size_bytes: u64,
}

pub struct SnapEngine {
    history: Vec<ScreenshotResult>,
}

impl SnapEngine {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn build_config(
        mode: CaptureMode,
        output_path: &str,
        format: ImageFormat,
    ) -> ScreenshotConfig {
        ScreenshotConfig {
            mode,
            output_path: PathBuf::from(output_path),
            format,
            include_cursor: false,
        }
    }

    pub fn validate_config(config: &ScreenshotConfig) -> Result<(), String> {
        if let CaptureMode::Region { width, height, .. } = &config.mode {
            if *width == 0 || *height == 0 {
                return Err("Region dimensions must be non-zero".to_string());
            }
        }
        Ok(())
    }

    pub fn record_capture(&mut self, result: ScreenshotResult) {
        self.history.push(result);
    }

    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    pub fn last_capture(&self) -> Option<&ScreenshotResult> {
        self.history.last()
    }
}

impl Default for SnapEngine {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod oma_snap_tests {
    use super::*;

    #[test]
    fn test_image_formats_extensions_aaa() {
        // Arrange & Act & Assert
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpg.extension(), "jpg");
        assert_eq!(ImageFormat::Webp.extension(), "webp");
    }

    #[test]
    fn test_window_mode_validation_aaa() {
        // Arrange
        let cfg = SnapEngine::build_config(CaptureMode::ActiveWindow, "/tmp/win.webp", ImageFormat::Webp);
        // Act
        let val = SnapEngine::validate_config(&cfg);
        // Assert
        assert!(val.is_ok());
        assert_eq!(cfg.format.extension(), "webp");
    }

    #[test]
    fn test_empty_history_aaa() {
        // Arrange & Act
        let engine = SnapEngine::new();
        // Assert
        assert_eq!(engine.history_count(), 0);
        assert!(engine.last_capture().is_none());
    }
}
