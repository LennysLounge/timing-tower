use bevy_egui::egui::{vec2, Color32, Rect, Rounding, Sense, Stroke, Ui};

struct TreeViewBuilderState {
    source: Option<Box<TreeViewBuilderState>>,
    depth: f32,
}
pub struct TreeViewBuilder2<'a> {
    state: TreeViewBuilderState,
    ui: &'a mut Ui,
}

impl<'a> TreeViewBuilder2<'a> {
    pub fn new(ui: &mut Ui, mut add_content: impl FnMut(TreeViewBuilder2<'_>)) {
        let mut child_ui: Ui = ui.child_ui(
            Rect::from_min_size(
                ui.cursor().min,
                vec2(ui.available_width(), ui.available_height()),
            ),
            *ui.layout(),
        );
        add_content(TreeViewBuilder2 {
            ui: &mut child_ui,
            state: TreeViewBuilderState {
                source: None,
                depth: 0.0,
            },
        });
        let rect = child_ui.min_rect();

        ui.painter()
            .rect_stroke(rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK));
    }

    pub fn leaf(&mut self, mut add_content: impl FnMut(&mut Ui)) {
        let res = self
            .ui
            .horizontal(|ui| {
                ui.allocate_response(vec2(100.0 * self.state.depth, 0.0), Sense::hover());
                add_content(ui);
                ui.allocate_response(vec2(ui.available_width(), 0.0), Sense::hover());
            })
            .response;
        self.ui
            .painter()
            .rect_stroke(res.rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK));
    }

    pub fn dir(self, mut add_conent: impl FnMut(&mut Ui)) -> Self {
        let res = self
            .ui
            .horizontal(|ui| {
                ui.allocate_response(vec2(100.0 * self.state.depth, 0.0), Sense::hover());
                add_conent(ui);
            })
            .response;
        self.ui
            .painter()
            .rect_stroke(res.rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK));

        TreeViewBuilder2 {
            ui: self.ui,
            state: TreeViewBuilderState {
                depth: self.state.depth + 1.0,
                source: Some(Box::new(self.state)),
            },
        }
    }

    pub fn close_dir(self) -> Option<Self> {
        self.state.source.map(|source_state| Self {
            state: *source_state,
            ui: self.ui,
        })
    }
}
