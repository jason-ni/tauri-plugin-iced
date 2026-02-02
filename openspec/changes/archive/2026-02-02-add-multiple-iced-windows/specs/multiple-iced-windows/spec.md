## ADDED Requirements

### Requirement: Create multiple iced windows
The system SHALL allow users to create multiple independent iced windows dynamically from an existing iced interface, where each window maintains its own state and renders content independently.

#### Scenario: User creates a new random word window
- **WHEN** user clicks "Create Random Window" button in main window
- **THEN** system creates a new Tauri window with unique label
- **AND** system attaches iced renderer to the new window
- **AND** new window displays a randomly selected word centered in canvas

#### Scenario: Multiple windows operate independently
- **WHEN** user creates three random word windows
- **THEN** each window displays its own random word
- **AND** all windows render simultaneously without interference
- **AND** main window continues to respond to user input

### Requirement: Shared message type across windows
All windows SHALL share the same message type, with windows that have static content ignoring all messages in their update method.

#### Scenario: Static content window ignores messages
- **WHEN** a RandomWord window receives any message (Increment, Decrement, etc.)
- **THEN** window ignores the message and does not change its state
- **AND** window continues displaying the same word

#### Scenario: Main window responds to its messages
- **WHEN** user clicks "+" button in main window
- **THEN** counter increments and main window updates
- **AND** RandomWord windows remain unchanged

### Requirement: Unique window labels
The system SHALL generate unique window labels for each new window to prevent conflicts in the window management system.

#### Scenario: Sequential window creation has unique labels
- **WHEN** user creates first window
- **THEN** window has label "window_1"
- **WHEN** user creates second window
- **THEN** window has label "window_2"
- **WHEN** user creates third window
- **THEN** window has label "window_3"

### Requirement: Window cleanup on close
The system SHALL properly clean up iced window state when a Tauri window is closed.

#### Scenario: Window close removes state
- **WHEN** user closes a random word window
- **THEN** system removes the window from internal management
- **AND** other windows continue to operate normally
- **AND** no memory leak occurs
