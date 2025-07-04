use crate::deck::Deck;
use crate::translate::Localize;
use crate::{data, lang};
use godot::classes::{
    Button, Control, FileAccess, IControl, Label, ResourceLoader, Texture, Texture2D, TextureRect,
};
use godot::prelude::*;
use std::sync::Arc;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct Card {
    #[init(val = Vector2::ZERO)]
    velocity: Vector2,
    #[init(val = 0.25)]
    damping: real,
    #[init(val = 2000.0)]
    stiffness: real,

    #[export]
    pub card_current_state: CardState,
    #[export]
    pub follow_target: Option<Gd<Control>>,
    pub pre_deck: Option<Gd<Deck>>,
    data: Option<Arc<cfg::card::CardsInfo>>,

    #[init(node = "Button")]
    pick_button: OnReady<Gd<Button>>,
    #[init(node = "Control/ColorRect/Icon")]
    icon: OnReady<Gd<TextureRect>>,
    #[init(node = "Control/ColorRect/Icon/Label")]
    label: OnReady<Gd<Label>>,

    base: Base<Control>,
}

impl Card {
    pub fn init_data(&mut self, data: Arc<cfg::card::CardsInfo>) {
        self.data = Some(data);
        self.card_current_state = CardState::Following;

        self.draw_card();
    }

    fn draw_card(&mut self) {
        if let Some(data) = &self.data {
            let img = load::<Texture2D>(&data.base_icon);
            self.icon.set_texture(&img);
            self.label.set_text(&lang!(&data.base_displayName));
        }
    }

    fn follow(&mut self, target_position: Vector2, delta: f64) {
        let displacement = target_position - self.base().get_global_position();
        let force = displacement * self.stiffness;
        self.velocity += force * (delta as f32);
        self.velocity *= 1.0 - self.damping;
        let position = self.base().get_global_position();
        let velocity = self.velocity;
        self.base_mut()
            .set_global_position(position + velocity * (delta as f32));
    }

    fn on_button_down(&mut self) {
        self.card_current_state = CardState::Dragging;

        if let Some(follow_target) = &self.follow_target {
            follow_target.clone().queue_free();
        }
    }
    fn on_button_up(&mut self) {
        let mouse_position = self.base().get_global_mouse_position();
        let nodes = self
            .base()
            .get_tree()
            .unwrap()
            .get_nodes_in_group("CardDroppable");
        let mut in_which_deck = None;
        for node in nodes.iter_shared() {
            let node = node.cast::<Control>();
            if node.get_global_rect().contains_point(mouse_position) && node.is_visible() {
                if let Ok(deck) = node.try_cast::<Deck>() {
                    in_which_deck = Some(deck.clone());
                    break;
                }
            }
        }

        if let Some(deck) = in_which_deck {
            deck.clone().bind_mut().add_card(self.base_mut().clone().cast::<Card>());
        } else {
            if let Some(deck) = &self.pre_deck {
                deck.clone().bind_mut().add_card(self.base_mut().clone().cast::<Card>());
            }
        }

        self.card_current_state = CardState::Following;
    }
}

#[godot_api]
impl IControl for Card {
    fn process(&mut self, delta: f64) {
        match self.card_current_state {
            CardState::Following => {
                if let Some(follow_target) = &self.follow_target {
                    let target_position = follow_target.get_global_position();
                    self.follow(target_position, delta);
                }
            }
            CardState::Dragging => {
                let base = self.base();
                let target_position =
                    base.get_global_mouse_position() - base.get_size() / real!(2.0);
                // let position = base.get_global_position();
                // base.set_global_position(position.lerp(target_position, 0.4));
                self.follow(target_position, delta);
            }
        }
    }

    fn ready(&mut self) {
        self.pick_button
            .signals()
            .button_down()
            .connect_other(&self.to_gd(), Card::on_button_down);

        self.pick_button
            .signals()
            .button_up()
            .connect_other(&self.to_gd(), Card::on_button_up);
    }
}

#[derive(GodotConvert, Var, Export, Eq, PartialEq, Hash, Debug)]
#[godot(via = i8)]
pub enum CardState {
    Following,
    Dragging,
}

impl Default for CardState {
    fn default() -> Self {
        Self::Following
    }
}
