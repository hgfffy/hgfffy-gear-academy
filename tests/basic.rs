#[cfg(test)]
mod tests {
    use gstd::prelude::*;
    use gtest::{Program, System};
    use pebbles_game_io::*;

    const EXISTENTIAL_DEPOSIT: u128 = 60000000000000000000;

    #[test]
    fn test_init() {
        let system = System::new();

        system.init_logger();

        system.mint_to(101, EXISTENTIAL_DEPOSIT);

        // Load the program from the WASM file
        let program = Program::from_file(
            &system,
            "./target/wasm32-unknown-unknown/debug/pebbles_game.opt.wasm",
        );

        let init_msg = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        };

        // Send the initialization message
        let res = program.send(101, init_msg.encode());
        println!("res::{:?}", res);

        // Read the state
        let state: GameState = program.read_state(()).expect("Failed to read state");
        assert_eq!(state.pebbles_count, 15);
        assert_eq!(state.max_pebbles_per_turn, 3);
        assert!(state.pebbles_remaining <= 15);
    }

    #[test]
    fn test_user_turn() {
        let system = System::new();
        system.init_logger();
        system.mint_to(101, EXISTENTIAL_DEPOSIT);

        let program = Program::current(&system);

        let init_msg = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        };

        program.send_bytes(101, init_msg.encode());

        let action_msg = PebblesAction::Turn(2);
        program.send_bytes(101, action_msg.encode());

        let state: GameState = program.read_state(()).unwrap();
        assert_eq!(state.pebbles_remaining, 13);
    }

    #[test]
    fn test_program_turn_easy() {
        let system = System::new();
        system.init_logger();
        system.mint_to(101, EXISTENTIAL_DEPOSIT);

        let program = Program::current(&system);

        let init_msg = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        };

        program.send_bytes(101, init_msg.encode());

        let action_msg = PebblesAction::Turn(2);
        program.send_bytes(101, action_msg.encode());

        let state: GameState = program.read_state(()).unwrap();
        assert!(state.pebbles_remaining <= 13);
    }

    #[test]
    fn test_program_turn_hard() {
        let system = System::new();
        system.init_logger();
        system.mint_to(101, EXISTENTIAL_DEPOSIT);

        let program = Program::current(&system);

        let init_msg = PebblesInit {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        };

        program.send_bytes(101, init_msg.encode());

        let action_msg = PebblesAction::Turn(2);
        program.send_bytes(101, action_msg.encode());

        let state: GameState = program.read_state(()).unwrap();
        assert!(state.pebbles_remaining <= 13);
    }

    #[test]
    fn test_give_up() {
        let system = System::new();
        system.init_logger();
        system.mint_to(101, EXISTENTIAL_DEPOSIT);

        let program = Program::current(&system);

        let init_msg = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        };

        program.send_bytes(101, init_msg.encode());

        let action_msg = PebblesAction::GiveUp;
        program.send_bytes(101, action_msg.encode());

        let state: GameState = program.read_state(()).unwrap();
        assert_eq!(state.winner, Some(Player::Program));
    }

    #[test]
    fn test_restart() {
        let system = System::new();
        system.init_logger();
        system.mint_to(101, EXISTENTIAL_DEPOSIT);

        let program = Program::current(&system);

        let init_msg = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        };

        program.send_bytes(101, init_msg.encode());

        let restart_msg = PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 20,
            max_pebbles_per_turn: 4,
        };

        program.send_bytes(101, restart_msg.encode());

        let state: GameState = program.read_state(()).unwrap();
        assert_eq!(state.pebbles_count, 20);
        assert_eq!(state.max_pebbles_per_turn, 4);
        assert!(state.pebbles_remaining <= 20);
        assert_eq!(state.difficulty, DifficultyLevel::Hard);
    }
}
