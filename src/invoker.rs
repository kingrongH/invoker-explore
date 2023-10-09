
mod inovker1 {
    use std::fmt::Debug;

    use futures_util::{future::BoxFuture, Future};
    use pin_project_lite::pin_project;


    #[derive(Debug, thiserror::Error)]
    pub enum InvokerError {
        #[error("general error")]
        GeneralError(#[from] anyhow::Error),
    }


    pin_project! {
        pub struct InvokerFuture<Res> {
            #[pin]
            fut: BoxFuture<'static, Result<Res, InvokerError>>
        }
    }

    impl<Res> InvokerFuture<Res> {

        /// create a new invoker
        pub fn new(fut: impl Future<Output = Result<Res, InvokerError>> + Send + Sync + 'static) -> Self {
            Self {
                fut: Box::pin(fut)
            }
        }

    }

    impl<Res> Future for InvokerFuture<Res> {

        type Output = Result<Res, InvokerError>;

        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
            let proj = self.project();
            proj.fut.poll(cx)
        }
    }




    /// inovker that represent a rpc invoke process
    pub trait Invoker<Req> {

        type Response;

        fn invoke(&self, req: Req) -> InvokerFuture<Self::Response>;

    }


    #[derive(Debug, Clone)]
    pub struct BaseInvoker {

    }

    impl Invoker<serde_json::Value> for BaseInvoker {

        type Response = serde_json::Value;

        fn invoke(&self, req: serde_json::Value) -> InvokerFuture<Self::Response> {
            let fut = async move {
                // message serialize 
                let s = serde_json::to_string(&req).map_err(|e| {
                    InvokerError::GeneralError(anyhow::Error::new(e))
                })?;

                // simulate  transport
                println!(" out: {}", s);

                let res = req.clone();
                Ok(res)
            };

            InvokerFuture::new(fut)
        }
    }



}



mod invoker2 {

    use std::future::Future;

    use futures_util::future::BoxFuture;
    use pin_project_lite::pin_project;
    use serde_json::Value;


    #[derive(Debug, thiserror::Error)]
    pub enum InvokerError {
        #[error("general error")]
        GeneralError(#[from] anyhow::Error),
    }


    pub trait Invoker<Req> {

        type Response;

        type Error;

        type Future: Future<Output = Result<Self::Response, Self::Error>>;

        fn invoke(&self, req: Req) -> Self::Future;

    }


    #[derive(Debug, Clone)]
    pub struct BaseInvoker {}

    pin_project! {
        pub struct BaseInvokerFuture<Res> {
            #[pin]
            fut: BoxFuture<'static, Result<Res, InvokerError>>
        }
    }

    impl<Res> BaseInvokerFuture<Res> {
        pub fn new(fut: impl Future<Output = Result<Res, InvokerError>> + Send + Sync + 'static) -> Self {
            Self {
                fut: Box::pin(fut),
            }
        }
    } 

    impl<Res> Future for BaseInvokerFuture<Res> {

        type Output = Result<Res, InvokerError>;

        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
            let proj = self.project();
            proj.fut.poll(cx)
        }
    }


    impl Invoker<Value> for BaseInvoker {

        type Response = Value;

        type Error = InvokerError;

        type Future = BaseInvokerFuture<Self::Response>;

        fn invoke(&self, req: Value) -> Self::Future {
            let fut = async move {
                // message serialize 
                let s = serde_json::to_string(&req).map_err(|e| {
                    InvokerError::GeneralError(anyhow::Error::new(e))
                })?;

                // simulate  transport
                println!(" out: {}", s);

                let res = req.clone();
                Ok(res)
            };

            BaseInvokerFuture::new(fut)
        }
    }

}


mod inovker3 {
    use std::{any::Any, fmt::Debug, future::Future};

    use futures_util::future::BoxFuture;
    use pin_project_lite::pin_project;
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    #[derive(Debug, thiserror::Error)]
    pub enum InvokerError {
        #[error("general error")]
        GeneralError(#[from] anyhow::Error),
    }


    #[derive(Debug, Clone)]
    pub struct Value {
        value: serde_json::Value
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ValueError {
        #[error("value encode error")]
        EncodeError(anyhow::Error),
        #[error("value decode error")]
        DecodeError(anyhow::Error)
    }

    impl Value {

        pub fn from<T: Serialize>(v: T) -> Result<Self, ValueError> {
            let v = serde_json::to_value(v).map_err(|e| {
                ValueError::EncodeError(anyhow::Error::new(e))
            })?;
            Ok(Self {
                value: v
            })
        }

        pub fn to<T: DeserializeOwned>(&self) -> Result<T, ValueError> {
            let res = serde_json::from_value(self.value.clone()).map_err(|e| {
                ValueError::DecodeError(anyhow::Error::new(e))
            })?;
            Ok(res)
        }

        pub fn get_inner(&self) -> &serde_json::Value {
            &self.value
        }

    }


    pin_project! {
        pub struct InvokerFuture<Res> {
            #[pin]
            fut: BoxFuture<'static, Result<Res, InvokerError>>
        }
    }

    impl<Res> InvokerFuture<Res> {

        /// create a new invoker
        pub fn new(fut: impl Future<Output = Result<Res, InvokerError>> + Send + Sync + 'static) -> Self {
            Self {
                fut: Box::pin(fut)
            }
        }

    }

    impl<Res> Future for InvokerFuture<Res> {

        type Output = Result<Res, InvokerError>;

        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
            let proj = self.project();
            proj.fut.poll(cx)
        }
    }


    pub trait Invoker {

        fn invoke(&self, req: Value) -> InvokerFuture<Value>;

    }


