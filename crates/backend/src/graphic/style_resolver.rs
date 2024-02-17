use bevy::{
    math::{vec2, Vec2, Vec3},
    render::color::Color,
};
use common::communication::{CellStyle, ClipAreaStyle};
use unified_sim_model::model::{Entry, Session};

use crate::{
    style::graphic::graphic_items::{cell::ComputedCell, clip_area::ComputedClipArea},
    value_store::{ModelContext, ValueResolver, ValueStore},
    value_types::{Boolean, Font, Number, Property, Text, Texture, Tint, Vec2Property},
};

#[derive(Clone)]
pub struct StyleResolver<'a> {
    value_store: &'a ValueStore,
    context: ModelContext<'a>,
    position: Vec3,
    render_layer: u8,
}
impl<'a> StyleResolver<'a> {
    pub fn new(value_store: &'a ValueStore, session: &'a Session) -> Self {
        Self {
            value_store,
            position: Vec3::ZERO,
            render_layer: 0,
            context: ModelContext {
                session: Some(session),
                entry: None,
            },
        }
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_render_layer(mut self, render_layer: u8) -> Self {
        self.render_layer = render_layer;
        self
    }

    pub fn with_entry(mut self, entry: &'a Entry) -> Self {
        self.context.entry = Some(entry);
        self
    }

    pub fn property<T>(&self, property: &Property<T>) -> Option<T>
    where
        ValueStore: ValueResolver<T>,
        T: Clone,
    {
        self.value_store.get_property(property, self.context)
    }

    pub fn session(&self) -> &Session {
        self.context
            .session
            .expect("Context should always have a session")
    }

    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    pub fn clip_area(&self, clip_area: &ComputedClipArea) -> ClipAreaStyle {
        ClipAreaStyle {
            pos: Vec3::new(
                self.value_store
                    .get_property(&clip_area.pos.x, self.context)
                    .unwrap_or_default()
                    .0,
                self.value_store
                    .get_property(&clip_area.pos.y, self.context)
                    .unwrap_or_default()
                    .0
                    * -1.0,
                self.value_store
                    .get_property(&clip_area.pos.z, self.context)
                    .unwrap_or_default()
                    .0,
            ) + self.position,
            size: Vec2::new(
                self.value_store
                    .get_property(&clip_area.size.x, self.context)
                    .unwrap_or_default()
                    .0,
                self.value_store
                    .get_property(&clip_area.size.y, self.context)
                    .unwrap_or_default()
                    .0,
            ),
            corner_offsets: {
                let skew = self
                    .value_store
                    .get_property(&clip_area.skew, self.context)
                    .unwrap_or_default()
                    .0;
                [
                    vec2(skew, 0.0),
                    vec2(skew, 0.0),
                    vec2(0.0, 0.0),
                    vec2(0.0, 0.0),
                ]
            },
            rounding: [
                self.value_store
                    .get_property(&clip_area.rounding.top_left, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&clip_area.rounding.top_right, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&clip_area.rounding.bot_left, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&clip_area.rounding.bot_right, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
            ],
            render_layer: clip_area.render_layer,
        }
    }

    pub fn cell(&self, cell: &ComputedCell) -> CellStyle {
        CellStyle {
            text: self
                .value_store
                .get_property(&cell.text, self.context)
                .unwrap_or_else(|| Text("unavailable".to_string()))
                .0,
            text_color: self
                .value_store
                .get_property(&cell.text_color, self.context)
                .unwrap_or(Tint(Color::BLACK))
                .0,
            text_size: self
                .value_store
                .get_property(&cell.text_size, self.context)
                .unwrap_or(Number(20.0))
                .0,
            text_alignment: cell.text_alginment.clone(),
            text_position: Vec2::new(
                self.value_store
                    .get_property(&cell.text_position.x, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.text_position.y, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
            ),
            font: self
                .value_store
                .get_property(&cell.font, self.context)
                .and_then(|f| match f {
                    Font::Default => None,
                    Font::Handle(handle) => Some(handle),
                }),
            color: self
                .value_store
                .get_property(&cell.color, self.context)
                .unwrap_or(Tint(Color::RED))
                .0,
            texture: self
                .value_store
                .get_property(&cell.image, self.context)
                .and_then(|t| match t {
                    Texture::None => None,
                    Texture::Handle(handle) => Some(handle),
                }),
            pos: Vec3::new(
                self.value_store
                    .get_property(&cell.pos.x, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.pos.y, self.context)
                    .unwrap_or(Number(0.0))
                    .0
                    * -1.0,
                self.value_store
                    .get_property(&cell.pos.z, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
            ) + self.position,
            size: Vec2::new(
                self.value_store
                    .get_property(&cell.size.x, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.size.y, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
            ),
            corner_offsets: {
                let skew = self
                    .value_store
                    .get_property(&cell.skew, self.context)
                    .unwrap_or_default()
                    .0;
                let get_vec = |prop: &Vec2Property| {
                    vec2(
                        self.value_store
                            .get_property(&prop.x, self.context)
                            .unwrap_or_default()
                            .0,
                        -self
                            .value_store
                            .get_property(&prop.y, self.context)
                            .unwrap_or_default()
                            .0,
                    )
                };
                [
                    get_vec(&cell.corner_offsets.top_left) + vec2(skew, 0.0),
                    get_vec(&cell.corner_offsets.top_right) + vec2(skew, 0.0),
                    get_vec(&cell.corner_offsets.bot_left) + vec2(0.0, 0.0),
                    get_vec(&cell.corner_offsets.bot_right) + vec2(0.0, 0.0),
                ]
            },
            visible: self
                .value_store
                .get_property(&cell.visible, self.context)
                .unwrap_or(Boolean(true))
                .0,
            rounding: [
                self.value_store
                    .get_property(&cell.rounding.top_left, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.rounding.top_right, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.rounding.bot_left, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.rounding.bot_right, self.context)
                    .unwrap_or(Number(0.0))
                    .0,
            ],
            render_layer: self.render_layer,
        }
    }
}
