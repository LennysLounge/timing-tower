use bevy_egui::egui::{ComboBox, Response, Ui, Widget};

pub struct LComboBox<'a, T> {
    subject: &'a mut T,
    options: Vec<(T, &'static str)>,
    comparison_func: fn(&T, &T) -> bool,
}
impl<'a, T> LComboBox<'a, T>
where
    T: PartialEq,
{
    #[allow(unused)]
    pub fn new(subject: &'a mut T) -> Self {
        Self {
            subject,
            options: Vec::new(),
            comparison_func: |a, b| a == b,
        }
    }
}

impl<'a, T> LComboBox<'a, T> {
    pub fn new_comparable(subject: &'a mut T, comparison_func: fn(&T, &T) -> bool) -> Self {
        Self {
            subject,
            options: Vec::new(),
            comparison_func,
        }
    }
}

impl<T> LComboBox<'_, T> {
    pub fn add_option(mut self, value: T, label: &'static str) -> Self {
        self.options.push((value, label));
        self
    }
}

impl<T> Widget for LComboBox<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut changed = false;
        let mut response = ComboBox::new(ui.next_auto_id(), "select one:")
            .selected_text(
                self.options
                    .iter()
                    .find_map(|(value, label)| {
                        ((self.comparison_func)(value, &self.subject)).then_some(*label)
                    })
                    .unwrap_or("-"),
            )
            .show_ui(ui, |ui| {
                for option in self.options {
                    let is_selected = (self.comparison_func)(&option.0, &self.subject);
                    if ui.selectable_label(is_selected, option.1).clicked() && !is_selected {
                        *self.subject = option.0;
                        changed = true;
                    }
                }
            })
            .response;
        if changed {
            response.mark_changed();
        }
        response
    }
}
