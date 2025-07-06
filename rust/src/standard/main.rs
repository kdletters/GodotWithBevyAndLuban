use crate::data::TABLES;
use crate::standard::card::Card;
use crate::standard::deck::Deck;
use godot::builtin::GString;
use godot::classes::{Button, INode, Node, PackedScene};
use godot::global::godot_print;
use godot::obj::{Base, Gd, OnReady};
use godot::prelude::*;
use godot_tokio::AsyncRuntime;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Main {
    #[export]
    deck_1: OnEditor<Gd<Deck>>,
    #[export]
    deck_2: OnEditor<Gd<Deck>>,
    #[export]
    deck_3: OnEditor<Gd<Deck>>,

    #[export]
    scene_dis_name: GString,
    #[export]
    max_random_item_num: i32,
    #[export]
    min_random_item_num: i32,

    site_items: HashMap<String, i32>,
    #[init(load = "res://card/card.tscn")]
    card_scene: OnReady<Gd<PackedScene>>,
    #[export]
    draw_button: OnEditor<Gd<Button>>,

    base: Base<Node>,
}

impl Main {
    pub fn add_new_card(
        card_scene: Gd<PackedScene>,
        card_name: &str,
        mut card_deck: Gd<Deck>,
        caller: Gd<Deck>,
    ) -> Gd<Card> {
        godot_print!("开始创建新卡牌：{}", card_name);
        let card_info = TABLES.TbCardsInfo.get(card_name).unwrap();
        let card_class = &card_info.base_cardClass;
        godot_print!("添加的卡的类型为：{:?}", card_class);

        let mut card = card_scene.instantiate_as::<Card>();
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
            for (card_name, weight) in &self.site_items {
                current_weight += weight;
                if current_weight > random_weight {
                    selected_cards.push(card_name.to_string());
                    break;
                }
            }
        }
        let card_scene = self.card_scene.clone();
        let deck_1 = self.deck_1.clone();
        let deck_3 = self.deck_3.clone();

        godot::task::spawn(async move {
            let rt = AsyncRuntime::runtime();
            rt.spawn(async {
                tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
            })
            .await
            .unwrap();
            for card_name in selected_cards {
                rt.spawn(async {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                })
                .await
                .unwrap();
                Self::add_new_card(
                    card_scene.clone(),
                    &card_name,
                    deck_1.clone(),
                    deck_3.clone(),
                );
            }
        });
    }

    fn get_total_weight(&self) -> i32 {
        self.site_items.values().sum()
    }
}

#[godot_api]
impl INode for Main {
    fn ready(&mut self) {
        self.draw_button
            .signals()
            .pressed()
            .connect_other(self, Main::get_some_card);

        self.site_items = TABLES
            .TbSiteItems
            .data_map
            .iter()
            .map(|(_, value)| (value.base_cardName.clone(), value.weight))
            .collect();
    }
}
