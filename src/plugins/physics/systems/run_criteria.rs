use std::{any::TypeId, borrow::Cow};

use bevy::{
    core::FixedTimestep,
    ecs::{ShouldRun, SystemId, ThreadLocalExecution, TypeAccess},
    prelude::*,
};

use crate::common::resources::PausePhysics;

pub struct RunCriteria {
    time_step: FixedTimestep,
    system_id: SystemId,
    resource_access: TypeAccess<TypeId>,
}

impl RunCriteria {
    pub fn from(time_step: FixedTimestep) -> Self {
        Self {
            time_step,
            system_id: SystemId::new(),
            resource_access: Default::default(),
        }
    }
}

impl System for RunCriteria {
    type In = ();

    type Out = ShouldRun;

    fn name(&self) -> std::borrow::Cow<'static, str> {
        Cow::Borrowed(std::any::type_name::<RunCriteria>())
    }

    fn id(&self) -> bevy::ecs::SystemId {
        self.system_id
    }

    fn update(&mut self, _world: &World) {}

    fn archetype_component_access(&self) -> &bevy::ecs::TypeAccess<bevy::ecs::ArchetypeComponent> {
        self.time_step.archetype_component_access()
    }

    fn resource_access(&self) -> &bevy::ecs::TypeAccess<std::any::TypeId> {
        &self.resource_access
    }

    fn thread_local_execution(&self) -> bevy::ecs::ThreadLocalExecution {
        ThreadLocalExecution::Immediate
    }

    unsafe fn run_unsafe(
        &mut self,
        _input: Self::In,
        world: &World,
        resources: &Resources,
    ) -> Option<Self::Out> {
        let pause_physics = resources.get::<PausePhysics>().unwrap();
        if pause_physics.0 {
            Some(ShouldRun::No)
        } else {
            self.time_step.run_unsafe((), world, resources)
        }
    }

    fn run_thread_local(&mut self, _world: &mut World, _resources: &mut Resources) {}

    fn initialize(&mut self, world: &mut World, resources: &mut Resources) {
        self.resource_access.add_read(TypeId::of::<PausePhysics>());
        self.time_step.initialize(world, resources);
        self.resource_access.union(self.time_step.resource_access());
    }
}
