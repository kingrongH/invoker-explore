
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



// /// totally typed client
// mod invoker4 {
//     use std::marker::PhantomData;


//     trait Transport<Req> {

//         type Response;

//         type Error;
        
//         type Futrue;

//         fn transport(&self, req: Req) -> Result<Self::Response, Self::Error>;

//     }

//     #[derive(Debug, thiserror::Error)]
//     pub enum InvokerError {
//         #[error("general error")]
//         GeneralError(#[from] anyhow::Error),
//     }


//     #[derive(Debug, Clone)]
//     pub struct Invoker<T, Req, Res> {
//         _marker: PhantomData<fn(Req) -> Res>,
//         transport: T,
//     }

//     impl<T, Req, Res> Invoker<T, Req, Res> 
//     where 
//         T: Transport<Req, Response = Res>
//     {


//         pub async fn invoke(&self) -> Result<Res, InvokerError> {



//         }

        
//     }

// }

