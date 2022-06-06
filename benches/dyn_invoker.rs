use std::{fs::OpenOptions, io::Write};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use invoker_explore::{invoker::{Invoker, InvokeContext, InvokerManager}, Typed};
use once_cell::sync::Lazy;


static INVOKER_MANAGER: Lazy<InvokerManager> = Lazy::new(|| {
    let mut manager = InvokerManager::new();
    manager.add_invoker(InvokerFoo);
    manager
});

static INVOKER_FOO: InvokerFoo = InvokerFoo; 


struct InvokerFoo;

#[derive(Debug)]
struct LocalContext {
    req_id: String,
    times: usize
}

impl Invoker<&str> for InvokerFoo {

    type Res = Result<String, String>;

    fn invoke(&self, context: &mut InvokeContext, req: &str) -> Self::Res {
        // simulate io operations
        let mut file = OpenOptions::new().append(true).create(true).open("./target/tmp/tmp.log").unwrap();
        file.write(req.as_bytes());

        let context_opt = context.get_mut::<LocalContext>();
        assert!(context_opt.is_some());
        let context = context_opt.unwrap();
        // add invoke times
        context.times += 1;
        Ok(context.req_id.to_owned())
    }
}


fn dyn_invoke(input: &str) {
    let invoker = INVOKER_MANAGER.get::<InvokerFoo>().unwrap();
    let mut context = InvokeContext::new();
    let local_context = LocalContext { req_id: "req_id".to_owned(), times: 0 };
    context.with_context(local_context);
    invoker.invoke(&mut context, input);
}

fn direct_invoke(input: &str) {
    let invoker = &INVOKER_FOO;
    let mut context = InvokeContext::new();
    let local_context = LocalContext { req_id: "req_id".to_owned(), times: 0 };
    context.with_context(local_context);
    invoker.invoke(&mut context, input);
}

fn dyn_invoke_benchmark(c: &mut Criterion) {
    c.bench_function("dyn_invoke", |b| b.iter(|| dyn_invoke(black_box("input"))));
}

fn direct_invoke_benchmark(c: &mut Criterion) {
    c.bench_function("direct_invoke", |b| b.iter(|| direct_invoke(black_box("input"))));
}

criterion_group!(benches, dyn_invoke_benchmark, direct_invoke_benchmark);
criterion_main!(benches);
