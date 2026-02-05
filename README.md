# Render Iced GUI on Tauri RawWindow

This plugin is inspired by [tauri-plugin-egui](https://github.com/clearlysid/tauri-plugin-egui). At first I added some features like multiple windows creation and destroy support in [my fork](https://github.com/jason-ni/tauri-plugin-egui). However, I found that Egui window consumes too much hardware resources due to its immediate mode rendering. And I observed around 20MB of memory leakage per window, in my Macbook Pro M4.

So I decided to explore other possible solutions. Then I found [iced](https://github.com/hecrj/iced) which is a pure Rust GUI library that is using contained mode and support wgpu and tiny-skia(cpu) renderer backend. What's more luck is that it has an example of integrating with winit and raw-window-handle. So I drive opencode and openspec to achieve this implementation currently in this repository.

But the memory leakage issue still exists. The test_example in this repository is intended to reproduce the memory leakage issue. In this test app, there are two buttons, one is to create raw empty window, and the the other is to take screenshot of the primary monitor and show it in a new window.

The leakage behavior:

- Creating empty raw window and destroying it may not cause memory leakage obviously. But when you create tens of windows and destroy them, the memory usage will keep increasing.

- Taking screenshot of the primary monitor and showing it in a new window will cause memory leakage obviously. When each window is closed, the memory usage increase by around 20MB.

To get black box comparation, I tested the same scenario in the Iced multi_window example. With tiny-skia backend, the multi_window example doesn't have memory leakage issue. Even I update it to show the screenshot image in a new window, the memory usage is stable(closed window memory usage is released).

So I think the leakage issue is related to how Tauri handles raw window graphics context. I'm calling for help from the Tauri community to find the root cause of this issue.

# How to use

Though this plugin has memory leakage issue, it can still be used for some simple scenarios. But you need to check the Cargo.toml and update the iced dependency. Currently, I use the `../iced` path to point to the local iced repository.