use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = <&Point>::query()
        .filter(component::<Player>());
    
    let mut enemies = <(Entity, &Point)>
}

// TODO: Collision detection chapter