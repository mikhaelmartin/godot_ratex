# [godot_ratex](https://github.com/mikhaelmartin/godot-ratex)

Render LaTeX math expressions to images inside Godot 4.2+ using the [RaTeX](https://github.com/erweixin/RaTeX) engine.

## Installation

1. Download the latest release from [Releases](../../releases).
2. Extract the zip into your Godot project's `addons/` folder.
3. Reload the project

The folder structure should look like:

```
your_project/
└── addons/
    └── godot_ratex/
        ├── godot_ratex.gdextension
        ├── linux/
        │   ├── libgodot_ratex.x86_64.so
        │   └── libgodot_ratex.arm64.so
        ├── windows/
        │   ├── godot_ratex.x86_64.dll
        │   └── godot_ratex.arm64.dll
        ├── macos/
        │   ├── libgodot_ratex.x86_64.dylib
        │   └── libgodot_ratex.arm64.dylib
        ├── android/
        │   ├── libgodot_ratex.arm64.so
        │   └── libgodot_ratex.x86_64.so
        └── ios/
            ├── libgodot_ratex.a
            ├── libgodot_ratex.x86_64.a
            └── libgodot_ratex.simulator.a
```

### Demo

This repository is also a working Godot 4.6.3 project. The demo scene (`addons/godot_ratex/demo/godot_ratex_demo.tscn`) provides a UI with presets, a LaTeX input field, and controls for font size, padding, background color, and font color. Click **Render** to see the result in real time.

## Usage (GDScript)

```gdscript
var renderer = RaTeXRenderer.new()

# Configure properties (shown with defaults)
renderer.font_size = 48.0
renderer.padding = 12.0
renderer.background_color = Color.WHITE
renderer.font_color = Color.BLACK

# Render to PNG bytes
var png_bytes = renderer.render_latex("E = mc^2")

if png_bytes.is_empty():
    push_error("LaTeX rendering failed")
    return

var image = Image.new()
image.load_png_from_buffer(png_bytes)
var texture = ImageTexture.create_from_image(image)

$LaTeXDisplay.texture = texture
```

### Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `font_size` | `float` | `48.0` | Font size in pixels |
| `padding` | `float` | `12.0` | Padding around the rendered formula |
| `background_color` | `Color` | `Color.WHITE` | Background color (alpha controls transparency) |
| `font_color` | `Color` | `Color.BLACK` | Foreground/text color |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `render_latex(latex_string: String)` | `PackedByteArray` | Renders the LaTeX expression with current property values. Returns PNG bytes, or empty array on error. |

## Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux | x86_64, arm64 | ✓ |
| Windows | x86_64, arm64 | ✓ |
| macOS | arm64, x86_64 | ✓ |
| iOS | arm64 (device), x86_64 (sim), arm64 (sim) | ✓ |
| Android | arm64, x86_64 | ✓ |

## Development

Please visit the [godot_ratex](https://github.com/mikhaelmartin/godot-ratex) project for more information on how to contribute and build the library.

## License

MIT
