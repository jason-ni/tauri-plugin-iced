## 1. Update View Layout

- [x] 1.1 Refactor `view()` method to use `stack!` instead of `column!`
- [x] 1.2 Add canvas as top layer in stack with `Length::Fill` sizing
- [x] 1.3 Verify existing widgets (text, buttons, text input) remain in bottom layer
- [x] 1.4 Ensure stack layout compiles without errors

## 2. Implement Event Pass-Through

- [x] 2.1 Add `interaction()` method to `canvas::Program` implementation
- [x] 2.2 Return `None` from `interaction()` method to disable event capture
- [x] 2.3 Verify signature matches trait requirement: `fn interaction(...) -> Option<mouse::Interaction>`

## 3. Add Cursor Position Display

- [x] 3.1 Extract cursor position from `draw()` method's `cursor` parameter
- [x] 3.2 Calculate relative cursor position within canvas bounds
- [x] 3.3 Format cursor coordinates as text string ("x: {x}, y: {y}")
- [x] 3.4 Create `canvas::Text` object for cursor display with appropriate color and size
- [x] 3.5 Position text at cursor coordinates using `canvas::Text::position`
- [x] 3.6 Add text to frame using `frame.fill_text()`

## 4. Testing and Verification

- [x] 4.1 Build and run test app
- [x] 4.2 Verify canvas renders above buttons and text input visually
- [x] 4.3 Test button clicks work (buttons respond to clicks beneath canvas)
- [x] 4.4 Test text input functionality (focus, typing) works beneath canvas
- [x] 4.5 Verify cursor position text appears when mouse moves over canvas
- [x] 4.6 Confirm cursor text follows mouse movement and updates coordinates
- [x] 4.7 Verify text is visible and readable (contrast with background content)
- [x] 4.8 Check for any performance issues or flickering during cursor movement
