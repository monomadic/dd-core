use conrod;

use vst2::plugin::HostCallback;

widget_ids! {
    pub struct Ids {
        body,
        slider,
    }
}

pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, host: &mut HostCallback) {

    use conrod::{Color, color, widget, Labelable, Colorable, Sizeable, Widget, Borderable, Positionable};
    use conrod::widget::Canvas;

    // background
    Canvas::new()
        .color(color::CHARCOAL)
        .border(0.1)
        .set(ids.body, ui);

    // slider
	if let Some(val) = widget::Slider::new(0.5, 0.0, 1.0)
		.w_h(300.0, 30.0)
		.middle_of(ids.body)
		.rgb(0.5, 0.3, 0.6)
		.border(1.0)
		// .label(&label)
		.label_color(color::WHITE)
	.set(ids.slider, ui) {
		info!("yesssss slide!");
	}
}
