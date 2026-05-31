use ratex_parser;
use ratex_layout::{self, LayoutOptions};
use ratex_render::{self, RenderOptions};
use ratex_types;

#[test]
fn parse_simple_expression() {
    let result = ratex_parser::parse(r"x^2 + y^2 = z^2");
    assert!(result.is_ok(), "Failed to parse simple expression: {:?}", result.err());
}

#[test]
fn parse_fraction() {
    let result = ratex_parser::parse(r"\frac{a}{b}");
    assert!(result.is_ok(), "Failed to parse fraction: {:?}", result.err());
}

#[test]
fn parse_sqrt() {
    let result = ratex_parser::parse(r"\sqrt{x + 1}");
    assert!(result.is_ok(), "Failed to parse sqrt: {:?}", result.err());
}

#[test]
fn parse_integral() {
    let result = ratex_parser::parse(r"\int_0^\infty e^{-x} dx");
    assert!(result.is_ok(), "Failed to parse integral: {:?}", result.err());
}

#[test]
fn parse_empty_outputs_empty_nodes() {
    let result = ratex_parser::parse("");
    assert!(result.is_ok(), "Empty string should be accepted");
    assert!(result.unwrap().is_empty(), "Empty string should produce no AST nodes");
}

#[test]
fn parse_garbled_returns_error() {
    let result = ratex_parser::parse("\x00\x01\x02");
    assert!(result.is_err(), "Garbled input should produce a parse error");
}

#[test]
fn full_pipeline_simple() {
    let nodes = ratex_parser::parse(r"E = mc^2").expect("Parse should succeed");
    let layout_box = ratex_layout::layout(&nodes, &LayoutOptions::default());
    let display_list = ratex_layout::to_display_list(&layout_box);
    let bg_color = ratex_types::Color::new(1.0, 1.0, 1.0, 1.0);
    let options = RenderOptions {
        font_size: 32.0,
        padding: 10.0,
        background_color: bg_color,
        ..Default::default()
    };
    let result = ratex_render::render_to_png(&display_list, &options);
    assert!(result.is_ok(), "Rendering failed: {:?}", result.err());
    let png_bytes = result.unwrap();
    assert!(!png_bytes.is_empty(), "PNG output should not be empty");
}

#[test]
fn full_pipeline_subscript_superscript() {
    let nodes = ratex_parser::parse(r"a_i^j + b_k").expect("Parse should succeed");
    let layout_box = ratex_layout::layout(&nodes, &LayoutOptions::default());
    let display_list = ratex_layout::to_display_list(&layout_box);
    let bg_color = ratex_types::Color::new(0.0, 0.0, 0.0, 1.0);
    let options = RenderOptions {
        font_size: 24.0,
        padding: 5.0,
        background_color: bg_color,
        ..Default::default()
    };
    let result = ratex_render::render_to_png(&display_list, &options);
    assert!(result.is_ok(), "Rendering failed: {:?}", result.err());
}

#[test]
fn full_pipeline_transparent_bg() {
    let nodes = ratex_parser::parse(r"\alpha + \beta = \gamma").expect("Parse should succeed");
    let layout_box = ratex_layout::layout(&nodes, &LayoutOptions::default());
    let display_list = ratex_layout::to_display_list(&layout_box);
    let bg_color = ratex_types::Color::new(0.0, 0.0, 0.0, 0.0);
    let options = RenderOptions {
        font_size: 48.0,
        padding: 15.0,
        background_color: bg_color,
        ..Default::default()
    };
    let result = ratex_render::render_to_png(&display_list, &options);
    assert!(result.is_ok(), "Rendering with transparent bg failed: {:?}", result.err());
}

#[test]
fn full_pipeline_zero_font_size() {
    let nodes = ratex_parser::parse(r"x").expect("Parse should succeed");
    let layout_box = ratex_layout::layout(&nodes, &LayoutOptions::default());
    let display_list = ratex_layout::to_display_list(&layout_box);
    let bg_color = ratex_types::Color::new(1.0, 1.0, 1.0, 1.0);
    let options = RenderOptions {
        font_size: 0.0,
        padding: 0.0,
        background_color: bg_color,
        ..Default::default()
    };
    let result = ratex_render::render_to_png(&display_list, &options);
    assert!(result.is_ok(), "Zero font size should not panic/crash");
}
