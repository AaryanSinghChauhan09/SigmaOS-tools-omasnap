use sigmaos_tools_omasnap::{CaptureMode, ImageFormat, ScreenshotResult, SnapEngine};
use std::path::PathBuf;

#[test]
fn test_fullscreen_config_aaa() {
    // Arrange & Act
    let cfg = SnapEngine::build_config(CaptureMode::Fullscreen, "/tmp/out.png", ImageFormat::Png);
    // Assert
    assert_eq!(cfg.format.extension(), "png");
    assert!(SnapEngine::validate_config(&cfg).is_ok());
}

#[test]
fn test_region_validation_aaa() {
    // Arrange
    let invalid = SnapEngine::build_config(
        CaptureMode::Region {
            x: 0,
            y: 0,
            width: 0,
            height: 100,
        },
        "/tmp/out.png",
        ImageFormat::Png,
    );
    // Act & Assert
    assert!(SnapEngine::validate_config(&invalid).is_err());
}

#[test]
fn test_capture_history_aaa() {
    // Arrange
    let mut engine = SnapEngine::new();
    let res = ScreenshotResult {
        path: PathBuf::from("/tmp/shot.png"),
        width: 1920,
        height: 1080,
        format: ImageFormat::Png,
        file_size_bytes: 4096,
    };
    // Act
    engine.record_capture(res);
    // Assert
    assert_eq!(engine.history_count(), 1);
    assert_eq!(engine.last_capture().unwrap().width, 1920);
}
