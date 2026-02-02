## ADDED Requirements

### Requirement: Capture primary screen
The system SHALL allow users to capture the primary screen display using the xcap library and display the captured image in a new iced window.

#### Scenario: User captures primary screen successfully
- **WHEN** user clicks "Take Screenshot" button in Counter window
- **THEN** system captures the primary screen using xcap::Monitor
- **AND** system converts captured image to iced::widget::image::Handle
- **AND** system creates a new Tauri window with unique label
- **AND** system attaches ScreenshotViewer to the new window
- **AND** new window displays the captured screenshot

#### Scenario: Screen capture fails
- **WHEN** xcap::Monitor::capture_image() returns an error
- **THEN** system logs the error message
- **AND** system creates a new Tauri window with unique label
- **AND** system creates ScreenshotViewer with error content
- **AND** new window displays error message "Capture failed: <error details>"

### Requirement: Adaptive window sizing for screenshots
The system SHALL calculate window size based on screen dimensions, scaling down to fit maximum dimensions of 1200x800 pixels while preserving aspect ratio, without scaling up if screen is smaller.

#### Scenario: Window scaling for large screen
- **WHEN** captured screen dimensions are 3840x2160 pixels
- **THEN** system calculates scale factor as min(1200/3840, 800/2160, 1.0) = 0.312
- **AND** system creates window with size 1198x674 pixels (rounded from 3840*0.312, 2160*0.312)
- **AND** window preserves original screen aspect ratio (16:9)

#### Scenario: Window sizing for standard screen
- **WHEN** captured screen dimensions are 1920x1080 pixels
- **THEN** system calculates scale factor as min(1200/1920, 800/1080, 1.0) = 0.625
- **AND** system creates window with size 1200x675 pixels (1920*0.625, 1080*0.625)
- **AND** window size does not exceed max dimensions (1200x800)

#### Scenario: Window size equals screen size
- **WHEN** captured screen dimensions are 1280x720 pixels (smaller than max)
- **THEN** system calculates scale factor as min(1200/1280, 800/720, 1.0) = 1.0
- **AND** system creates window with exact screen size 1280x720 pixels
- **AND** system does not scale up the window

### Requirement: Display screenshot with content fit
The system SHALL display captured screenshots using ContentFit::Contain mode to ensure the entire image is visible within the window while preserving aspect ratio.

#### Scenario: Screenshot fills window exactly
- **WHEN** window size matches screenshot aspect ratio and dimensions
- **THEN** screenshot fills entire window area
- **AND** no padding or letterboxing is visible

#### Scenario: Screenshot maintains aspect ratio
- **WHEN** window has different aspect ratio than screenshot
- **THEN** screenshot scales to fit within window bounds
- **AND** screenshot preserves original aspect ratio
- **AND** ContentFit::Contain adds padding as needed

### Requirement: Screenshot viewer window message handling
The system SHALL create ScreenshotViewer windows that ignore all messages in their update method, as the screenshot content is static and requires no interaction.

#### Scenario: ScreenshotViewer ignores all messages
- **WHEN** ScreenshotViewer receives any message (Increment, Decrement, etc.)
- **THEN** ScreenshotViewer ignores the message
- **AND** ScreenshotViewer continues displaying the same screenshot or error
- **AND** window remains unchanged
