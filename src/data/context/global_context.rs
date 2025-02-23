use crate::presentation::routes::Routes;

#[derive(Default)]
pub struct GlobalContext {
    route_stack: Vec<Routes>,
}

impl GlobalContext {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn route(&self) -> Option<&Routes> {
        self.route_stack.last()
    }
    pub fn route_push(&mut self, route: Routes) {
        self.route_stack.push(route);
    }
    pub fn route_replace(&mut self, route: Routes) {
        self.route_pop();
        self.route_push(route);
    }
    pub fn route_pop(&mut self) {
        self.route_stack.pop();
    }
    pub fn route_clear(&mut self) {
        self.route_stack.clear();
    }
}
