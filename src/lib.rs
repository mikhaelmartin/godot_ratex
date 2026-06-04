use godot::prelude::*;

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
    fn build_display_list(&self, latex_string: &str) -> Option<ratex_types::DisplayList> {
        let font_color =
            ratex_types::Color::new(self.font_color.r, self.font_color.g, self.font_color.b, self.font_color.a);

        let parse_nodes = match ratex_parser::parse(latex_string) {
            Ok(nodes) => nodes,
            Err(e) => {
                godot_print!("RaTeX Parse Error: {:?}", e);
                return None;
            }
        };

        let layout_options = ratex_layout::LayoutOptions::default().with_color(font_color);
        let layout_box = ratex_layout::layout(&parse_nodes, &layout_options);
        Some(ratex_layout::to_display_list(&layout_box))
    }

    #[func]
    fn render_png(&self, latex_string: String) -> PackedByteArray {
        let display_list = match self.build_display_list(&latex_string) {
            Some(dl) => dl,
            None => return PackedByteArray::new(),
        };

        let background_color = ratex_types::Color::new(
            self.background_color.r,
            self.background_color.g,
            self.background_color.b,
            self.background_color.a,
        );

        let render_options = ratex_render::RenderOptions {
            font_size: self.font_size,
            padding: self.padding,
            background_color,
            ..Default::default()
        };

        match ratex_render::render_to_png(&display_list, &render_options) {
            Ok(png_bytes) => PackedByteArray::from_iter(png_bytes),
            Err(e) => {
                godot_print!("RaTeX Render Error: {:?}", e);
                PackedByteArray::new()
            }
        }
    }

    #[func]
    fn render_svg(&self, latex_string: String) -> godot::builtin::GString {
        let display_list = match self.build_display_list(&latex_string) {
            Some(dl) => dl,
            None => return godot::builtin::GString::new(),
        };

        let svg_options = ratex_svg::SvgOptions {
            font_size: self.font_size as f64,
            padding: self.padding as f64,
            embed_glyphs: true,
            ..Default::default()
        };

        let svg = ratex_svg::render_to_svg(&display_list, &svg_options);
        let svg = convert_rgba_to_hex(&svg);

        let bg_hex = format!(
            "#{:02x}{:02x}{:02x}",
            (self.background_color.r * 255.0).round() as u8,
            (self.background_color.g * 255.0).round() as u8,
            (self.background_color.b * 255.0).round() as u8,
        );

        let em = self.font_size as f64;
        let pad = self.padding as f64;
        let vb_w = display_list.width * em + 2.0 * pad;
        let vb_h = (display_list.height + display_list.depth) * em + 2.0 * pad;

        let svg_with_bg = if let Some(gt) = svg.find('>') {
            let rect = format!(
                "<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" fill-opacity=\"{}\"/>",
                vb_w, vb_h, bg_hex, self.background_color.a
            );
            let (head, tail) = svg.split_at(gt + 1);
            format!("{}{}{}", head, rect, tail)
        } else {
            svg
        };

        godot::builtin::GString::from(svg_with_bg.as_str())
    }

    #[func]
    fn render_pdf(&self, latex_string: String) -> PackedByteArray {
        let display_list = match self.build_display_list(&latex_string) {
            Some(dl) => dl,
            None => return PackedByteArray::new(),
        };

        let pdf_options = ratex_pdf::PdfOptions {
            font_size: self.font_size as f64,
            padding: self.padding as f64,
            ..Default::default()
        };

        match ratex_pdf::render_to_pdf(&display_list, &pdf_options) {
            Ok(pdf_bytes) => PackedByteArray::from_iter(pdf_bytes),
            Err(e) => {
                godot_print!("RaTeX PDF Error: {}", e);
                PackedByteArray::new()
            }
        }
    }
}

fn convert_rgba_to_hex(svg: &str) -> String {
    let mut result = String::with_capacity(svg.len());
    let search = "rgba(";
    let mut pos = 0;
    while let Some(start) = svg[pos..].find(search) {
        let abs_start = pos + start;
        result.push_str(&svg[pos..abs_start]);
        let rgba_start = abs_start + search.len();
        let rest = &svg[rgba_start..];
        if let Some(end) = rest.find(')') {
            let rgba = &rest[..end];
            let parts: Vec<&str> = rgba.split(',').collect();
            if parts.len() >= 3 {
                let r: u8 = parts[0].trim().parse().unwrap_or(0);
                let g: u8 = parts[1].trim().parse().unwrap_or(0);
                let b: u8 = parts[2].trim().parse().unwrap_or(0);
                let a: f32 = parts.get(3).map(|s| s.trim().parse().unwrap_or(1.0)).unwrap_or(1.0);
                if (a - 1.0).abs() < f32::EPSILON {
                    use std::fmt::Write;
                    let _ = write!(result, "#{:02x}{:02x}{:02x}\"", r, g, b);
                } else {
                    use std::fmt::Write;
                    let _ = write!(result, "#{:02x}{:02x}{:02x}\" fill-opacity=\"{}\"", r, g, b, a);
                }
            }
            pos = rgba_start + end + 2;
        } else {
            result.push_str(search);
            pos = abs_start + search.len();
        }
    }
    result.push_str(&svg[pos..]);
    result
}