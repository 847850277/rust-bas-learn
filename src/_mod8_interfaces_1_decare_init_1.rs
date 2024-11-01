use std::future::Future;

trait Printable {
    fn print(&self);
}
pub(crate) fn test() {

    //let printable = Printable{}; //<-Error

}

trait EndPoint {
    fn handle(&self) -> impl Future + Send;
}


struct Route{

}

impl EndPoint for Route {
    fn handle(&self) -> impl Future + Send {
        async {
            println!("Route handle");
        }
    }
}


struct RouteMethod{

}

impl EndPoint for RouteMethod {
    fn handle(&self) -> impl Future + Send {
        async {
            println!("RouteMethod handle");
        }
    }
}


pub(crate) async fn test_2() {

    let route = Route{};
    let route_method = RouteMethod{};
    let route_future = route.handle();
    let route_method_future = route_method.handle();
    route_future.await;
    route_method_future.await;
    println!("Route and RouteMethod handle");



}

pub(crate) async fn test_3() {
    todo!()
}