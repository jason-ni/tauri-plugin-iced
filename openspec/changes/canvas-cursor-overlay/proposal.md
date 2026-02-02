## Why

To demonstrate how to layer a canvas widget above interactive UI elements (buttons, inputs) while preserving their usability. This pattern enables overlay UI elements like cursor position displays, tooltips, or floating labels that follow mouse movement without blocking user interaction with underlying widgets.

## What Changes

- Modify test app to use `stack!` widget for layered layout
- Implement canvas with `interaction()` method returning `None` to enable event pass-through
- Add cursor position tracking and display as floating text overlay
- Draw cursor position text directly in canvas at cursor coordinates
- Keep all interactive widgets (buttons, text inputs) functional beneath the canvas

## Capabilities

### New Capabilities
- `canvas-overlay`: Layering canvas widgets over interactive content with event pass-through, enabling overlay UI elements like cursor info, tooltips, or floating labels

### Modified Capabilities
- (none - no existing spec requirements are changing)

## Impact

- Changes to `packages/test_app/src-tauri/src/lib.rs` to demonstrate the pattern
- No API changes to tauri-plugin-iced or existing components
- Uses existing Iced widgets: `stack!`, `canvas`, `MouseArea`, `pin`, `container`
- Provides reusable pattern for overlay UI in Iced applications
