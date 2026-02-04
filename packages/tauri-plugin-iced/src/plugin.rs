// Plugin implementation module

use crate::event_conversion;
use crate::renderer::IcedRenderer;
use crate::utils::IcedWindow;
use crate::IcedControls;
use anyhow::Error;
use iced_core::keyboard;
use iced_tiny_skia::graphics::Viewport;
use iced_winit::runtime::user_interface::Cache;
use iced_winit::Clipboard;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use tauri_runtime::dpi::PhysicalSize;
use tauri_runtime::window::CursorIcon;
use tauri_runtime::UserEvent;
use tauri_runtime_wry::tao::event::{Event, WindowEvent as TaoWindowEvent};
use tauri_runtime_wry::tao::event_loop::{ControlFlow, EventLoopProxy, EventLoopWindowTarget};
use tauri_runtime_wry::tao::window::WindowId;
use tauri_runtime_wry::{Context, Message, Plugin, PluginBuilder, WindowMessage};
use tauri_runtime_wry::{EventLoopIterationContext, WebContextStore};

/// Wrapper for staging IcedWindow to handle race conditions during window creation.
pub struct StagingWindowWrapper<M> {
    pub window: Option<(String, IcedWindow<M>)>,
}

/// Builder for creating the Iced plugin instance.
///
/// This implements the Tauri PluginBuilder trait.
pub struct Builder<M> {
    app: AppHandle,
    _phantom: std::marker::PhantomData<M>, // this does nothing, just keeps compiler happy
}

impl<M> Builder<M> {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            _phantom: PhantomData,
        }
    }
}

impl<T: 'static + UserEvent + std::fmt::Debug, M: 'static> PluginBuilder<T> for Builder<M> {
    type Plugin = IcedPlugin<T, M>;

    fn build(self, _: Context<T>) -> Self::Plugin {
        let iced_window_map: HashMap<String, IcedWindow<M>> = HashMap::new();
        let staging_window = Arc::new(Mutex::new(StagingWindowWrapper { window: None }));
        self.app.manage(staging_window.clone());
        IcedPlugin::new(self.app.clone(), staging_window, iced_window_map)
    }
}

/// The Iced plugin instance that hooks into Tauri's event loop.
pub struct IcedPlugin<T: UserEvent + std::fmt::Debug, M> {
    #[allow(dead_code)]
    app: AppHandle,
    staging_window: Arc<Mutex<StagingWindowWrapper<M>>>,
    windows: RefCell<HashMap<String, IcedWindow<M>>>,
    _phantom: std::marker::PhantomData<T>, // this does nothing, just keeps compiler happy
}

impl<T: UserEvent + std::fmt::Debug, M> IcedPlugin<T, M> {
    fn new(
        app: AppHandle,
        staging_window: Arc<Mutex<StagingWindowWrapper<M>>>,
        windows: HashMap<String, IcedWindow<M>>,
    ) -> Self {
        Self {
            app,
            staging_window,
            windows: RefCell::new(windows),
            _phantom: PhantomData,
        }
    }

    /// Helper function to extract label from a tao window ID (task 7.11).
    fn get_label_from_tao_id(
        window_id: WindowId,
        context: &EventLoopIterationContext<'_, T>,
    ) -> Option<String> {
        Self::get_id_from_tao_id(window_id, context).and_then(|id| {
            context
                .windows
                .0
                .borrow()
                .get(&id)
                .map(|ww| ww.label().to_string())
        })
    }

    /// Helper function to extract tao window ID from a tao window ID (task 7.11).
    fn get_id_from_tao_id(
        window_id: WindowId,
        context: &EventLoopIterationContext<'_, T>,
    ) -> Option<tauri_runtime::window::WindowId> {
        context.window_id_map.get(&window_id)
    }

