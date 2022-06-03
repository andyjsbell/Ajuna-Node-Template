// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use core::fmt::Display;

use crate::{self as pallet_ajuna_board, TurnBasedGame};
use codec::{Decode, Encode};
use dot4gravity::{Coordinates, Game, GamePhase, GameState, Side};
use scale_info::TypeInfo;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub mod mock;

pub struct Dot4GravityGame;

type MockAccountId = u32;
const MAX_PLAYERS: usize = 2;

pub struct Turn {}

#[derive(Clone, Debug, Decode, Eq, Encode, PartialEq, TypeInfo)]
pub enum Dot4GravityTurn {
	/// Drop a bomb movement
	DropBomb { position: Coordinates },
	/// Drop a stone movement
	DropStone { position: u8, side: Side },
}

impl TurnBasedGame for Dot4GravityGame {
	type Turn = Dot4GravityTurn;

	type Player = MockAccountId;

	type State = GameState;

	fn init(players: &[Self::Player]) -> Option<Self::State> {
		if players.len() != MAX_PLAYERS {
			return None
		};

		let game_state = Game::new_game(players[0], players[1]);
		Some(game_state)
	}

	fn play_turn(
		player: Self::Player,
		state: Self::State,
		turn: Self::Turn,
	) -> Option<Self::State> {
		let state = Game::check_winner_player(state);
		if state.winner.is_some() {
			return None
		}

		if !state.is_player_in_game(&player) {
			return None
		}

		if state.phase == GamePhase::Play && !state.is_player_turn(&player) {
			return None
		}

		match turn {
			Dot4GravityTurn::DropBomb { position } => {
				if let Ok(new_state) = Game::drop_bomb(state, position, player) {
					return Some(new_state)
				} else {
					return None
				}
			},
			Dot4GravityTurn::DropStone { position, side } => {
				if let Ok(new_state) = Game::drop_stone(state, player, side, position) {
					return Some(new_state)
				} else {
					return None
				}
			},
		}
	}

	fn is_finished(state: &Self::State) -> ajuna_common::Finished<Self::Player> {
		match state.winner {
			None => pallet_ajuna_board::Finished::No,
			Some(winner) => pallet_ajuna_board::Finished::Winner(winner),
		}
	}
}
