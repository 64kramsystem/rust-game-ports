use std::any::Any;

use crate::prelude::*;

#[my_actor_based]
pub struct Goal {
    team: u8,
}

impl Goal {
    pub fn new(team: u8) -> Self {
        let x = HALF_LEVEL_W;
        let y = if team == 0 { 0. } else { LEVEL_H };
        let vpos = Vector2::new(x, y);

        let img_base = "goal";
        let img_indexes = vec![team];

        Self {
            img_base,
            img_indexes,
            vpos,
            team,
            anchor: Anchor::Center,
        }
    }
}

impl Targetable for Goal {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn active(&self, ball: &Ball) -> bool {
        //# Is ball within 500 pixels on the Y axis?
        (ball.vpos.y - self.vpos.y).abs() < 500.
    }
}
