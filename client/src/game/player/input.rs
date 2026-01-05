use bevy::prelude::*;

#[derive(Resource,Default)]
pub struct PlayerInput{
    //x component is forward and y direction is right and z is up
    pub movement : Vec3,
}