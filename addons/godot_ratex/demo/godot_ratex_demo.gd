extends Control

@onready var latex_input: LineEdit = %LatexInput
@onready var font_size_spin: SpinBox = %FontSizeSpin
@onready var padding_spin: SpinBox = %PaddingSpin
@onready var bg_color_picker: ColorPickerButton = %BgColorPicker
@onready var fg_color_picker: ColorPickerButton = %FgColorPicker
@onready var render_button: Button = %RenderButton
@onready var texture_rect: TextureRect = %TextureRect
@onready var status_label: Label = %StatusLabel
@onready var presets: OptionButton = %Presets

const PRESETS := {
	"E = mc\u00B2": "E = mc^2",
	"Quadratic formula": "x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}",
	"Integral": "\\int_0^\\infty e^{-x}\\,dx",
	"Sum": "\\sum_{n=1}^\\infty \\frac{1}{n^2} = \\frac{\\pi^2}{6}",
	"Greek letters": "\\alpha + \\beta = \\gamma",
	"Trig identity": "\\sin^2\\theta + \\cos^2\\theta = 1",
}


func _ready() -> void:
	for label in PRESETS:
		presets.add_item(label)
	_check_plugin()

func _check_plugin() -> void:
	if not ClassDB.class_exists("RaTeXRenderer"):
		status_label.text = (
			"Error: RaTeXRenderer class not found.\n"
			+ "Try reinstalling the plugin or restart the editor."
		)
		status_label.add_theme_color_override("font_color", Color.RED)
		render_button.disabled = true

func _on_presets_item_selected(index: int) -> void:
	var label := presets.get_item_text(index)
	if PRESETS.has(label):
		latex_input.text = PRESETS[label]

func _on_render_button_pressed() -> void:
	var latex := latex_input.text.strip_edges()
	if latex.is_empty():
		return

	var renderer := RaTeXRenderer.new()
	renderer.font_size = float(font_size_spin.value)
	renderer.padding = float(padding_spin.value)
	renderer.background_color = bg_color_picker.color
	renderer.font_color = fg_color_picker.color

	var png_bytes := renderer.render_latex(latex)

	if png_bytes.is_empty():
		status_label.text = "Render failed — check console for errors"
		status_label.add_theme_color_override("font_color", Color.RED)
		texture_rect.texture = null
		return

	var image := Image.new()
	var err := image.load_png_from_buffer(png_bytes)
	if err != OK:
		status_label.text = "Failed to decode PNG (error %d)" % err
		status_label.add_theme_color_override("font_color", Color.RED)
		texture_rect.texture = null
		return

	var texture := ImageTexture.create_from_image(image)
	
	texture_rect.texture = texture
	texture_rect.size = texture.get_size()

	status_label.text = "Rendered: %dx%d px" % [image.get_width(), image.get_height()]
	status_label.remove_theme_color_override("font_color")
