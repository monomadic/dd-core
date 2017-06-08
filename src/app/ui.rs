use conrod;

use vst2::plugin::HostCallback;

widget_ids! {
    pub struct Ids {
        body,
        button,
    }
}

pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, host: &mut HostCallback) {
    use conrod::{color, widget, Labelable, Colorable, Sizeable, Widget, Borderable};
    use conrod::widget::Canvas;

    Canvas::new()
        .color(color::BLUE)
        .border(0.5)
        .w_h(110.0, 150.0)
        .set(ids.body, ui);

    // let floating = widget::Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::RED);

    let button = widget::Button::new().label("clickkk").color(color::RED).w_h(60.0, 30.0);
    for _click in button.floating(true).set(ids.button, ui) {
        info!("Bing!");
    }
}
