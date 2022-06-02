
use std::{any::{Any, TypeId}, collections::HashMap};

use crate::Typed;


type BoxAny = Box<dyn Any + Send + Sync>;

pub trait Invoker<Req> {

    type Res;

    fn invoke(&self, context: &mut InvokeContext, req: Req) -> Self::Res;
    
}



#[derive(Debug)]
pub struct InvokeContext {
    data: HashMap<TypeId, BoxAny>
}

impl InvokeContext {

    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn with_context<I: 'static + Send + Sync>(&mut self, context: I) {
        let id = TypeId::of::<I>();
        self.data.insert(id, Box::new(context));
    }

}


impl Typed for InvokeContext {

    fn get<I: 'static>(&self) -> Option<&I> {
        let id = TypeId::of::<I>();
        self.data.get(&id).map(|a| a.downcast_ref()).flatten()
    }

    fn get_mut<I: 'static>(&mut self) -> Option<&mut I> {
        let id = TypeId::of::<I>();
        self.data.get_mut(&id).map(|a| a.downcast_mut()).flatten()
    }
}


#[derive(Debug)]
pub struct InvokerManager {
    invokers: HashMap<TypeId, BoxAny>
}

impl InvokerManager {

    pub fn new() -> Self {
        Self { invokers: HashMap::new() }
    }

    pub fn add_invoker<Req, I>(&mut self, invoker: I)
    where 
        I: Invoker<Req> + 'static + Send + Sync
    {
        let id = TypeId::of::<I>();
        self.invokers.insert(id, Box::new(invoker));
    }

}


impl Typed for InvokerManager {

    fn get<I: 'static>(&self) -> Option<&I> {
        let id = TypeId::of::<I>();
        self.invokers.get(&id).map(|a| a.downcast_ref()).flatten()
    }

    fn get_mut<I: 'static>(&mut self) -> Option<&mut I> {
        let id = TypeId::of::<I>();
        self.invokers.get_mut(&id).map(|a| a.downcast_mut()).flatten()
    }
}


#[cfg(test)]
mod test {

    use std::{any::Any, time::Duration, lazy::SyncLazy, sync::Mutex};

    use crate::Typed;
    use super::{Invoker, InvokeContext, InvokerManager};

    static INVOKER_MANAGER: SyncLazy<Mutex<InvokerManager>> = SyncLazy::new(|| {
        let manager = InvokerManager::new();
        Mutex::new(manager)
    });

    struct InvokerFoo;

    #[derive(Debug)]
    struct LocalContext {
        req_id: String,
        times: usize
    }

    impl Invoker<String> for InvokerFoo {

        type Res = Result<String, String>;

        fn invoke(&self, context: &mut InvokeContext, req: String) -> Self::Res {
            println!("InvokereFoo invoked with req: {}", req);
            let context_opt = context.get_mut::<LocalContext>();
            assert!(context_opt.is_some());
            let context = context_opt.unwrap();
            // add invoke times
            context.times += 1;
            Ok(context.req_id.to_owned())
        }
    }


    fn init() {
        INVOKER_MANAGER.lock().unwrap().add_invoker(InvokerFoo);
    }

    #[test]
    fn test_invoker() {
        init();
        let mut context = InvokeContext::new();

        let local_context = LocalContext { req_id: "req_id".to_owned(), times: 0 };
        context.with_context(local_context);

        let guard = INVOKER_MANAGER.lock().unwrap();
        let invoker = guard.get::<InvokerFoo>().unwrap();
        let res = invoker.invoke(&mut context, "this is req".to_owned());
        assert_eq!(1, context.get::<LocalContext>().unwrap().times);
        let res = guard.get::<InvokerFoo>().unwrap().invoke(&mut context, "req2".into());
        assert_eq!(2, context.get::<LocalContext>().unwrap().times);
    }



}