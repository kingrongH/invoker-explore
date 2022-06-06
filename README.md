

This is a test to see if we can define a flexible engouth `Invoker` Abstraction for rpc like framework

# explain


## Invoker

The abstraction of `Invoker` is like this which is simple and enough to use

```rust
pub trait Invoker<Req> {

    type Res;

    fn invoke(&self, context: &mut InvokeContext, req: Req) -> Self::Res;
    
}
```

## Context

The different type of impl of `Invoker` trait may have different type of context. So that the context here should not care too much about context detail for impls. It should be flexible and easy to use.

So we got this: store different type of context as `BoxAny`;

```rust
type BoxAny = Box<dyn Any + Send + Sync>;

#[derive(Debug)]
pub struct InvokeContext {
    data: HashMap<TypeId, BoxAny>
}
```

We can get different type of context by add utilities like below, and this enforce user try to use *NewType* for diferent type of context. 

```rust
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
```

And the same logic for `InvokerManager`


# Benches

```shell
cargo criterion
```

with *MacBook Pro (16-inch, 2019)*, *16 GB 2667 MHz DDR4* the bench shows as following

| bench | time |
| - | - |
| dyn_invoke | [37.717 us 37.936 us 38.169 us] |
| dyn_mutex_invoke | [38.563 us 39.101 us 39.786 us] |
| direct_invoke | [38.762 us 39.133 us 39.526 us] |
| dyn_without_io_invoke | [274.58 ns 276.68 ns 278.89 ns] |
| direct_without_io | [253.80 ns 255.18 ns 256.76 ns] |


from what's show above, with io operations there is very little difference between `dyn_invoke` and `direct_invoke`


# Furthermore 


1. protocol provider

Since we have a protocol agnostic invoker, we may provide a abstraction over `ProtocolProvider` trait, so that the user can have different protocol impl for the same invoker, without care too much about detail of the protocol itself.

2. Use `Provider` Api in std lib, rather than `HashMap<TypeId, BoxAny>`

there is actually a nice thing: https://github.com/rust-lang/rust/pull/91970

if this is stable, we may replace `HashMap<TypeId, BoxAny>` with `Provider` api stuff. 

or maybe better wait https://github.com/rust-lang/rust/issues/65991 land

3. once `Provider` Api has been merged add it to bench

