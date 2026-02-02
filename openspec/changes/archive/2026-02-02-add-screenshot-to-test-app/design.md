## Context

The test_app currently demonstrates multiple iced window creation through a "Create Random Window" button that creates windows displaying random words via a canvas-based RandomWord component. The app already has:
- Counter window with IcedControls implementation
- Multi-window support via tauri-plugin-iced
- Shared message type pattern across windows
- Counter creates new windows using `create_iced_window()` API

This change adds screen capture functionality, replacing the random word feature with screenshot display, to demonstrate image handling within the Tauri-Iced integration.

**Constraints:**
- Use synchronous (blocking) capture in update method - async Task is acceptable for future work
- Must integrate with existing tauri-plugin-iced window creation pattern
- Must use xcap library for screen capture
- Must maintain existing Counter message flow and patterns

## Goals / Non-Goals

**Goals:**
- Capture primary screen using xcap and display in new iced window
- Implement adaptive window scaling to fit 1200x800 max while preserving aspect ratio
- Display screenshots with proper error handling
- Follow existing IcedControls pattern for ScreenshotViewer
- Use blocking synchronous capture (acceptable 10-100ms pause)

**Non-Goals:**
- Interactive screenshot features (cropping, editing, saving)
- Async capture via Task (acceptable future enhancement)
- Multi-screen capture (only primary screen)
- Performance optimization for rapid repeated captures
- Canvas-based screenshot viewer (using simple image widget)

## Decisions

### Window Timing: Capture before window creation
**Decision:** Capture screenshot synchronously first, then create window with result

**Rationale:**
- Simpler implementation - no need to update window state after creation
- Better UX - window appears with content immediately
- Avoids complex message passing for async updates

**Alternatives considered:**
- Create window immediately with loading state, then update with capture result
  - Rejected: More complex, requires updating existing window state

### Screenshot Display: Simple image widget over canvas
**Decision:** Use `iced::widget::image` with `ContentFit::Contain`

**Rationale:**
- Simpler implementation (~5 lines vs 200+ lines for canvas viewer)
- Built-in scaling and aspect ratio preservation
- No custom drawing code needed
- Sufficient for "display only" requirement

**Alternatives considered:**
- Canvas-based ImageViewer (from screenshot_app example)
  - Rejected: Overkill for simple display, adds unnecessary complexity

### Screenshot Capture: Synchronous blocking in update()
**Decision:** Call xcap directly in Counter::update() without Task

**Rationale:**
- Simpler code flow, no async complexity
- Screen capture is fast (10-100ms), blocking is acceptable
- User explicitly requested this approach
- Async via Task acceptable for future work

**Alternatives considered:**
- Task::perform with async capture function
  - Rejected: User requested blocking approach, acceptable to defer async

### Window Size: Fixed max 1200x800 with adaptive scaling
**Decision:** Calculate scale = min(1200/screen_w, 800/screen_h, 1.0), window = screen * scale

**Rationale:**
- Balances usability (large enough to see details) and practicality (fits most screens)
- Preserves aspect ratio so image looks correct
- Never scales up (avoids poor quality on small screens)

**Alternatives considered:**
- Fixed window size (e.g., 800x600)
  - Rejected: Would crop or have empty space on different screen sizes
- 80% of primary screen
  - Rejected: More complex, requires getting screen size before capture

### Error Handling: Display error in new window
**Decision:** On capture failure, create window with ScreenshotViewer(Error(...)) content

**Rationale:**
- Consistent with success path - always create a window
- User sees feedback when something goes wrong
- Same pattern for both success and failure simplifies code

**Alternatives considered:**
- Don't create window, just log error
  - Rejected: User sees nothing, might click button multiple times
- Show error alert in parent window
  - Rejected: Different error pattern than success flow

### Dependency Location: Add "image" feature to workspace iced
**Decision:** Modify workspace dependency to add "image" feature to iced

**Rationale:**
- Cleaner workspace configuration
- If tauri-plugin-iced needs image support later, it's already available
- Single source of truth for iced features

**Alternatives considered:**
- Add "image" feature only to test_app (override workspace)
  - Rejected: Duplicate configuration, harder to maintain

### Message Type: Empty message type for ScreenshotViewer
**Decision:** ScreenshotViewer uses `()` as Message type with empty update()

**Rationale:**
- Matches existing RandomWord pattern
- Screenshot content is static, no interaction needed
- Simpler than creating dedicated message type

**Alternatives considered:**
- Create dedicated ScreenshotMessage enum
  - Rejected: Overkill, no messages needed for static display

## Risks / Trade-offs

**Risk: Blocking capture freezes UI momentarily**
→ Capture is fast (10-100ms), acceptable for demo purposes. Future work can add async via Task.

**Risk: Large screen captures consume significant memory**
→ Only one screenshot window at a time in this demo, memory impact minimal. User can close window to free memory.

**Trade-off: Simple image widget vs full-featured canvas viewer**
→ Canvas viewer offers cropping/editing but adds 200+ lines of code. Image widget meets "display only" requirement with 5 lines.

**Risk: xcap library platform compatibility**
→ xcap is cross-platform (Windows, macOS, Linux). Test_app already cross-platform via Tauri.

## Migration Plan

1. Update workspace Cargo.toml to add "image" feature to iced
2. Add xcap = "0.8" dependency to test_app/Cargo.toml
3. Modify test_app/src-tauri/src/lib.rs:
   - Add ScreenshotData and ScreenshotViewer structs
   - Add ViewerContent enum
   - Add CaptureScreenshot message variant
   - Implement capture_primary_screen() function
   - Implement calculate_window_size() function
   - Implement IcedControls for ScreenshotViewer
   - Update Counter::update() to handle CaptureScreenshot
   - Update Counter::view() to change button text to "Take Screenshot"
4. Test capture on different screen sizes
5. Test error handling (e.g., by disabling screen permissions)
6. Verify window cleanup when closing screenshot windows

**Rollback strategy:** If issues arise, revert to original "Create Random Window" functionality by restoring previous lib.rs state.

## Open Questions

None - all design decisions are finalized based on user requirements and exploration session.
