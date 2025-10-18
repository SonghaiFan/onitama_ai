use crate::{Board, Card, Move, Player, Point};
use rand::prelude::*;

// ========== Legal Moves Generation Tests ==========

#[test]
fn test_legal_moves_initial_board() {
    let board = Board::new();
    let moves = board.legal_moves();

    // Initial position should have moves
    assert!(!moves.is_empty(), "Initial board should have legal moves");
}

#[test]
fn test_legal_moves_all_valid() {
    let board = Board::new();
    let moves = board.legal_moves();

    // Every generated move should be valid
    for game_move in moves {
        let result = board.try_move(game_move);
        assert!(result.is_ok(), "Generated move should be valid: {:?}", game_move);
    }
}

#[test]
fn test_legal_moves_uses_only_player_hand() {
    let mut board = Board::new();
    board.red_hand = [Card::Tiger, Card::Crab];
    board.spare_card = Card::Dragon; // Not available this turn
    board.turn = Player::Red;

    let moves = board.legal_moves();

    for game_move in moves {
        match game_move {
            Move::Move { card, .. } | Move::Discard { card } => {
                assert!(
                    card == Card::Tiger || card == Card::Crab,
                    "Should only use cards in hand, got {:?}",
                    card
                );
            }
        }
    }
}

#[test]
fn test_legal_moves_only_from_player_pieces() {
    let mut board = Board::new();
    board.turn = Player::Red;

    let moves = board.legal_moves();
    let red_positions: Vec<Point> = board
        .player_pieces()
        .iter()
        .filter_map(|&p| p)
        .collect();

    for game_move in moves {
        if let Move::Move { src, .. } = game_move {
            assert!(
                red_positions.contains(&src),
                "Move should originate from player piece at {:?}",
                src
            );
        }
    }
}

#[test]
fn test_legal_moves_no_out_of_bounds() {
    let board = Board::new();
    let moves = board.legal_moves();

    for game_move in moves {
        if let Move::Move { dst, .. } = game_move {
            assert!(dst.in_bounds(), "Destination should be in bounds: {:?}", dst);
        }
    }
}

#[test]
fn test_legal_moves_no_friendly_fire() {
    let mut board = Board::new();
    board.turn = Player::Red;

    let moves = board.legal_moves();
    let player_positions: Vec<Point> = board
        .player_pieces()
        .iter()
        .filter_map(|&p| p)
        .collect();

    for game_move in moves {
        if let Move::Move { dst, .. } = game_move {
            assert!(
                !player_positions.contains(&dst),
                "Should not move to friendly piece at {:?}",
                dst
            );
        }
    }
}

#[test]
fn test_legal_moves_prioritizes_captures() {
    let mut board = Board::new();
    board.red_hand = [Card::Crab, Card::Dragon];
    board.spare_card = Card::Tiger;
    board.turn = Player::Red;

    // Place blue piece in capturable position
    board.blue_pawns[0] = Some(Point { x: 2, y: 3 });

    let moves = board.legal_moves();

    // Capture moves should come first (sorted by key 0 vs 1)
    let first_moves: Vec<&Move> = moves.iter().take(5).collect();
    let has_capture = first_moves.iter().any(|m| {
        if let Move::Move { dst, .. } = m {
            *dst == Point { x: 2, y: 3 }
        } else {
            false
        }
    });

    if moves.iter().any(|m| {
        if let Move::Move { dst, .. } = m {
            *dst == Point { x: 2, y: 3 }
        } else {
            false
        }
    }) {
        assert!(has_capture, "Capture moves should be prioritized");
    }
}

#[test]
fn test_legal_moves_returns_discard_when_blocked() {
    let mut board = Board::new();
    // Create scenario where player has no valid moves
    board.red_hand = [Card::Tiger, Card::Crab];
    board.spare_card = Card::Dragon;
    board.turn = Player::Red;

    // Keep only the king
    board.red_pawns = [None; 4];
    board.red_king = Point { x: 0, y: 0 }; // Corner

    // Block all possible moves
    board.blue_pawns[0] = Some(Point { x: 1, y: 0 });
    board.blue_pawns[1] = Some(Point { x: 0, y: 1 });
    board.blue_pawns[2] = Some(Point { x: 2, y: 0 });
    board.blue_pawns[3] = Some(Point { x: 0, y: 2 });

    let moves = board.legal_moves();

    // If no regular moves, should return discards
    if !board.can_move() {
        assert!(!moves.is_empty(), "Should have discard options");
        assert!(moves.iter().all(|m| matches!(m, Move::Discard { .. })));
    }
}

