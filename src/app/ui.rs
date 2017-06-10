use conrod;

use vst2::plugin::HostCallback;

use app::config::AppConfig;
use vst2::host::Host;

/// Declare all widgets you're using.
widget_ids! {
    pub struct Ids {
        body,
        slider,
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

    // slider
	if let Some(val) = widget::Slider::new(app.params.params[0].value, 0.0, 1.0)
		.w_h(300.0, 30.0)
		.middle_of(ids.body)
		.rgb(0.5, 0.3, 0.6)
		.border(1.0)
		// .label(&label)
		.label_color(color::WHITE)
        .set(ids.slider, ui) {
            app.params.params[0].value = val;
            panic!("hihii");
            // app.host.automate(0 as i32, app.params.params[0].value);
        }
}
