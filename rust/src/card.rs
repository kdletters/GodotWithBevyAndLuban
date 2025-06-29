use crate::data;
use godot::classes::{
    Button, Control, Engine, FileAccess, IControl, Label, ResourceLoader, ResourcePreloader,
    TextureRect,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct Card {
    #[init(val = Vector2::ZERO)]
    velocity: Vector2,
    #[init(val = 0.35)]
    damping: real,
    #[init(val = 500.0)]
    stiffness: real,

    #[export]
    card_current_state: CardState,
    #[export]
    follow_target: Option<Gd<Control>>,
    #[init(node = "Button")]
    button: OnReady<Gd<Button>>,
    #[init(node = "Control/ColorRect/Icon")]
    icon: OnReady<Gd<TextureRect>>,
    #[init(node = "Control/ColorRect/Icon/Label")]
    label: OnReady<Gd<Label>>,

    base: Base<Control>,
}

#[godot_api]
impl IControl for Card {
    fn process(&mut self, delta: f64) {
        match self.card_current_state {
            CardState::Following => {
                if let Some(follow_target) = &self.follow_target {
                    let target_position = follow_target.get_global_position();
                    let displacement = target_position - self.base().get_global_position();
                    let force = displacement * self.stiffness;
                    self.velocity += force * (delta as f32);
                    self.velocity *= 1.0 - self.damping;
                    let position = self.base().get_global_position();
                    let velocity = self.velocity;
                    self.base_mut()
                        .set_global_position(position + velocity * (delta as f32));
                }
            }
            CardState::Dragging => {
                let mut base = self.base_mut();
                let target_position =
                    base.get_global_mouse_position() - base.get_size() / real!(2.0);
                let position = base.get_global_position();
                base.set_global_position(position.lerp(target_position, 0.4));
            }
        }
    }

    fn ready(&mut self) {
        godot_print!("button: {}", self.button.get_name());
        godot_print!("icon: {}", self.icon.get_texture().unwrap().get_name());
        godot_print!("label: {}-{}", self.label.get_name(), self.label.get_text());

        self.button
            .signals()
            .button_down()
            .connect_other(&self.to_gd(), Card::on_button_down);

        self.button
            .signals()
            .button_up()
            .connect_other(&self.to_gd(), Card::on_button_up);

        self.label
            .set_text(&data::TABLES.TbItem.get(&10000).unwrap().desc);
        for x in data::TABLES.TbItem.data_list.clone() {
            if let Some(item) = x.upgrade_to_item_id_ref.clone() {
                godot_print!("{:?}-{:?}", x.id, item.id);
            }
        }
    }
}

impl Card {
    fn on_button_down(&mut self) {
        self.card_current_state = CardState::Dragging;
    }
    fn on_button_up(&mut self) {
        self.card_current_state = CardState::Following;
    }
}

#[derive(GodotConvert, Var, Export)]
#[godot(via = i8)]
enum CardState {
    Following,
    Dragging,
}

impl Default for CardState {
    fn default() -> Self {
        Self::Following
    }
}
