use crate::{Card, Player, Point};

// ========== Point Tests ==========

#[test]
fn test_point_addition() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let result = p1 + p2;
    assert_eq!(result, Point { x: 4, y: 6 });
}

#[test]
fn test_point_subtraction() {
    let p1 = Point { x: 5, y: 7 };
    let p2 = Point { x: 2, y: 3 };
    let result = p1 - p2;
    assert_eq!(result, Point { x: 3, y: 4 });
}

#[test]
fn test_point_negation() {
    let p = Point { x: 3, y: -2 };
    let result = -p;
    assert_eq!(result, Point { x: -3, y: 2 });
}

#[test]
fn test_point_in_bounds() {
    assert!(Point { x: 0, y: 0 }.in_bounds());
    assert!(Point { x: 2, y: 2 }.in_bounds());
    assert!(Point { x: 4, y: 4 }.in_bounds());

    // Test all edges
    assert!(Point { x: 0, y: 2 }.in_bounds());
    assert!(Point { x: 4, y: 2 }.in_bounds());
    assert!(Point { x: 2, y: 0 }.in_bounds());
    assert!(Point { x: 2, y: 4 }.in_bounds());
}

#[test]
fn test_point_out_of_bounds() {
    assert!(Point { x: -1, y: 0 }.out_of_bounds());
    assert!(Point { x: 0, y: -1 }.out_of_bounds());
    assert!(Point { x: 5, y: 2 }.out_of_bounds());
    assert!(Point { x: 2, y: 5 }.out_of_bounds());
    assert!(Point { x: -1, y: -1 }.out_of_bounds());
    assert!(Point { x: 5, y: 5 }.out_of_bounds());
}

#[test]
fn test_point_invert() {
    let p = Point { x: 1, y: 2 };
    let inverted = p.invert();
    assert_eq!(inverted, Point { x: 3, y: 2 });

    // Test center point
    let center = Point { x: 2, y: 2 };
    assert_eq!(center.invert(), Point { x: 2, y: 2 });

    // Test corners
    assert_eq!(Point { x: 0, y: 0 }.invert(), Point { x: 4, y: 4 });
    assert_eq!(Point { x: 4, y: 4 }.invert(), Point { x: 0, y: 0 });
}

// ========== Player Tests ==========

#[test]
fn test_player_invert() {
    assert_eq!(Player::Red.invert(), Player::Blue);
    assert_eq!(Player::Blue.invert(), Player::Red);
}

#[test]
fn test_player_double_invert() {
    let player = Player::Red;
    assert_eq!(player.invert().invert(), player);

    let player = Player::Blue;
    assert_eq!(player.invert().invert(), player);
}

#[test]
fn test_player_display() {
    assert_eq!(format!("{}", Player::Red), "Red");
    assert_eq!(format!("{}", Player::Blue), "Blue");
}

// ========== Card Tests ==========

#[test]
fn test_card_moves_not_empty() {
    use enum_iterator::IntoEnumIterator;

    for card in Card::into_enum_iter() {
        let moves = card.moves();
        assert!(!moves.is_empty(), "Card {:?} has no moves", card);
        assert!(moves.len() <= 4, "Card {:?} has too many moves", card);
    }
}

#[test]
fn test_card_tiger_moves() {
    let moves = Card::Tiger.moves();
    assert_eq!(moves.len(), 2);
    assert!(moves.contains(&Point { x: 0, y: -2 }));
    assert!(moves.contains(&Point { x: 0, y: 1 }));
}

#[test]
fn test_card_crab_moves() {
    let moves = Card::Crab.moves();
    assert_eq!(moves.len(), 3);
    assert!(moves.contains(&Point { x: 0, y: -1 }));
    assert!(moves.contains(&Point { x: -2, y: 0 }));
    assert!(moves.contains(&Point { x: 2, y: 0 }));
}

#[test]
fn test_card_monkey_moves() {
    let moves = Card::Monkey.moves();
    assert_eq!(moves.len(), 4);
    // All diagonal moves
    assert!(moves.contains(&Point { x: -1, y: -1 }));
    assert!(moves.contains(&Point { x: 1, y: -1 }));
    assert!(moves.contains(&Point { x: -1, y: 1 }));
    assert!(moves.contains(&Point { x: 1, y: 1 }));
}

#[test]
fn test_card_index_roundtrip() {
    use enum_iterator::IntoEnumIterator;

    for card in Card::into_enum_iter() {
        let idx = card.index();
        let recovered = Card::from(idx);
        assert_eq!(card, recovered, "Card index roundtrip failed for {:?}", card);
    }
}

#[test]
fn test_card_indices_unique() {
    use enum_iterator::IntoEnumIterator;
    use std::collections::HashSet;

    let mut indices = HashSet::new();
    for card in Card::into_enum_iter() {
        let idx = card.index();
        assert!(indices.insert(idx), "Duplicate index {} for card {:?}", idx, card);
    }
}

#[test]
fn test_card_direction() {
    use crate::CardDirection;

    // Test some known directions
    assert!(matches!(Card::Frog.direction(), CardDirection::Left));
    assert!(matches!(Card::Rabbit.direction(), CardDirection::Right));
    assert!(matches!(Card::Tiger.direction(), CardDirection::Balanced));
    assert!(matches!(Card::Dragon.direction(), CardDirection::Balanced));
}

#[test]
fn test_card_display() {
    assert_eq!(format!("{}", Card::Tiger), "Tiger");
    assert_eq!(format!("{}", Card::Dragon), "Dragon");
}
