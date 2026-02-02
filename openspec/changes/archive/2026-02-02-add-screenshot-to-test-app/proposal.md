## Why

Add screenshot functionality to test_app to demonstrate screen capture capabilities using xcap library within the Tauri-Iced integration. This feature will allow users to capture the primary screen and display it in a new window, providing a practical example of image handling and dynamic window content.

## What Changes

- Add screenshot capture button to Counter window (replacing "Create Random Window" button)
- Implement screen capture logic using xcap library to capture primary screen
- Create new ScreenshotViewer struct that implements IcedControls to display captured images
- Add adaptive window sizing that scales screenshot windows to fit 1200x800 max while preserving aspect ratio
- Add error handling for screen capture failures with error message display in window
- Add dependencies: xcap = "0.8" and "image" feature to iced

## Capabilities

### New Capabilities
- `screen-capture`: Ability to capture the primary screen and display the screenshot in a new iced window with adaptive scaling

### Modified Capabilities
- `multiple-iced-windows`: This change extends the existing multiple-iced-windows capability by adding a new window type (ScreenshotViewer) alongside the existing RandomWord window type. The core window creation mechanism remains unchanged.

## Impact

- **Code**: `packages/test_app/src-tauri/src/lib.rs` - Add ScreenshotViewer struct, capture function, and update Counter
- **Dependencies**: Add xcap = "0.8" to test_app Cargo.toml and enable "image" feature on iced workspace dependency
- **Performance**: Screenshot capture blocks UI thread during capture (expected 10-100ms), acceptable for this use case
- **API**: No external API changes - internal only
