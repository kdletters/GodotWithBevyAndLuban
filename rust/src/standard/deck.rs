use crate::standard::card::{Card, CardState};
use godot::classes::{Control, HBoxContainer, IPanel, Panel};
use godot::global::godot_print;
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

impl Deck {
    fn sort_nodes_by_position(&mut self, mut children: Vec<Gd<Card>>) {
        children.sort_by(Self::sort_by_position);
        for (i, child) in children.iter().enumerate() {
            if child.bind().card_current_state == CardState::Following {
                child.clone().set_z_index((children.len() - i) as i32);
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
        let mouse_position = self.base().get_global_mouse_position();
        godot_print!("添加卡牌到牌组: 鼠标位置: {:?}", mouse_position);

        let card_bg = self.card_bg_scene.instantiate_as::<Control>();
        self.card_poi_deck.add_child(&card_bg);
        godot_print!("  创建卡牌背景并添加到卡牌点位牌组");

        // Determine insertion index based on mouse position if provided

        // Find the appropriate index based on mouse position
        let children = self.card_poi_deck.get_children();
        let mut insert_index = children.len() as i32;
        godot_print!(
            "  根据鼠标位置 ({}, {}) 确定插入位置",
            mouse_position.x,
            mouse_position.y
        );

        for i in 0..children.len() {
            let child = children.get(i).unwrap().cast::<Control>();
            let child_pos = child.get_global_position() + child.get_global_rect().size / 2.0;

            // If mouse is to the left of this card, insert before it
            if mouse_position.x < child_pos.x {
                insert_index = i as i32;
                godot_print!(
                    "  找到插入位置: {} (在位置 ({}, {}) 的卡牌之前)",
                    i,
                    child_pos.x,
                    child_pos.y
                );
                break;
            }
        }

        godot_print!("  最终插入索引: {}", insert_index);

        // Move the card background to the correct position
        self.card_poi_deck.move_child(&card_bg, insert_index);
        godot_print!("  移动卡牌背景到索引位置: {}", insert_index);

        let global_poi = card.get_global_position();
        godot_print!("  卡牌全局位置: ({}, {})", global_poi.x, global_poi.y);

        if let Some(mut parent) = card.get_parent() {
            godot_print!("  从原父节点移除卡牌");
            parent.remove_child(&card);
        }

        self.card_deck.add_child(&card);
        godot_print!("  将卡牌添加到卡牌牌组");

        card.set_global_position(global_poi);
        godot_print!("  设置卡牌全局位置: ({}, {})", global_poi.x, global_poi.y);

        let mut card = card.bind_mut();
        card.follow_target = Some(card_bg);
        card.pre_deck = Some(self.to_gd());
        card.card_current_state = CardState::Following;
        godot_print!("  设置卡牌状态为 Following，完成卡牌添加");
    }
}

#[godot_api]
impl IPanel for Deck {
    fn process(&mut self, _delta: f32) {
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
