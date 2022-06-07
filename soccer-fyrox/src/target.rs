use crate::prelude::*;

// Stupid simple workaround for the source project duck typing.

#[derive(Clone, Copy)]
pub enum Target {
    None,
    Player(Handle<Player>),
    Goal(Handle<Goal>),
}

impl Target {
    pub fn is_goal(&self) -> bool {
        match self {
            Self::Player(_) => false,
            Self::Goal(_) => true,
            Self::None => panic!(),
        }
    }

    pub fn is_player(&self) -> bool {
        match self {
            Self::Player(_) => true,
            Self::Goal(_) => false,
            Self::None => panic!(),
        }
    }

    pub fn load<'a>(&self, pools: &'a Pools) -> &'a dyn Targetable {
        match self {
            Self::Player(handle) => pools.players.borrow(*handle),
            Self::Goal(handle) => pools.goals.borrow(*handle),
            Self::None => panic!(),
        }
    }

    // There's no trivial solution to this - instantiating each variant with the respective pool is
    // a nice idea, but requires either Rc's, that pollute the program with borrow()'s, or references,
    // which pollute the program with lifetimes.
    // Alternatively, players and goals could be stored in a single pool under a single trait, although
    // a mixed Pool type should be implemented (it's farly easy), otherwise, all the borrows require
    // downcasting (from Any), which is, again, very polluting.
    //
    pub fn vpos(&self, pools: &Pools) -> Vector2<f32> {
        self.load(pools).vpos()
    }

    pub fn active(&self, pools: &Pools, ball: &Ball) -> bool {
        self.load(pools).active(ball)
    }
}
