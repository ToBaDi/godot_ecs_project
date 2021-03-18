use gdnative::prelude::*;

godot_init!(init);

fn init(handle: InitHandle) {
    handle.add_class::<Game>();
}

type GameObjectType = Object;

#[derive(NativeClass)]
#[inherit(GameObjectType)]
struct Game {
    world: legion::World,
    init: legion::Schedule,
    schedule: legion::Schedule,
    resources: legion::Resources,
}

#[methods]
impl Game {
    fn new(_owner: &GameObjectType) -> Self {
        let world = legion::World::default();
        let init = legion::Schedule::builder().build();
        let schedule = legion::Schedule::builder().build();
        let resources = legion::Resources::default();

        Game {
            world,
            init,
            schedule,
            resources,
        }
    }

    #[export]
    fn ready(&mut self, _owner: &GameObjectType) {
        self.init.execute(&mut self.world, &mut self.resources)
    }

    #[export]
    fn process(&mut self, _owner: &GameObjectType, _delta: f64) {
        self.schedule.execute(&mut self.world, &mut self.resources)
    }
}

mod res {
    use gdnative::core_types::Rid;
    pub struct Scenario(Rid);

    impl std::ops::Deref for Scenario {
        type Target = Rid;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Scenario {
        pub fn new(rid: Rid) -> Self {
            Scenario(rid)
        }
    }
}

mod sys {
    use legion::*;

    #[system]
    fn cube_init() {}
}
