use crate::card::{Card, CardState};
use godot::classes::{Control, HBoxContainer, IPanel, Panel};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Panel)]
pub struct Deck {
    #[init(node = "CardDeck")]
    card_deck: OnReady<Gd<Control>>,
    #[init(node = "ScrollContainer/CardPoiDeck")]
    card_poi_deck: OnReady<Gd<HBoxContainer>>,

    #[init(load = "res://card/card_background.tscn")]
    card_bg_scene: OnReady<Gd<PackedScene>>,

    base: Base<Panel>,
}

unsafe impl Sync for Deck {}

#[godot_api]
impl Deck {
    fn sort_nodes_by_position(&mut self, mut children: Vec<Gd<Card>>) {
        children.sort_by(Self::sort_by_position);
        for (i, child) in children.iter().enumerate() {
            if child.bind().card_current_state == CardState::Following {
                child.clone().set_z_index(i as i32);
                self.card_deck.move_child(child, i as i32);
            }
        }
    }

    fn sort_by_position(a: &Gd<Card>, b: &Gd<Card>) -> std::cmp::Ordering {
        let a_x = &a.get_position().x;
        let b_x = &b.get_position().x;
        b_x.total_cmp(a_x)
    }

    pub fn add_card(&mut self, mut card: Gd<Card>) {
        let index = card.get_z_index();
        let card_bg = self.card_bg_scene.instantiate_as::<Control>();
        self.card_poi_deck.add_child(&card_bg);
        if index <= self.card_poi_deck.get_child_count() {
            self.card_poi_deck.move_child(&card_bg, index);
        } else {
            self.card_poi_deck.move_child(&card_bg, -1);
        }

        let global_poi = card.get_global_position();
        if let Some(mut parent) = card.get_parent() {
            parent.remove_child(&card);
        }
        self.card_deck.add_child(&card);
        card.set_global_position(global_poi);
        let mut card = card.bind_mut();
        card.follow_target = Some(card_bg);
        card.pre_deck = Some(self.to_gd());
        card.card_current_state = CardState::Following
    }
}

#[godot_api]
impl IPanel for Deck {
    fn process(&mut self, delta: f64) {
        if self.card_deck.get_child_count() != 0 {
            let children = self.card_deck.get_children();
            let children = children
                .iter_shared()
                .map(|x| x.clone().cast::<Card>())
                .collect::<Vec<_>>();
            self.sort_nodes_by_position(children);
        }
    }
}
