## Context

The test app currently displays widgets in a simple column layout with a canvas. Users want to overlay UI elements (like cursor position display) on top of interactive widgets without blocking their interaction. Iced's `stack!` widget provides layering capabilities, but the challenge is ensuring mouse events pass through the overlay canvas to underlying widgets.

### Current State
- `packages/test_app/src-tauri/src/lib.rs` has a `Counter` struct implementing `IcedControls`
- Layout uses `column!` with canvas, text, buttons, and text input
- Canvas implements `canvas::Program` trait with `draw()` method

### Constraints
- Must use existing Iced widgets (`stack!`, `canvas`, `container`)
- Cannot modify tauri-plugin-iced or Iced library
- Overlay must not interfere with widget interaction
- Implementation should be demonstrable in the test app

## Goals / Non-Goals

**Goals:**
- Demonstrate canvas layering above interactive widgets using `stack!`
- Implement cursor position tracking and display as floating text
- Preserve full interactivity of widgets beneath the canvas
- Provide a reusable pattern for overlay UI in Iced applications

**Non-Goals:**
- Creating a reusable library component (this is an example demonstration)
- Supporting complex overlay interactions (clicking the overlay, drag-and-drop, etc.)
- Implementing text wrapping or advanced positioning logic for floating text
- Adding keyboard shortcuts or controls to toggle overlay visibility

## Decisions

### Decision 1: Use `stack!` for Layering
**Choice**: Use Iced's `stack!` macro to layer canvas above interactive widgets.

**Rationale**: 
- `stack!` is the idiomatic Iced approach for layering widgets
- First child = bottom layer, last child = top layer (natural ordering)
- No custom layout logic required

**Alternatives Considered**:
- Custom absolute positioning widget: More complex, reinvents the wheel
- Separate overlay window: Breaks application context, window management complexity

### Decision 2: Canvas Returns `None` from `interaction()`
**Choice**: Implement `interaction()` method in `canvas::Program` to return `None`.

**Rationale**:
- When a widget returns `mouse::Interaction::None`, Iced's stack logic passes events to lower layers
- From `stack.rs:249-257`: `if interaction != mouse::Interaction::None { cursor = cursor.levitate(); }`
- This ensures lower-layer widgets receive mouse clicks and hover states

**Alternatives Considered**:
- `MouseArea` with no handlers: Still captures events, blocks lower layers
- Transparent container wrapper: Would need custom widget implementation, complex
- Conditional rendering (hide overlay when clicking): Poor UX, jarring visual changes

### Decision 3: Draw Floating Text Directly in Canvas
**Choice**: Draw cursor position text within canvas's `draw()` method using `canvas::Text`.

**Rationale**:
- Canvas already receives `cursor` parameter in `draw()` method (position relative to canvas bounds)
- No additional widgets (`pin`, `container`) needed for positioning
- Single drawing context, simpler implementation
- Text is part of canvas geometry, no z-index conflicts

**Alternatives Considered**:
- Separate `text` widget in stack: Would need to track state, update position manually
- `pin` widget in top layer: Positions relative to container, not absolute screen coordinates
- Tooltip widget: Only shows on hover, not always visible, designed for different use case

### Decision 4: Text Positioned at Cursor Coordinates
**Choice**: Position floating text at the cursor position (x, y) relative to canvas bounds.

**Rationale**:
- Direct visual feedback showing "this is where your cursor is"
- No offset calculation complexity
- Simple and predictable for users

**Alternatives Considered**:
- Offset text slightly (e.g., +10px, +10px): Prevents text from obscuring cursor, but adds complexity
- Center text in canvas: Doesn't provide spatial relationship to cursor
- Fixed position (e.g., top-left corner): Less intuitive, doesn't follow cursor movement

### Decision 5: Use `canvas::Frame::fill_text()` for Rendering
**Choice**: Use `frame.fill_text(canvas::Text { ... })` to render floating text.

**Rationale**:
- Native canvas drawing primitive
- Efficient (text becomes part of canvas geometry)
- Supports color, size, and positioning
- Consistent with existing canvas drawing pattern in test app

**Alternatives Considered**:
- Separate text widget in stack: Requires event handling, state management
- Custom drawing with lines/rectangles: No text support, would need to draw glyphs manually

## Risks / Trade-offs

### Risk 1: Text Obscures Canvas Content
[Risk] Floating text drawn at cursor position may cover or obscure content beneath it (other canvas drawings, widgets if visible through transparent canvas)

→ **Mitigation**: Use high-contrast text color (e.g., white, bright colors) that stands out. In production, could add semi-transparent background box behind text.

### Risk 2: Performance Impact with Frequent Redraws
[Risk] Canvas redraws on every cursor movement, potentially causing performance issues with complex canvas content or slow rendering.

→ **Mitigation**: For demonstration, this is acceptable. In production, consider throttling updates or using simpler overlay widgets. Canvas geometry caching (not part of this change) could also help.

### Trade-off: No Click Interaction with Overlay
[Trade-off] Overlay canvas cannot respond to clicks (e.g., to drag the floating text, copy coordinates) because we disable event capture.

→ **Acceptance**: This is by design for this use case. If interaction is needed, use different pattern (e.g., `MouseArea` with event forwarding, or interactive canvas that captures events and manually delegates to lower-layer widgets).

### Trade-off: Canvas Always Visible
[Trade-off] Overlay is always present, even when not needed (e.g., cursor outside window, user not interested in coordinates).

→ **Acceptance**: Simple implementation. Could add state to hide/show overlay (e.g., toggle button, hover to show) as enhancement.

## Migration Plan

No migration needed—this is a new feature demonstration in the test app.

### Deployment Steps
1. Modify `packages/test_app/src-tauri/src/lib.rs`:
   - Update `view()` method to use `stack!` instead of `column!`
   - Add `interaction()` method to `canvas::Program` implementation
   - Modify `draw()` method to add cursor text rendering
2. Test app build and run
3. Verify:
   - Canvas renders above buttons and text input
   - Buttons are clickable
   - Text input is focusable and accepts typing
   - Cursor position text follows mouse movement
   - Text is visible over canvas content

### Rollback Strategy
Revert changes to `lib.rs` to use original `column!` layout. No data or API changes, so rollback is safe and complete.

## Open Questions

None—all technical decisions are made. Implementation can proceed based on this design.
