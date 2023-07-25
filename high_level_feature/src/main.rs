struct Param(Vec<String>);

struct Route(String);

struct Status(String);

struct Context {
    param: Vec<String>,
    route: String,
    header: String,
}

trait FromContext {
    fn resolve(context: &Context) -> Self;
}

impl FromContext for Param {
    fn resolve(context: &Context) -> Self {
        Self(context.param.clone())
    }
}

impl FromContext for Route {
    fn resolve(context: &Context) -> Self {
        Self(context.route.clone())
    }
}

impl FromContext for Status {
    fn resolve(context: &Context) -> Self {
        Self(context.header.clone())
    }
}

fn handler_one(Param(p): Param) {
    println!("handle_one, param:{:?}", p);
}

fn handler_two(Route(r): Route, Param(p): Param) {
    println!("handler_two, route: {}, param:{:?}", r, p);
}

fn handler_two_swap(Param(p): Param, Route(r): Route) {
    println!("handler_two_swap, route: {}, param:{:?}", r, p);
}

fn handler_three(Route(r): Route, Param(p): Param, Status(s): Status) {
    println!("handler_three, route: {}, param:{:?}, status: {}", r, p, s);
}

trait Handler<T> {
    fn call(self, c: &Context);
}

impl<T1, F> Handler<T1> for F
    where F: Fn(T1), T1: FromContext
{
    fn call(self, c: &Context) {
        (self)(T1::resolve(c));
    }
}

impl<T1, T2, F> Handler<(T1, T2)> for F
    where F: Fn(T1, T2), T1: FromContext, T2: FromContext
{
    fn call(self, c: &Context) {
        (self)(T1::resolve(c), T2::resolve(c));
    }
}

impl<T1, T2, T3, F> Handler<(T1, T2, T3)> for F
    where F: Fn(T1, T2, T3), T1: FromContext, T2: FromContext, T3: FromContext
{
    fn call(self, c: &Context) {
        (self)(T1::resolve(c), T2::resolve(c), T3::resolve(c));
    }
}

fn trigger<H, T>(c: &Context, h: H)
    where H: Handler<T>
{
    h.call(c);
}


fn main() {
    let c = Context {
        param: vec!["123".to_string(), "456".to_string()],
        route: "asdf".to_string(),
        header: "123".to_string(),
    };

    trigger(&c, handler_one);
    trigger(&c, handler_two);
    trigger(&c, handler_two_swap);
    trigger(&c, handler_three);
}
