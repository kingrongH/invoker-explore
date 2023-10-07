use std::{fs::OpenOptions, io::Write, sync::Mutex};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use invoker_explore::{invoker_manager::{Invoker, InvokeContext, InvokerManager}, Typed};
use once_cell::sync::Lazy;


static INVOKER_MANAGER: Lazy<InvokerManager> = Lazy::new(|| {
    let mut manager = InvokerManager::new();
    manager.add_invoker(InvokerFoo);
    manager
});

static WITHOUT_IO_INVOKER_MANAGER: Lazy<InvokerManager> = Lazy::new(|| {
    let mut manager = InvokerManager::new();
    manager.add_invoker(InvokerBar);
    manager
});


static MUTEX_INVOKER_MANAGER: Lazy<Mutex<InvokerManager>> = Lazy::new(|| {
    let mut manager = InvokerManager::new();
    manager.add_invoker(InvokerFoo);
    Mutex::new(manager)
});

static INVOKER_FOO: InvokerFoo = InvokerFoo; 

static INVOKER_BAR: InvokerBar = InvokerBar;


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

struct InvokerBar;

impl Invoker<&str> for InvokerBar {

    type Res = Result<String, String>;

    fn invoke(&self, context: &mut InvokeContext, _req: &str) -> Self::Res {
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

fn dyn_mutex_invoke(input: &str) {
    let manager = MUTEX_INVOKER_MANAGER.lock().unwrap();
    let invoker = manager.get::<InvokerFoo>().unwrap();
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

fn dyn_without_io_invoke(input: &str) {
    let invoker = WITHOUT_IO_INVOKER_MANAGER.get::<InvokerBar>().unwrap();
    let mut context = InvokeContext::new();
    let local_context = LocalContext { req_id: "req_id".to_owned(), times: 0 };
    context.with_context(local_context);
    invoker.invoke(&mut context, input);
}

fn direct_without_io_invoke(input: &str) {
    let invoker = &InvokerBar;
    let mut context = InvokeContext::new();
    let local_context = LocalContext { req_id: "req_id".to_owned(), times: 0 };
    context.with_context(local_context);
    invoker.invoke(&mut context, input);
}

fn dyn_invoke_benchmark(c: &mut Criterion) {
    c.bench_function("dyn_invoke", |b| b.iter(|| dyn_invoke(black_box("input"))));
}

fn dyn_mutex_invoke_benchmark(c: &mut Criterion) {
    c.bench_function("dyn_mutex_invoke", |b| b.iter(|| dyn_mutex_invoke(black_box("input"))));
}

fn direct_invoke_benchmark(c: &mut Criterion) {
    c.bench_function("direct_invoke", |b| b.iter(|| direct_invoke(black_box("input"))));
}

fn dyn_without_io_invoke_benchmark(c: &mut Criterion) {
    c.bench_function("dyn_without_io_invoke", |b| b.iter(|| dyn_without_io_invoke(black_box("input"))));
}

fn direct_without_io_invoke_benchmark(c: &mut Criterion) {
    c.bench_function("direct_without_io", |b| b.iter(|| direct_without_io_invoke(black_box("input"))));
}

criterion_group!(benches, dyn_invoke_benchmark, dyn_mutex_invoke_benchmark, direct_invoke_benchmark, dyn_without_io_invoke_benchmark, direct_without_io_invoke_benchmark);
criterion_main!(benches);
