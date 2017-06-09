use conrod;

use vst2::plugin::HostCallback;

widget_ids! {
    pub struct Ids {
        body,
        button,
        slider,
    }
}

use rand::{thread_rng, Rng};

pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, host: &mut HostCallback) {

    use conrod::{Color, color, widget, Labelable, Colorable, Sizeable, Widget, Borderable, Positionable};
    use conrod::widget::Canvas;

    let mut rng = thread_rng();
    let r = rng.gen::<f32>();
    let g = rng.gen::<f32>();
    let b = rng.gen::<f32>();

    // background
    Canvas::new()
        // .color(color::BLUE)
        .rgb(r, g, b)
        .border(0.1)
        .w_h(110.0, 150.0)
        .set(ids.body, ui);

    // slider
	if let Some(val) = widget::Slider::new(0.5, 0.0, 1.0)
		.w_h(300.0, 50.0)
		.middle_of(ids.body)
		.rgb(0.5, 0.3, 0.6)
		.border(1.0)
		// .label(&label)
		.label_color(color::WHITE)
	.set(ids.slider, ui) {
		info!("yesssss slide!");
	}

    // button
    for _click in widget::Button::new()
        .label("clickkk")
        .color(color::RED)
        .w_h(60.0, 30.0)
    .set(ids.button, ui) {
        info!("Bing!");
    }
}
