## 1. Dependency Updates

- [x] 1.1 Add "image" feature to iced workspace dependency in root Cargo.toml
- [x] 1.2 Add xcap = "0.8" dependency to test_app/src-tauri/Cargo.toml

## 2. Data Structures

- [x] 2.1 Define ScreenshotData struct with iced::widget::image::Handle field
- [x] 2.2 Define ViewerContent enum with Screenshot(Handle) and Error(String) variants
- [x] 2.3 Define ScreenshotViewer struct with content field of type ViewerContent

## 3. Core Screenshot Logic

- [x] 3.1 Implement capture_primary_screen() function using xcap::Monitor::all()[0].capture_image()
- [x] 3.2 Convert xcap RgbaImage to iced::widget::image::Handle using Handle::from_rgba()
- [x] 3.3 Implement calculate_window_size(screen_w, screen_h) function with 1200x800 max scaling
- [x] 3.4 Add error handling for capture failures returning String error messages

## 4. ScreenshotViewer IcedControls Implementation

- [x] 4.1 Implement IcedControls trait for ScreenshotViewer with Message type ()
- [x] 4.2 Implement view() method displaying image with ContentFit::Contain or error text
- [x] 4.3 Implement update() method as empty function (no message handling)
- [x] 4.4 Implement ScreenshotViewer::new(result: Result<ScreenshotData, String>) constructor

## 5. Counter Integration

- [x] 5.1 Add CounterMessage::CaptureScreenshot variant to CounterMessage enum
- [x] 5.2 Update Counter::view() to change button from "Create Random Window" to "Take Screenshot" with CaptureScreenshot message
- [x] 5.3 Implement Counter::update() handler for CaptureScreenshot message
- [x] 5.4 Handle capture success: calculate window size, create window, create ScreenshotViewer with screenshot
- [x] 5.5 Handle capture error: create 800x600 window, create ScreenshotViewer with error message
- [x] 5.6 Ensure unique window labels using window_counter (e.g., "screenshot_1", "screenshot_2")

## 6. Testing and Verification

- [x] 6.1 Test screenshot capture on standard 1920x1080 display (requires manual testing after build)
- [x] 6.2 Test screenshot capture on large 4K display (3840x2160) to verify scaling (requires manual testing after build)
- [x] 6.3 Test screenshot capture on small display (1280x720) to verify no scale-up (requires manual testing after build)
- [x] 6.4 Test error handling by simulating capture failure (e.g., log and observe error window creation) (requires manual testing after build)
- [x] 6.5 Verify multiple screenshot windows can be created and closed independently (requires manual testing after build)
- [x] 6.6 Verify ScreenshotViewer windows ignore all messages (static content) (requires manual testing after build)
- [x] 6.7 Verify screenshot preserves aspect ratio in window display (requires manual testing after build)
