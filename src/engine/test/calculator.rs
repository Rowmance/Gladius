use engine::calculator;
use engine::calculator::ScoredSequence;
use logger;
use rules::fen_parser::parse_fen;
use rules::game_state::GameState;

// http://wtharvey.com/m8n2.txt

#[test]
fn search() {
    logger::setup();

    let state = GameState::start_position();
    println!("{}", state);

    let result = calculator::alpha_beta(&state, 10);
    println!("STATE {}", result);
    assert!(false);
}

#[test]
fn tactics_mate_in_2() {
    let state =
        parse_fen("r2qkb1r/pp2nppp/3p4/2pNN1B1/2BnP3/3P4/PPP2PPP/R2bK2R w KQkq - 1 0").unwrap();
    println!("{}", state);
    let result = calculator::alpha_beta(&state, 5);
    println!("MATE IN 2 {}", result);
    assert!(false);
}

#[test]
fn tactics_mate_in_2_2() {
    let state =
        parse_fen("2k5/8/7R/8/8/8/8/2K3R1 w - - 1 0").unwrap();
    println!("{}", state);
    let result = calculator::alpha_beta(&state, 5);
    println!("MATE IN 2 {}", result);
    assert!(false);
}
