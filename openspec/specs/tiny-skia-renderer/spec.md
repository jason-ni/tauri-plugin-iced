# tiny-skia-renderer Capability

## Purpose

Provides a software-based rendering capability using tiny_skia and softbuffer for Iced UI with custom scene drawing support. This capability eliminates GPU dependencies and memory leaks associated with wgpu rendering while maintaining full UI functionality.

## Requirements

### Requirement: Renderer initialization
The system SHALL initialize a tiny_skia renderer with default font and text size settings.

#### Scenario: Successful renderer creation
- **WHEN** renderer is created with Font::default() and Pixels::from(16)
- **THEN** renderer is initialized without errors
- **THEN** renderer contains valid Engine and empty layer stack

### Requirement: Softbuffer surface creation
The system SHALL create a softbuffer surface from Tauri window handle with specified dimensions.

#### Scenario: Surface creation on window
- **WHEN** softbuffer::Context::new() is called with Tauri window
- **THEN** context is created successfully
- **WHEN** softbuffer::Surface::new() is called with context and window
- **THEN** surface is created successfully
- **THEN** surface can be resized to valid dimensions

### Requirement: Render pipeline execution
The system SHALL render Iced UI and custom scenes to pixel buffer and present to window.

#### Scenario: Full render cycle
- **WHEN** RedrawRequested event occurs
- **THEN** system gets mutable pixel buffer from softbuffer surface
- **THEN** system creates PixmapMut from buffer bytes
- **THEN** system builds UserInterface with controls and viewport
- **THEN** system updates UI with current events
- **THEN** system draws UI to renderer (populates layers)
- **THEN** system composites renderer layers to PixmapMut
- **THEN** system draws custom scene to PixmapMut (if exists)
- **THEN** system presents buffer to window

#### Scenario: Render without custom scene
- **WHEN** RedrawRequested event occurs with no custom scene
- **THEN** system renders only Iced UI layers
- **THEN** system skips scene drawing step
- **THEN** system presents buffer to window

### Requirement: Resize handling
The system SHALL handle window resize by updating surface dimensions and viewport.

#### Scenario: Window resize event
- **WHEN** window receives Resized event
- **THEN** system marks window as resized
- **WHEN** next RedrawRequested occurs
- **THEN** system resizes softbuffer surface to new dimensions
- **THEN** system updates viewport with new physical size and scale factor
- **THEN** system clears resize flag

### Requirement: Mouse interaction tracking
The system SHALL extract mouse interaction state from UI update and return cursor icon.

#### Scenario: Mouse interaction update
- **WHEN** UI update returns State::Updated with mouse_interaction
- **THEN** system converts mouse_interaction to Tauri CursorIcon
- **THEN** system sends SetCursorIcon message to window

#### Scenario: No mouse interaction
- **WHEN** UI update returns State with no mouse_interaction
- **THEN** system does not send cursor icon update

### Requirement: Surface error handling
The system SHALL handle softbuffer surface errors gracefully by requesting redraw.

#### Scenario: Surface buffer access error
- **WHEN** surface.buffer_mut() returns Err
- **THEN** system requests window redraw
- **THEN** system returns early without rendering
- **THEN** next frame attempts buffer access again

### Requirement: Scene rendering API
The system SHALL provide Scene trait for custom drawing to pixel buffer.

#### Scenario: Scene draws to pixmap
- **WHEN** scene.draw(pixmap, bg_color) is called
- **THEN** scene draws geometric shapes directly to PixmapMut
- **THEN** scene uses tiny_skia drawing primitives
- **THEN** scene draws on top of Iced UI layers

### Requirement: Viewport management
The system SHALL maintain viewport with physical size, logical size, and scale factor.

#### Scenario: Viewport creation
- **WHEN** viewport is created with width, height, and scale_factor
- **THEN** viewport contains correct physical size
- **THEN** viewport contains correct logical size (physical / scale_factor)
- **THEN** viewport contains correct scale_factor

#### Scenario: Scale factor change
- **WHEN** window receives ScaleFactorChanged event
- **THEN** system updates viewport with new scale factor
- **THEN** system marks window as resized
- **THEN** next render uses new scale factor

### Requirement: Renderer layer management
The system SHALL manage renderer layers for damage-aware rendering optimization.

#### Scenario: Layer population
- **WHEN** interface.draw() is called on renderer
- **THEN** renderer stores all UI elements in layer stack
- **THEN** layers can be accessed via renderer.layers()
- **THEN** renderer.flush() prepares layers for compositing

### Requirement: Background color handling
The system SHALL apply background color to damage regions during compositing.

#### Scenario: Background color application
- **WHEN** renderer.draw() composites layers to PixmapMut
- **THEN** system fills damage rectangles with background color
- **THEN** system renders layers on top of background
- **THEN** background color comes from controls.background_color()

### Requirement: Damage rectangle optimization
The system SHALL support damage rectangle optimization for partial redraws.

#### Scenario: Full screen damage
- **WHEN** no previous frame exists or background color changed
- **THEN** system damages entire viewport
- **THEN** system redraws all layers

#### Scenario: Partial damage
- **WHEN** previous frame exists with same background
- **THEN** system calculates difference from previous layers
- **THEN** system damages only changed regions
- **THEN** system redraws only damaged regions
