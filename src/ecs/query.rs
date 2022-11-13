use super::Component;



pub struct Query {

}


pub struct QueryResult {

}


impl Query {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn with_component<T: Component>(&mut self) -> &mut Self {
        self
    }

    pub fn run(&self) -> QueryResult {
        QueryResult {  }
    }
}