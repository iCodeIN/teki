use crate::teki::ecs::components::*;
use crate::teki::ecs::resources::EnemyFormation;
use crate::teki::ecs::EnemyType;
use crate::teki::utils::consts::*;
use lazy_static::lazy_static;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use sdl2::rect::{Point, Rect};

lazy_static! {
    pub static ref POSITION_ZERO: Position = Position(Point::new(0, 0));
}
#[system]
pub fn spawn_enemy(#[resource] enemy_formation: &mut EnemyFormation, commands: &mut CommandBuffer) {
    if enemy_formation.done_appearance {
        return;
    }
    let mut enemies: Vec<Enemy> = Vec::new();

    for x in 0..X_COUNT {
        enemies.push(Enemy { enemy_type: EnemyType::Corgi, formation_index: x as u8 });
    }

    for enemy in enemies {
        let drawable = SpriteDrawable { sprite_name: CORGI_SPRITE, rect: Rect::new(5, 5, 40, 40) };
        commands.push((enemy, *POSITION_ZERO, drawable));
    }
    enemy_formation.done_appearance = true;
}

#[system]
pub fn update_enemy_formation(#[resource] enemy_formation: &mut EnemyFormation) {
    enemy_formation.update();
}

#[system(for_each)]
#[write_component(Position)]
pub fn move_enemy_formation(
    enemy: &mut Enemy,
    entity: &Entity,
    world: &mut SubWorld,
    #[resource] enemy_formation: &mut EnemyFormation,
) {
    let position = <&mut Position>::query().get_mut(world, *entity).unwrap();
    position.0 = enemy_formation.pos(&enemy.formation_index);
}
