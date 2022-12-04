use std::{any::{Any, TypeId}, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use super::{command_buffer::CommandBuffer, query::Query, world::World};

pub trait GameEvent {}

type GameEventHanlder<T: GameEvent> = fn(&T, Query, &mut CommandBuffer);

trait EventHandlerStorage {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn is_empty(&self) -> bool;
    fn remove_any(&mut self, type_id: &TypeId);
}

type GameHandlerVec<T> = Vec<GameEventHanlder<T>>;

impl<T: GameEvent + 'static> EventHandlerStorage for GameHandlerVec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn remove_any(&mut self, type_id: &TypeId) {
        todo!()
    }
}

pub struct WorldEvents {
    handlers: Rc<RefCell<HashMap<TypeId, Box<dyn EventHandlerStorage>>>>,
}

pub struct EventEmitter<'a> {
    handlers: Rc<RefCell<HashMap<TypeId, Box<dyn EventHandlerStorage>>>>,
    world: &'a World<'a>,
}

pub trait WorldEventSubscriber {
    fn subscribe<T: GameEvent + 'static>(&mut self, handler: GameEventHanlder<T>);
}

pub trait WorldEventEmmiter {
    fn emit<T: GameEvent + 'static>(&self, event: T, cmd_buffer: &mut CommandBuffer);
}

impl WorldEvents {
    pub fn new() -> Self {
        Self {
            handlers: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn emiter<'a>(&self, world: &'a World<'a>) -> EventEmitter<'a> {
        EventEmitter::<'a> {
            handlers: self.handlers.clone(),
            world
        }
    }
}


impl WorldEventSubscriber for WorldEvents {
        fn subscribe<T: GameEvent + 'static>(&mut self, handler: GameEventHanlder<T>) {
        let id = TypeId::of::<T>();

        self.handlers
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| Box::new(Vec::<GameEventHanlder<T>>::new()));

        self.handlers
            .borrow_mut()
            .get_mut(&id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Vec<GameEventHanlder<T>>>()
            .unwrap()
            .push(handler);
    }
}

impl <'a> WorldEventEmmiter for EventEmitter<'a> {
    fn emit<T: GameEvent + 'static>(&self, event: T, cmd_buffer: &mut CommandBuffer) {
        let id = TypeId::of::<T>();
        let handlers = self
            .handlers
            .borrow()
            .get(&id)
            .unwrap()
            .as_any()
            .downcast_ref::<GameHandlerVec<T>>()
            .unwrap();
        for handler in handlers {
            handler(&event, self.world.query(), cmd_buffer);
        }
    }

}

#[cfg(test)]
mod test {
    use crate::ecs::{command_buffer::CommandBuffer, query::Query, world::World};

    use super::WorldEvents;
    use super::WorldEventEmmiter;
    use super::WorldEventSubscriber;

    #[derive(ecs_macro::GameEvent)]
    struct SomethingHappend;

    fn handle_something_happend(
        event: &SomethingHappend,
        query: Query,
        cmd_buffer: &mut CommandBuffer,
    ) {
    }

    #[test]
    fn register_handler() {
        let world = World::new();
        let mut events = WorldEvents::new(&world);
        events.subscribe::<SomethingHappend>(handle_something_happend);
    }

    #[test]
    fn emit_event() {
        let world = World::new();
        let mut cmd_buffer = CommandBuffer::new();
        let mut events = WorldEvents::new(&world);
        events.subscribe::<SomethingHappend>(handle_something_happend);
        events.emit(SomethingHappend {}, &mut cmd_buffer);
    }
}
