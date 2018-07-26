use models;


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Modalities {
    HumanVsComputer,
    ComputerVsComputer,
    ComputerVsHuman,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AILevel {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    VeryHard,
    Extreme,
}

pub fn ai_level_from_str(s: &str) -> Option<AILevel> {
	match s {
		"VeryEasy" => Some(AILevel::VeryEasy),
		"Easy" => Some(AILevel::Easy),
		"Medium" => Some(AILevel::Medium),
		"Hard" => Some(AILevel::Hard),
		"VeryHard" => Some(AILevel::VeryHard),
		"Extreme" => Some(AILevel::Extreme),
		_ => None,
	}
}

pub fn ai_moves(ai_level: AILevel) -> usize {
    match ai_level {
        AILevel::VeryEasy => 80,
        AILevel::Easy => 150,
        AILevel::Medium => 500,
        AILevel::Hard => 2000,
        AILevel::VeryHard => 5000,
        AILevel::Extreme => 10000,
    }
}

pub fn get_opposite_from_turn(player: models::Player, modality: Modalities) -> models::Player {
    match player {
        models::Player::Player1 | models::Player::AIPlayer1 => models::Player::AIPlayer2,
        models::Player::Player2 => models::Player::AIPlayer1,
        models::Player::AIPlayer2 if modality == Modalities::HumanVsComputer => models::Player::Player1,
        models::Player::AIPlayer2 if modality == Modalities::ComputerVsComputer => models::Player::AIPlayer1,
        models::Player::AIPlayer2 if modality == Modalities::ComputerVsHuman => models::Player::Player1,
        // ??? needed to compile
        _ => models::Player::AIPlayer2
    }
}
