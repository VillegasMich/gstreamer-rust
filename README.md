# 📸 GTK4 Video Player with Live GStreamer Filters

A modern **photobooth-style GTK4 application** written in **Rust**. This app allows users to play videos with real-time **GStreamer visual effects**, choose filters dynamically, and interact with a responsive and styled UI.

---

## 🎥 Video

## ✨ Features

- 🎥 Video playback with `Play`, `Pause`, and `Stop` controls.
- 🎛️ Real-time GStreamer filters (effects like `timeoverlay`, `vertigotv`, and more).
- 🖌️ Custom GTK4 styling with light and dark theme support.
- 📦 Clean and responsive UI using `gtk::Picture`, `gtk::Button`, and CSS.
- 🎚️ Audio volume control and seek bar.
- 🪟 Floating window support for popups or previews.

---

## 🛠️ Requirements

- **Rust** (stable)
- **GTK4** (`libgtk-4-dev` on Linux)
- **GStreamer** (with `gstreamer`, `gstreamer-base`, `gstreamer-video`, etc.)
- `cargo`, `pkg-config`, and development headers for GTK and GStreamer.

Install dependencies on Linux:

```bash
sudo apt install libgtk-4-dev libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
```

On Arch-based systems:

```bash
sudo pacman -S gtk4 gstreamer gst-plugins-base gst-plugins-good
```

## 🚀 Run the App

```bash
cargo run <path-to-video>
```

## 🧩 Available Filters

You can dynamically select filters from a dropdown:

- no-filter – Original video

- timeoverlay – Shows timestamp overlay

- vertigotv – Trippy TV distortion effect

- edgetv – Edge detection shader

- dicetv – Cubic block distortion

- quarktv – Psychedelic wave effect

- agingtv – Vintage TV look

- shagadelictv – Groovy colorized distortion

- glfilterblur – Gaussian blur effect (OpenGL)

- glfilterinvert – Color inversion (OpenGL)

- glfiltersepia – Sepia tone (OpenGL)

- glfilteredge – Edge detection (OpenGL)

# 💬 Credits

Developed with ❤️ using GTK4 and GStreamer in Rust.
