use gdnative::prelude::*;

godot_init!(init);

fn init(handle: InitHandle) {
    handle.add_class::<Game>();
}

type GameObjectType = Object;

#[derive(NativeClass)]
#[inherit(GameObjectType)]
struct Game {}

#[methods]
impl Game {
    fn new(_owner: &GameObjectType) -> Self {
        Game {}
    }
}
