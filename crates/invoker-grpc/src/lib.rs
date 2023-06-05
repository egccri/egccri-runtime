use invoker::{InvokerContext, InvokerExecutable};
use std::collections::HashMap;
use std::path::Path;

mod server;

#[derive(thiserror::Error, Debug)]
pub enum InvokerGrpcError {
    #[error("Grpc server start error, cause by: {0}")]
    GrpcServerStartError(#[from] tonic::transport::Error),
}

pub type RuntimeData = ();

/// Support a common rpc call with function service id.
pub struct GrpcInvoker {
    context: InvokerContext<Self>,
    /// Rpc bind address.
    addr: String,
    /// Function service id map to component id.
    components_router: HashMap<String, String>,
}

#[async_trait::async_trait]
impl InvokerExecutable for GrpcInvoker {
    type RuntimeData = RuntimeData;

    async fn run(&self) {
        let _ = server::start(self.addr.as_str()).await;
    }

    // call context, first, fetch component from registry, secondly, instantiate_pre
    async fn instantiate_pre() {
        todo!()
    }

    async fn execute(&self) {
        todo!()
    }
}
