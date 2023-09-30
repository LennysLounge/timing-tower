use bevy_egui::egui::{self, collapsing_header::CollapsingState, Id, InnerResponse, Response, Ui};

pub struct SplitCollapsingState<T> {
    pub id: Id,
    pub header_response: InnerResponse<(Response, T)>,
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

        SplitCollapsingState {
            id,
            header_response,
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
    pub fn get_header_response(self) -> (Response, InnerResponse<T>) {
        let header = self.header_response.response;
        let (button, header_return) = self.header_response.inner;

        (button, InnerResponse::new(header_return, header))
    }
}
