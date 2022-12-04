use super::query::Query;



pub trait GameEvent {}


type GameVentHanlder<T: GameEvent> = fn(Query, &T);




