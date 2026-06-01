use godot::prelude::*;
use godot::global::{godot_print};

struct RaTeXExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RaTeXExtension {}

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct RaTeXRenderer {
    base: Base<RefCounted>,

    #[var]
    font_size: f32,

    #[var]
    padding: f32,

    #[var]
    background_color: Color,

    #[var]
    font_color: Color,
}

#[godot_api]
impl IRefCounted for RaTeXRenderer {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            font_size: 48.0,
            padding: 12.0,
            background_color: Color::from_rgba(1.0, 1.0, 1.0, 1.0),
            font_color: Color::from_rgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}

#[godot_api]
impl RaTeXRenderer {
    #[func]
    fn render_latex(&self, latex_string: String) -> PackedByteArray {
        
        let font_color = ratex_types::Color::new(self.font_color.r, self.font_color.g, self.font_color.b, self.font_color.a);
        let background_color = ratex_types::Color::new(self.background_color.r, self.background_color.g, self.background_color.b, self.background_color.a);

        // 1. Parsing String menjadi AST
        let parse_nodes = match ratex_parser::parse(&latex_string) {
            Ok(nodes) => nodes,
            Err(e) => {
                godot_print!("RaTeX Parse Error: {:?}", e);
                return PackedByteArray::new();
            }
        };
        
        // 2. Buat LayoutOptions dan ubah AST menjadi LayoutBox
        let layout_options = ratex_layout::LayoutOptions::default().with_color(font_color);
        let layout_box = ratex_layout::layout(&parse_nodes, &layout_options);
        
        // 3. Ubah LayoutBox menjadi DisplayList
        let display_list = ratex_layout::to_display_list(&layout_box);
        
        // 4. Set RenderOptions
        let render_options = ratex_render::RenderOptions {
            font_size: self.font_size,
            padding: self.padding,
            background_color,
            ..Default::default()
        };
        
        // 6. Render DisplayList ke PNG
        let result = ratex_render::render_to_png(&display_list, &render_options);
        
        match result {
            Ok(png_bytes) => {
                PackedByteArray::from_iter(png_bytes)
            }
            Err(e) => {
                godot_print!("RaTeX Render Error: {:?}", e);
                PackedByteArray::new()
            }
        }
    }
}