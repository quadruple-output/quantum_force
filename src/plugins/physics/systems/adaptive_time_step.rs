use super::super::resources::TimeStepState;
use bevy::{
    ecs::{ArchetypeComponent, ShouldRun, SystemId, ThreadLocalExecution, TypeAccess},
    prelude::*,
};
use std::{any::TypeId, borrow::Cow, time::Duration};

pub struct AdaptiveTimeStep {
    max_dt: Duration,
    accumulator: Duration,
    looping: bool,
    system_id: SystemId,
    resource_access: TypeAccess<TypeId>,
    archetype_access: TypeAccess<ArchetypeComponent>,
}

impl Default for AdaptiveTimeStep {
    fn default() -> Self {
        let max_step_duration = dbg!(Duration::from_millis(1000 / 200)); // at least 200 steps/second
        Self {
            max_dt: max_step_duration,
            system_id: SystemId::new(),
            accumulator: Default::default(),
            looping: false,
            resource_access: Default::default(),
            archetype_access: Default::default(),
        }
    }
}

impl AdaptiveTimeStep {
    pub fn update(&mut self, time: &Time, state: &mut TimeStepState) -> ShouldRun {
        if !self.looping {
            self.accumulator += time.delta();
        } else {
            // we don't know the step duration in advance, so we can
            // only take the duration from the previous run:
            self.accumulator -= state.get_step_duration();
            // the result will never be negative but the last iteration of a loop
            // will bring the accumulator to exactly zero.
        }

        if self.accumulator > Duration::default() {
            state.initialize_step(self.accumulator.min(self.max_dt));
            self.looping = true;
            ShouldRun::YesAndLoop
        } else {
            self.looping = false;
            ShouldRun::No
        }
    }
}

impl System for AdaptiveTimeStep {
    type In = ();
    type Out = ShouldRun;

    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(std::any::type_name::<AdaptiveTimeStep>())
    }

    fn id(&self) -> SystemId {
        self.system_id
    }

    fn update(&mut self, _world: &World) {}

    fn archetype_component_access(&self) -> &TypeAccess<ArchetypeComponent> {
        &self.archetype_access
    }

    fn resource_access(&self) -> &TypeAccess<TypeId> {
        &self.resource_access
    }

    fn thread_local_execution(&self) -> ThreadLocalExecution {
        ThreadLocalExecution::Immediate
    }

    unsafe fn run_unsafe(
        &mut self,
        _input: Self::In,
        _world: &World,
        resources: &Resources,
    ) -> Option<Self::Out> {
        let time = resources.get::<Time>().unwrap();
        let mut state = resources.get_mut::<TimeStepState>().unwrap();
        let result = self.update(&time, &mut state);
        Some(result)
    }

    fn run_thread_local(&mut self, _world: &mut World, _resources: &mut Resources) {}

    fn initialize(&mut self, _world: &mut World, _resources: &mut Resources) {
        self.resource_access.add_read(TypeId::of::<Time>());
        self.resource_access
            .add_write(TypeId::of::<TimeStepState>());
    }
}