    /// Convert Iced mouse interaction to Tauri cursor icon (task 7.12).
    fn convert_cursor_icon(mouse_interaction: &iced_core::mouse::Interaction) -> CursorIcon {
        match mouse_interaction {
            iced_core::mouse::Interaction::None => CursorIcon::Default,
            iced_core::mouse::Interaction::Hidden => CursorIcon::Default,
            iced_core::mouse::Interaction::Idle => CursorIcon::Default,
            iced_core::mouse::Interaction::ContextMenu => CursorIcon::ContextMenu,
            iced_core::mouse::Interaction::Help => CursorIcon::Help,
            iced_core::mouse::Interaction::Pointer => CursorIcon::Hand,
            iced_core::mouse::Interaction::Progress => CursorIcon::Progress,
            iced_core::mouse::Interaction::Wait => CursorIcon::Wait,
            iced_core::mouse::Interaction::Cell => CursorIcon::Cell,
            iced_core::mouse::Interaction::Crosshair => CursorIcon::Crosshair,
            iced_core::mouse::Interaction::Text => CursorIcon::Text,
            iced_core::mouse::Interaction::Alias => CursorIcon::Alias,
            iced_core::mouse::Interaction::Copy => CursorIcon::Copy,
            iced_core::mouse::Interaction::Move => CursorIcon::Move,
            iced_core::mouse::Interaction::NoDrop => CursorIcon::NoDrop,
            iced_core::mouse::Interaction::NotAllowed => CursorIcon::NotAllowed,
            iced_core::mouse::Interaction::Grab => CursorIcon::Grab,
            iced_core::mouse::Interaction::Grabbing => CursorIcon::Grabbing,
            iced_core::mouse::Interaction::ResizingHorizontally => CursorIcon::EwResize,
            iced_core::mouse::Interaction::ResizingVertically => CursorIcon::NsResize,
            iced_core::mouse::Interaction::ResizingDiagonallyUp => CursorIcon::NwseResize,
            iced_core::mouse::Interaction::ResizingDiagonallyDown => CursorIcon::NeswResize,
            iced_core::mouse::Interaction::ResizingColumn => CursorIcon::ColResize,
            iced_core::mouse::Interaction::ResizingRow => CursorIcon::RowResize,
            iced_core::mouse::Interaction::AllScroll => CursorIcon::AllScroll,
            iced_core::mouse::Interaction::ZoomIn => CursorIcon::ZoomIn,
            iced_core::mouse::Interaction::ZoomOut => CursorIcon::ZoomOut,
        }
    }

    /// Transfer staging window to main window map if labels match.
    fn transfer_staging_window(&mut self, label: &str) {
        let mut staging_window = self.staging_window.lock().unwrap();

        if !self.windows.borrow().contains_key(label) {
            if let Some(staging_label_opt) = staging_window.window.as_ref().map(|(l, _)| l.clone())
            {
                if label == staging_label_opt {
                    if let Some((staging_label, staging_win)) = staging_window.window.take() {
                        self.windows.borrow_mut().insert(staging_label, staging_win);
                    }
                }
            }
        }
    }
}

/// Extension trait for AppHandle to add Iced window support.  ///
///
/// This trait adds the `create_iced_window` method to Tauri's AppHandle.
pub trait AppHandleExt {
    /// Create an Iced-rendered window with the given controls.
    ///
    /// # Arguments
    /// * `label` - The window label (must match an existing Tauri window)
    /// * `controls` - The user's IcedControls implementation (must have Message = ())
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an error if:
    /// - Plugin not initialized
    /// - Window not found
    /// - Renderer initialization fails
    ///
    /// # Note
    /// The controls implementation must use `Message = ()` and override `handle_event`
    /// instead of `update` for event handling.
    fn create_iced_window<M: 'static>(
        &self,
        label: &str,
        controls: Box<dyn IcedControls<Message = M> + Send + Sync>,
    ) -> Result<(), Error>;
}

impl AppHandleExt for AppHandle {
    fn create_iced_window<M: 'static>(
        &self,
        label: &str,
        controls: Box<dyn IcedControls<Message = M> + Send + Sync>,
    ) -> Result<(), Error> {
        let window = self
            .get_window(label)
            .ok_or_else(|| anyhow::anyhow!("No window found with label: {}", label))?;

        crate::utils::set_window_transparent(&window);

        let scale_factor = window.scale_factor().unwrap_or(1.0) as f32;
        let PhysicalSize { width, height } = window.inner_size()?;

        let viewport: Viewport = event_conversion::create_viewport(width, height, scale_factor);

        // Using headless clipboard fallback (task 10.4)
        // Full clipboard integration requires winit window access
        let clipboard: Clipboard = Clipboard::unconnected();

        let iced_window = IcedWindow {
            label: label.to_string(),
            window,
            controls,
            renderer: None,
            viewport,
            events: std::vec::Vec::new(),
            cache: Cache::new(),
            clipboard,
            cursor: iced_core::mouse::Cursor::Unavailable,
            scale_factor,
            size: PhysicalSize { width, height },
            scene: None,
            resized: false,
            modifiers: keyboard::Modifiers::empty(),
        };

        let staging_window = self
            .try_state::<Arc<Mutex<StagingWindowWrapper<M>>>>()
            .ok_or_else(|| anyhow::anyhow!("TauriPluginIced is not initialized"))?;

        let mut stage = staging_window.lock().unwrap();
        stage.window = Some((label.to_string(), iced_window));

        Ok(())
    }
}