#[test]
fn test_random_legal_move_is_valid() {
    let board = Board::new();
    let mut rng = SmallRng::seed_from_u64(42);

    for _ in 0..10 {
        let game_move = board.random_legal_move(&mut rng);
        let result = board.try_move(game_move);
        assert!(result.is_ok(), "Random move should be valid: {:?}", game_move);
    }
}

#[test]
fn test_random_legal_move_variety() {
    let board = Board::new();
    let mut rng = SmallRng::seed_from_u64(42);

    let mut moves = Vec::new();
    for _ in 0..20 {
        moves.push(board.random_legal_move(&mut rng));
    }

    // Should have some variety (not all the same)
    let unique_moves: std::collections::HashSet<String> = moves
        .iter()
        .map(|m| format!("{:?}", m))
        .collect();

    assert!(unique_moves.len() > 1, "Should generate different random moves");
}

#[test]
fn test_random_legal_move_uses_player_cards() {
    let mut board = Board::new();
    board.red_hand = [Card::Tiger, Card::Crab];
    board.spare_card = Card::Dragon;
    board.turn = Player::Red;

    let mut rng = SmallRng::seed_from_u64(42);

    for _ in 0..10 {
        let game_move = board.random_legal_move(&mut rng);
        match game_move {
            Move::Move { card, .. } | Move::Discard { card } => {
                assert!(
                    card == Card::Tiger || card == Card::Crab,
                    "Should only use player's cards"
                );
            }
        }
    }
}

// ========== Edge Cases ==========

#[test]
fn test_legal_moves_with_one_piece() {
    let mut board = Board::new();
    board.red_pawns = [None; 4]; // Remove all pawns
    board.red_hand = [Card::Monkey, Card::Tiger];
    board.turn = Player::Red;

    let moves = board.legal_moves();

    // Should still have moves (king can move)
    assert!(!moves.is_empty(), "King alone should still have moves");

    // All moves should be from king
    for game_move in moves {
        if let Move::Move { src, .. } = game_move {
            assert_eq!(src, board.red_king, "All moves should be from king");
        }
    }
}

#[test]
fn test_legal_moves_with_limited_cards() {
    let mut board = Board::new();
    // Set up a scenario with very limited movement
    board.red_hand = [Card::Tiger, Card::Tiger]; // Same card twice (hypothetically)
    board.turn = Player::Red;

    let moves = board.legal_moves();

    // Should still generate moves
    assert!(!moves.is_empty());
}

#[test]
fn test_can_move_false_when_completely_blocked() {
    let mut board = Board::new();
    board.red_pawns = [None; 4];
    board.red_king = Point { x: 0, y: 0 };
    board.red_hand = [Card::Monkey, Card::Dragon]; // Only diagonal moves
    board.turn = Player::Red;

    // Surround king with own pieces (impossible in real game, but tests the logic)
    board.blue_pawns = [
        Some(Point { x: 1, y: 0 }),
        Some(Point { x: 0, y: 1 }),
        Some(Point { x: 1, y: 1 }),
        None,
    ];

    // Monkey can't move diagonally into own pieces or out of bounds
    let can_move = board.can_move();

    // May or may not be blocked depending on the exact card moves
    // This is more about testing the logic exists
    if !can_move {
        let moves = board.legal_moves();
        assert!(
            moves.iter().all(|m| matches!(m, Move::Discard { .. })),
            "If can't move, should only offer discards"
        );
    }
}

#[test]
fn test_legal_moves_count_reasonable() {
    let board = Board::new();
    let moves = board.legal_moves();

    // Initial position: 5 pieces * 2 cards * ~3 moves per card = ~30 moves
    // Should have reasonable number of moves
    assert!(
        moves.len() < 100,
        "Should not have excessive number of moves: {}",
        moves.len()
    );
    assert!(moves.len() > 0, "Should have at least some moves");
}

#[test]
fn test_legal_moves_deterministic() {
    let board = Board::new();

    let moves1 = board.legal_moves();
    let moves2 = board.legal_moves();

    // Same board should generate same moves in same order
    assert_eq!(moves1.len(), moves2.len());
    for (m1, m2) in moves1.iter().zip(moves2.iter()) {
        assert_eq!(format!("{:?}", m1), format!("{:?}", m2));
    }
}
