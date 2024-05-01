// Starting with night ww to attack a player at an index choosen by the wolf (TODO)
use crate::{
    param::Parameter,
    player::{HouseNr, Player},
    role::Role,
    state::State,
};

pub trait Action {
    type Params: Parameter;

    fn performable_by(player: &Player, state: &State) -> bool;

    fn from_params(params: Self::Params) -> Self;

    fn perform(self, actor: &mut Player, state: &mut State);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CubKill {
    target: HouseNr,
    role: Role,
}

impl Action for CubKill {
    type Params = (HouseNr, Role);

    fn performable_by(player: &Player, _: &State) -> bool {
        player.role() == Role::Cub
    }

    fn from_params(params: Self::Params) -> Self {
        CubKill {
            target: params.0,
            role: params.1,
        }
    }

    fn perform(self, actor: &mut Player, state: &mut State) {
        for player in state.get_players_at_mut(self.target) {
            if player.role() == self.role {
                player.kill();
                actor.set_role(Role::Werewolf);
            }
        }
    }
}