    #[derive(Debug, Clone)]
    pub struct BaseInvoker {

    }

    impl Invoker for BaseInvoker {

        fn invoke(&self, req: Value) -> InvokerFuture<Value> {
            let fut = async move {
                // message serialize 
                let s = serde_json::to_string(&req.get_inner()).map_err(|e| {
                    InvokerError::GeneralError(anyhow::Error::new(e))
                })?;

                // simulate  transport
                println!(" out: {}", s);

                let res = req.clone();
                Ok(res)
            };

            InvokerFuture::new(fut)
        }

    }


}



mod invoker4 {
    use std::marker::PhantomData;

    use futures_util::Future;


    pub trait Transport<Req> {

        type Response;

        type Error;
        
        type Futrue: Future<Output = Result<Self::Response, Self::Error>>;

        fn transport(&self, req: Req) -> Self::Futrue;

    }

    #[derive(Debug, thiserror::Error)]
    pub enum InvokerError {
        #[error("general error")]
        GeneralError(#[from] anyhow::Error),
    }


    #[derive(Debug, Clone)]
    pub struct Invoker<T, Req, Res> {
        _marker: PhantomData<fn(Req) -> Res>,
        transport: T,
    }

    impl<T, Req, Res> Invoker<T, Req, Res> 
    where 
        T: Transport<Req, Response = Res>,
        T::Error: Into<InvokerError>
    {


        pub async fn invoke(&self, req: Req) -> Result<Res, InvokerError> {
            self.transport.transport(req).await.map_err(|e| e.into())
        }
        
    }


}


mod invoker5 {

    use std::{collections::HashMap, any::{TypeId, Any}, future::Future, marker::PhantomData};

    use futures_util::future::BoxFuture;
    use pin_project_lite::pin_project;
    use serde_json::Value;


    pub trait Encoder<V> {

        type Message;
        type Error;

        fn encode(&self, value: V) -> Result<Self::Message, Self::Error>;

    }

    pub trait Decoder<M> {

        type Value;
        type Error;

        fn decode(&self, message: M) -> Result<Self::Value, Self::Error>;

    }



    pub trait Message: Sized {

        type MsgType;

        type Encoder: Encoder<Self>;

        type Decoder: Decoder<Self::MsgType>;
        
    }


    #[derive(Debug, Clone)]
    pub struct MethodDefInfo{
        map: HashMap<String, String>,
    }

    impl MethodDefInfo {

        /// create new method def info
        pub fn new() -> Self {
            Self { map: HashMap::new() }
        }

    }


    pub trait MethodDef {

        const NAME: &'static str;

        type Request: Message + Send + Sync + 'static;
        type Response: Message + Send + Sync + 'static;

        fn get_method_def_info(&self) -> &MethodDefInfo;

    }

    trait Transport<T> {

        type Response;
        type Error;
        type Future: Future<Output = Result<Self::Response, Self::Error>>;

        


    }


    trait Invoker<M: MethodDef> {

        type Error: Into<anyhow::Error>;

        type Future: Future<Output = Result<M::Response, Self::Error>> + Send + 'static;

        // invoke 
        fn invoke(&self, req: M::Request) -> Self::Future;
    }



    /// impl
   
    pub struct JsonEncoder;
    pub struct JsonDecoder<V> {
        _m: PhantomData<V>
    }

    impl<T> Encoder<T> for JsonEncoder 
    where 
        T: serde::Serialize
    { 
        type Message = String;
        type Error = serde_json::Error;

        fn encode(&self, value: T) -> Result<Self::Message, Self::Error> {
            serde_json::to_string(&value)
        } 
    }

    impl<M, V> Decoder<M> for JsonDecoder<V>
    where
        M: AsRef<str>,
        V: serde::de::DeserializeOwned
    {

        type Value = V;
        type Error = serde_json::Error;

        fn decode(&self, message: M) -> Result<Self::Value, Self::Error> {
            serde_json::from_str(message.as_ref())
        }
    }



    impl Message for Value { 
        type MsgType = String; 
        type Encoder = JsonEncoder; 
        type Decoder = JsonDecoder<Self>; 
    }

    pub struct GenericMethod;
    impl MethodDef for GenericMethod {

        const NAME: &'static str = "genericInvoke";

        type Request = Value;

        type Response = Value;

        fn get_method_def_info(&self) -> &MethodDefInfo {
            todo!()
        }
    }



    #[derive(Debug, thiserror::Error)]
    pub enum InvokerError {
        #[error("general error")]
        GeneralError(#[from] anyhow::Error),
    }

    pub struct BaseJsonInvoker;

    pin_project! {
        pub struct InvokerFut {
            #[pin]
            fut: BoxFuture<'static, Result<Value, InvokerError>>
        }
    }

    impl Future for InvokerFut {

        type Output = Result<Value, InvokerError>;

        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
            let proj = self.project();
            proj.fut.poll(cx)
        }
    }



    impl Invoker<GenericMethod> for BaseJsonInvoker {

        type Error = InvokerError;

        type Future = InvokerFut;

        fn invoke(&self, req: <GenericMethod as MethodDef>::Request) -> Self::Future {

            let fut = async move {
                // message serialize
                let encoder = JsonEncoder;          
                let s = encoder.encode(req).map_err(|e| {
                    InvokerError::GeneralError(anyhow::Error::new(e))
                })?;

                // io
                println!("io out {}", s);

                // message deserialize
                let decoder = JsonDecoder { _m: PhantomData };
                decoder.decode(s).map_err(|e| {
                    InvokerError::GeneralError(anyhow::Error::new(e))
                })
            };

            InvokerFut { fut: Box::pin(fut) }
        }
    }





}

