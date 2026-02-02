## MODIFIED Requirements

### Requirement: Create multiple iced windows
The system SHALL allow users to create multiple independent iced windows dynamically from an existing iced interface, where each window maintains its own state and renders content independently. Each window shall implement the IcedControls trait and can display different content types (e.g., random words, screenshots).

#### Scenario: User creates a new random word window
- **WHEN** user clicks "Create Random Window" button in main window
- **THEN** system creates a new Tauri window with unique label
- **AND** system attaches iced renderer to the new window
- **AND** new window displays a randomly selected word centered in canvas

#### Scenario: User creates a new screenshot window
- **WHEN** user clicks "Take Screenshot" button in main window
- **THEN** system captures the primary screen using xcap
- **AND** system creates a new Tauri window with unique label
- **AND** system attaches iced renderer to the new window
- **AND** new window displays the captured screenshot with adaptive sizing

#### Scenario: Multiple windows operate independently
- **WHEN** user creates three windows (random word or screenshot types)
- **THEN** each window displays its own content independently
- **AND** all windows render simultaneously without interference
- **AND** main window continues to respond to user input

### Requirement: Shared message type across windows
All windows SHALL share the same message type, with windows that have static content ignoring all messages in their update method.

#### Scenario: Static content window ignores messages
- **WHEN** a RandomWord or ScreenshotViewer window receives any message (Increment, Decrement, etc.)
- **THEN** window ignores the message and does not change its state
- **AND** window continues displaying its static content (word or screenshot)

#### Scenario: Main window responds to its messages
- **WHEN** user clicks "+" button in main window
- **THEN** counter increments and main window updates
- **AND** RandomWord and ScreenshotViewer windows remain unchanged
