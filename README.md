# godot_ratex

Render LaTeX math expressions to images inside Godot 4.2+ using the [RaTeX](https://github.com/erweixin/RaTeX) engine.

## Installation

1. Download the latest release from [Releases](../../releases).
2. Extract the zip into your Godot project's `addons/` folder.
3. Enable the plugin in **Project → Project Settings → Plugins**.

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
        └── macos/
            ├── libgodot_ratex.x86_64.dylib
            └── libgodot_ratex.arm64.dylib
```

### Demo

This repository is also a working Godot project. Open `project.godot` in Godot 4.6.3, then run `./build.sh` to compile the extension for your platform. The demo scene (`main.tscn`) provides a UI to type LaTeX expressions, tweak font size/padding/background color, and see the rendered result.

## Usage (GDScript)

```gdscript
var renderer = RaTeXRenderer.new()
var png_bytes = renderer.render_latex("E = mc^2", 48.0, 12.0, Color.WHITE, Color.BLACK)

if png_bytes.is_empty():
    push_error("LaTeX rendering failed")
    return

var image = Image.new()
image.load_png_from_buffer(png_bytes)
var texture = ImageTexture.create_from_image(image)

$LaTeXDisplay.texture = texture
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `latex_string` | `String` | The LaTeX math expression to render |
| `font_size` | `float` | Font size in pixels |
| `padding` | `float` | Padding around the rendered formula |
| `background_color` | `Color` | Background color (including alpha for transparency) |
| `font_color` | `Color` | Foreground/text color for the rendered formula |

Returns a `PackedByteArray` containing PNG image bytes, or an empty array on error.

## Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux | x86_64, arm64 | ✓ |
| Windows | x86_64, arm64 | ✓ |
| macOS | arm64, x86_64 | ✓ |
| iOS | arm64, x86_64 | ✓ |
| Android | arm64, x86_64 | ✓ |

## Development

### Prerequisites

- Rust toolchain (stable)
- Godot 4.2+

### Building from source

```bash
# Build for current platform (debug)
./build.sh

# Build for current platform (release)
./build.sh --release

# Build specific platform
./build.sh --platform linux,android

# Build everything for release
./build.sh --all
```

Compiled libraries are placed directly in `addons/godot_ratex/<platform>/`.

### Running tests

```bash
./test.sh                # Run all tests
./test.sh --release      # Run in release mode
./test.sh --test parse   # Run only tests matching "parse"
./test.sh --verbose      # Show full test output
```

## License

MIT
