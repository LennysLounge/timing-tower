use bevy_egui::egui::{vec2, Align, Align2, Area, Frame, Id, Key, Layout, Order, Pos2, Ui};

#[allow(unused)]
pub struct Popup {
    id: Id,
    pos: Pos2,
    pivot: Align2,
    should_open: bool,
    should_close: bool,
    should_toggle: bool,
}
impl Popup {
    #[allow(unused)]
    pub fn new(id: Id, pos: Pos2) -> Self {
        Self {
            id,
            pos,
            pivot: Align2::LEFT_TOP,
            should_open: false,
            should_close: false,
            should_toggle: false,
        }
    }

    #[allow(unused)]
    pub fn should_open(mut self, should_open: bool) -> Self {
        self.should_open = should_open;
        self
    }

    #[allow(unused)]
    pub fn should_close(mut self, should_close: bool) -> Self {
        self.should_close = should_close;
        self
    }

    #[allow(unused)]
    pub fn should_toggle(mut self, should_toggle: bool) -> Self {
        self.should_toggle = should_toggle;
        self
    }

    #[allow(unused)]
    pub fn pivot(mut self, pivot: Align2) -> Self {
        self.pivot = pivot;
        self
    }

    #[allow(unused)]
    pub fn show<R>(
        mut self,
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut Ui, &mut bool) -> R,
    ) -> Option<R> {
        if self.should_toggle {
            self.should_close = ui.memory(|mem| mem.is_popup_open(self.id));
            self.should_open = !self.should_close;
        }
        if self.should_open {
            ui.memory_mut(|mem| mem.open_popup(self.id));
        }
        if self.should_close && ui.memory(|mem| mem.is_popup_open(self.id)) {
            ui.memory_mut(|mem| mem.close_popup());
        }

        if ui.memory(|mem| mem.is_popup_open(self.id)) {
            let mut close_requested = false;
            let area_res = Area::new(self.id)
                .order(Order::Foreground)
                .constrain(true)
                .fixed_pos(self.pos)
                .pivot(self.pivot)
                .show(ui.ctx(), |ui| {
                    let frame = Frame::popup(ui.style());
                    frame
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                                ui.set_max_size(vec2(0.0, 0.0));
                                add_content(ui, &mut close_requested)
                            })
                            .inner
                        })
                        .inner
                });

            if ui.input(|i| i.key_pressed(Key::Escape))
                || (area_res.response.clicked_elsewhere() && !self.should_open)
                || close_requested
            {
                ui.memory_mut(|mem| mem.close_popup());
            }
            Some(area_res.inner)
        } else {
            None
        }
    }
}

// pub fn popup<R>(
//     ui: &Ui,
//     popup_id: Id,
//     widget_response: &Response,
//     above_or_below: AboveOrBelow,
//     add_contents: impl FnOnce(&mut Ui) -> R,
// ) -> Option<R> {
//     if ui.memory(|mem| mem.is_popup_open(popup_id)) {
//         let (pos, pivot) = match above_or_below {
//             AboveOrBelow::Above => (widget_response.rect.left_top(), Align2::LEFT_BOTTOM),
//             AboveOrBelow::Below => (widget_response.rect.left_bottom(), Align2::LEFT_TOP),
//         };

//         let area_res = Area::new(popup_id)
//             .order(Order::Foreground)
//             .constrain(true)
//             .fixed_pos(pos)
//             .pivot(pivot)
//             .show(ui.ctx(), |ui| {
//                 let frame = Frame::popup(ui.style());
//                 let frame_margin = frame.total_margin();
//                 frame
//                     .show(ui, |ui| {
//                         ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
//                             ui.set_width(widget_response.rect.width() - frame_margin.sum().x);
//                             add_contents(ui)
//                         })
//                         .inner
//                     })
//                     .inner
//             });

//         ui.painter().rect_filled(
//             area_res.response.rect.expand(10.0),
//             0.0,
//             Color32::RED.linear_multiply(0.1),
//         );

//         if ui.input(|i| i.key_pressed(Key::Escape))
//             || (widget_response.clicked_elsewhere() && area_res.response.clicked_elsewhere())
//         {
//             ui.memory_mut(|mem| mem.close_popup());
//         }
//         Some(area_res.inner)
//     } else {
//         None
//     }
// }
