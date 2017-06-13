use conrod;

use vst2::plugin::HostCallback;

use app::config::AppConfig;
use vst2::host::Host;

/// Declare all widgets you're using.
widget_ids! {
    pub struct Ids {
        body,
        gain_slider,
        threshold_slider,
    }
}

/// Layout your UI here with conrod widgets. Gets updated every frame.
pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, app: &mut AppConfig) {

    use conrod::{Color, color, widget, Labelable, Colorable, Sizeable, Widget, Borderable, Positionable};
    use conrod::widget::Canvas;

    // background
    Canvas::new()
        .color(color::Color::Rgba(0.1, 0.1, 0.1, 1.0))
        .border(0.1)
        .set(ids.body, ui);

    // gain_slider
	if let Some(val) = widget::Slider::new(app.params[0].value, 0.0, 1.0)
		.w_h(300.0, 30.0)
		.x_y(0.0, 50.0)
		.color(color::LIGHT_BLUE)
		.border(1.0)
		// .label(&label)
		.label_color(color::WHITE)
        .set(ids.gain_slider, ui) {
            app.params[0].value = val;
            app.host.automate(0 as i32, app.params[0].value);
        }

    // threshold_slider
	if let Some(val) = widget::Slider::new(app.params[1].value, 0.0, 1.0)
		.w_h(300.0, 30.0)
		.x_y(0.0, -50.0)
		.color(color::LIGHT_PURPLE)
		.border(1.0)
		// .label(&label)
		.label_color(color::WHITE)
        .set(ids.threshold_slider, ui) {
            app.params[1].value = val;
            // info!("vst version: {:?}", app.host.vst_version());
            app.host.automate(1 as i32, app.params[1].value);
        }
}
