use conrod;
use conrod::widget;
use conrod::widget::{ Widget };

pub struct Knob<'a> {
    // inherited crap
    common: widget::CommonBuilder,
    // knobs label
    maybe_label: Option<&'a str>,
    style: Style,
}

widget_style!{
    style Style {
        /// Color of the knob.
        - color: conrod::Color { theme.shape_color }
        /// Color of the knob's label.
        - label_color: conrod::Color { theme.label_color }
        /// Font size of the knob's label.
        - label_font_size: conrod::FontSize { theme.font_size_medium }
        /// Specify a unique font for the label.
        - label_font_id: Option<conrod::text::font::Id> { theme.font_id }
    }
}

widget_ids! {
    struct Ids {
        point_path,
    }
}

pub struct State {
    ids: Ids,
}

impl<'a> Knob<'a> {
    pub fn new() -> Self {
        Knob {
            common: widget::CommonBuilder::new(),
            maybe_label: None,
            style: Style::new(),
        }
    }
}


impl<'a> Widget for Knob<'a> {
    type State = State;
    type Style = Style;
    type Event = Option<()>;

    fn common(&self) -> &widget::CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut widget::CommonBuilder {
        &mut self.common
    }

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, mut ui, style, .. } = args;

        use conrod::Colorable;

        let points = vec!([0.0, 0.1], [1.0, 100.0]).into_iter();
        let color = style.color(ui.theme());

        widget::PointPath::new(points)
            .graphics_for(id)
            .parent(id)
            .thickness(1.0)
            .color(color)
            .set(state.ids.point_path, ui);

        None
    }
}

