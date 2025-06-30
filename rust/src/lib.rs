use crate::card::Card;
use crate::data::*;
use crate::deck::Deck;
use bevy::prelude::*;
use godot::classes::file_access::ModeFlags;
use godot::classes::{Button, Engine, FileAccess};
use godot::prelude::*;
use godot_bevy::prelude::*;
use std::time::Duration;
use tokio::runtime::Runtime;

mod card;
mod data;
mod deck;
mod translate;

#[bevy_app]
fn build_app(app: &mut App) {}

struct MyExtension;

unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Main {
    #[export]
    deck_1: Option<Gd<Deck>>,
    #[export]
    deck_2: Option<Gd<Deck>>,
    #[export]
    deck_3: Option<Gd<Deck>>,

    #[export]
    scene_dis_name: GString,
    #[export]
    max_random_item_num: i32,
    #[export]
    min_random_item_num: i32,

    #[export]
    site_items: Dictionary,
    #[init(load = "res://card/card.tscn")]
    card_scene: OnReady<Gd<PackedScene>>,
    #[export]
    draw_button: Option<Gd<Button>>,

    base: Base<Node>,
}

impl Main {
    pub fn add_new_card(
        &self,
        card_name: &str,
        mut card_deck: Gd<Deck>,
        caller: Gd<Deck>,
    ) -> Gd<Card> {
        godot_print!("开始创建新卡牌：{}", card_name);
        let card_info = TABLES.TbCardsInfo.get(card_name).unwrap();
        let card_class = &card_info.base_cardClass;
        godot_print!("添加的卡的类型为：{:?}", card_class);

        let mut card = self.card_scene.instantiate_as::<Card>();
        card.set_global_position(caller.get_global_position());
        card.set_z_index(100);
        card_deck.bind_mut().add_card(card.clone());
        card.bind_mut().init_data(card_info);

        return card;
    }

    fn get_some_card(&mut self) {
        let num_cards = rand::random::<i32>()
            % (self.max_random_item_num - self.min_random_item_num + 1)
            + self.min_random_item_num;

        let total_weight = self.get_total_weight();
        let mut selected_cards = vec![];

        for _ in 0..num_cards {
            let random_weight = rand::random::<i32>() % total_weight;
            let mut current_weight = 0;
            for (card_name, weight) in self.site_items.iter_shared() {
                current_weight += weight.to::<i32>();
                if current_weight > random_weight {
                    selected_cards.push(card_name.to_string());
                    break;
                }
            }
        }

        let mut delay = 0.1f64;
        for card_name in selected_cards {
            self.base()
                .get_tree()
                .unwrap()
                .create_timer(delay)
                .unwrap()
                .signals()
                .timeout()
                .connect_other(&self.to_gd(), move |this| {
                    this.add_new_card(
                        &card_name,
                        this.deck_1.clone().unwrap(),
                        this.deck_3.clone().unwrap(),
                    )
                });

            delay += 0.1;
        }
    }

    fn get_total_weight(&self) -> i32 {
        self.site_items
            .iter_shared()
            .fold(0, |x, (_, weight)| x + weight.to::<i32>())
    }
}

#[godot_api]
impl INode for Main {
    fn ready(&mut self) {
        if let Some(button) = &self.draw_button {
            button
                .signals()
                .pressed()
                .connect_other(self, Main::get_some_card);
        }
    }
}
