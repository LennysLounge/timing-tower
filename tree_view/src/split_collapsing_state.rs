use bevy_egui::egui::{self, collapsing_header::CollapsingState, Id, InnerResponse, Response, Ui};

pub struct SplitCollapsingState<T> {
    pub id: Id,
    pub button_response: Response,
    pub header_response: InnerResponse<T>,
}

impl<T> SplitCollapsingState<T> {
    pub fn show_header(
        ui: &mut Ui,
        id: Id,
        mut add_header: impl FnMut(&mut Ui) -> T,
    ) -> SplitCollapsingState<T> {
        let mut state = CollapsingState::load_with_default_open(ui.ctx(), id, true);
        let header_response = ui.horizontal(|ui| {
            let prev_item_spacing = ui.spacing_mut().item_spacing;
            ui.spacing_mut().item_spacing.x = 0.0; // the toggler button uses the full indent width
                                                   //let collapser = self.show_default_button_indented(ui);
            let collapser =
                state.show_toggle_button(ui, egui::collapsing_header::paint_default_icon);

            ui.spacing_mut().item_spacing = prev_item_spacing;
            (collapser, add_header(ui))
        });
        state.store(ui.ctx());

        let header = header_response.response;
        let (button, header_return) = header_response.inner;
        SplitCollapsingState {
            id,
            button_response: button,
            header_response: InnerResponse::new(header_return, header),
        }
    }

    pub fn toggle(&mut self, ui: &mut Ui) {
        if let Some(mut state) = CollapsingState::load(ui.ctx(), self.id) {
            state.toggle(ui);
            state.store(ui.ctx());
        }
    }

    pub fn show_body(
        &self,
        ui: &mut Ui,
        add_body: impl FnMut(&mut Ui) -> T,
    ) -> Option<InnerResponse<T>> {
        let mut state = CollapsingState::load_with_default_open(ui.ctx(), self.id, true);
        state.show_body_indented(&self.header_response.response, ui, add_body)
    }
}