impl<T: UserEvent + std::fmt::Debug, M> Plugin<T> for IcedPlugin<T, M> {
    fn on_event(
        &mut self,
        event: &Event<Message<T>>,
        _event_loop: &EventLoopWindowTarget<Message<T>>,
        proxy: &EventLoopProxy<Message<T>>,
        _control_flow: &mut ControlFlow,
        context: EventLoopIterationContext<'_, T>,
        _: &WebContextStore,
    ) -> bool {
        match event {
            Event::LoopDestroyed => false,
            Event::WindowEvent {
                event: TaoWindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                if let Some(label) = Self::get_label_from_tao_id(*window_id, &context) {
                    log::info!(
                        "Window with label {} destroyed, windows count before: {}",
                        label,
                        self.windows.borrow().len()
                    );
                    if let Some(w) = self.windows.borrow_mut().remove(&label) {
                        drop(w);
                    }
                    log::info!("Windows count after: {}", self.windows.borrow().len());
                }
                false
            }
            Event::WindowEvent {
                event: tao_window_event,
                window_id,
                ..
            } => {
                if let Some(label) = Self::get_label_from_tao_id(*window_id, &context) {
                    self.transfer_staging_window(&label);

                    if let Some(iced_window) = self.windows.borrow_mut().get_mut(&label) {
                        match tao_window_event {
                            TaoWindowEvent::Resized(size) => {
                                iced_window.size = PhysicalSize::new(size.width, size.height);
                                iced_window.resized = true;
                            }
                            _ => {
                                if iced_window.handle_event(tao_window_event) {
                                    if let Some(win_id) =
                                        Self::get_id_from_tao_id(*window_id, &context)
                                    {
                                        let _ = proxy.send_event(Message::Window(
                                            win_id,
                                            WindowMessage::RequestRedraw,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }

                false
            }

            Event::RedrawRequested(window_id) => {
                if let Some(label) = Self::get_label_from_tao_id(*window_id, &context) {
                    self.transfer_staging_window(&label);

                    if let Some(iced_window) = self.windows.borrow_mut().get_mut(&label) {
                        if iced_window.renderer.is_none() {
                            let window = Arc::new(iced_window.window.clone());

                            let context = softbuffer::Context::new((*window).clone());
                            if let Err(e) = &context {
                                log::error!("Failed to create softbuffer context: {}", e);
                                return false;
                            }
                            let context = context.unwrap();

                            let surface_resource =
                                crate::renderer::SurfaceResource::new(context, window);

                            let renderer = IcedRenderer::new(surface_resource);
                            if let Err(e) = &renderer {
                                log::error!("Renderer initialization failed: {}", e);
                                return false;
                            }
                            iced_window.renderer = Some(renderer.unwrap());
                        }
                        iced_window.process_events();

                        // Render and get mouse interaction for cursor updates
                        if let Some(mouse_interaction) = iced_window.render_with_retry(&self.app) {
                            // Convert Iced mouse interaction to Tauri cursor icon
                            let cursor_icon = Self::convert_cursor_icon(&mouse_interaction);

                            // Get the Tauri window ID from tao window ID
                            if let Some(tauri_window_id) =
                                Self::get_id_from_tao_id(*window_id, &context)
                            {
                                // Send cursor icon update message to the window
                                let _ = proxy.send_event(Message::Window(
                                    tauri_window_id,
                                    WindowMessage::SetCursorIcon(cursor_icon),
                                ));
                            }
                        }
                    }
                }

                false
            }

            _ => false,
        }
    }
}
