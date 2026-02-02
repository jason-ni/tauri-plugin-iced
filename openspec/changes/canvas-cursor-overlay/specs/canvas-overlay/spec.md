## ADDED Requirements

### Requirement: Canvas layered above interactive widgets
The system SHALL enable canvas widgets to be displayed in a stack layer above other interactive widgets (buttons, text inputs, etc.) using Iced's `stack!` macro or `Stack::with_children()`.

#### Scenario: Canvas renders above widgets
- **WHEN** a canvas is added as the last child in a `stack!`
- **THEN** the canvas SHALL render visually above all other stack children
- **AND** the canvas SHALL cover the full area of the stack when sized with `Length::Fill`

### Requirement: Event pass-through for overlay canvas
The system SHALL configure the canvas to return `None` from its `interaction()` method, allowing mouse events to pass through to widgets in lower layers of the stack.

#### Scenario: Click passes through to button below
- **WHEN** user clicks on a button that is beneath an overlay canvas
- **THEN** the button SHALL receive and handle the click event
- **AND** the canvas SHALL not capture the click event

#### Scenario: Text input remains functional
- **WHEN** user types in a text input that is beneath an overlay canvas
- **THEN** the text input SHALL receive keyboard events and update its content
- **AND** the canvas SHALL not interfere with text input focus

### Requirement: Cursor position tracking and display
The system SHALL track cursor position relative to the canvas bounds and display it as floating text that follows the cursor movement.

#### Scenario: Cursor position displayed on move
- **WHEN** user moves the mouse cursor over the overlay canvas
- **THEN** the canvas SHALL display floating text showing the current cursor coordinates (x, y)
- **AND** the text SHALL be positioned at or near the cursor position

#### Scenario: Text formatted with coordinates
- **WHEN** cursor position is (123.456, 789.012)
- **THEN** the floating text SHALL display coordinates formatted as "x: 123, y: 789" (rounded to integers)
- **OR** display with precision appropriate to use case

### Requirement: Floating text rendering in canvas
The system SHALL draw the cursor position text using the canvas's drawing primitives within the `draw()` method, positioned at the cursor coordinates relative to the canvas bounds.

#### Scenario: Text positioned at cursor
- **WHEN** cursor is at position (100, 50) within canvas bounds
- **THEN** the canvas SHALL draw text with its top-left corner at or near (100, 50)
- **AND** the text SHALL use a contrasting color for visibility

#### Scenario: Text visible over canvas content
- **WHEN** canvas has drawn content at the cursor position
- **THEN** the floating text SHALL still be visible above or alongside the canvas content
- **AND** text color SHALL ensure readability (e.g., white, bright colors, or with background)

### Requirement: No event capture by overlay
The overlay canvas SHALL implement `interaction()` to return `None`, ensuring it never captures mouse interaction and always allows lower layers to handle events.

#### Scenario: interaction() returns None
- **WHEN** Iced queries the canvas for mouse interaction
- **THEN** the canvas's `interaction()` method SHALL return `Option<mouse::Interaction>::None`
- **AND** this SHALL occur regardless of cursor position within the canvas bounds

### Requirement: Stack layout preserves widget functionality
All interactive widgets in the bottom layer of the stack SHALL retain their full functionality (click handling, focus management, keyboard input, etc.) when an overlay canvas is present above them.

#### Scenario: Button click works with overlay
- **WHEN** a button is in the bottom layer of a stack with an overlay canvas on top
- **THEN** clicking the button SHALL trigger its associated message
- **AND** the button SHALL show normal hover and pressed states

#### Scenario: Text input focus works with overlay
- **WHEN** a text input is in the bottom layer of a stack with an overlay canvas on top
- **AND** user clicks on the text input
- **THEN** the text input SHALL receive focus
- **AND** user SHALL be able to type into it
