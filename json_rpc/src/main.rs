use serde::{Serialize,Deserialize};
use jsonrpc_http_server::{AccessControlAllowOrigin,DomainsValidation,RestApi,ServerBuilder};
use jsonrpc_core::{futures::future, BoxFuture, IoHandler, IoHandlerExtension, Result};
use jsonrpc_derive::rpc;

// #[rpc]
// pub trait Rpc<A> {
// 	/// Adds two numbers and returns a result
// 	#[rpc(name = "add")]
// 	fn add(&self, a: u64, b: u64) -> Result<u64>;

//     #[rpc(name = "sendtransaction")]
//     fn sendtransaction(&self,txn : A) -> Result<String>;

// }

// pub struct RpcImpl;

// #[derive(Debug, Deserialize, Serialize,Clone)]
// struct Transaction {
//     balance: f64,
//     from: String,
//     to: String,
// }

// impl Rpc<Transaction> for RpcImpl {
// 	fn add(&self, a: u64, b: u64) -> Result<u64> {
// 		Ok(a + b)
// 	}

//     fn sendtransaction(&self,txn : Transaction) -> Result<String> {
//         println!("Transaction received : {:?}",txn);
//         Ok("hello".to_owned())
//     }
// }

#[rpc(server)]
pub trait Rpc<One, Two, Three> {
	/// Get One type.
	#[rpc(name = "getOne")]
	fn one(&self) -> Result<One>;

	/// Adds two numbers and returns a result
	#[rpc(name = "setTwo")]
	fn set_two(&self, a: Two) -> Result<()>;

	#[rpc(name = "getThree")]
	fn get_three(&self) -> Result<Three>;

	/// Performs asynchronous operation
	#[rpc(name = "beFancy")]
	fn call(&self, a: One) -> BoxFuture<Result<String>>;
}

struct RpcImpl;

#[derive(Serialize, Deserialize)]
struct InAndOut {
	foo: u64,
}
#[derive(Deserialize)]
struct In {}
#[derive(Serialize)]
struct Out {}

impl Rpc<InAndOut, In, Out> for RpcImpl {
	fn one(&self) -> Result<InAndOut> {
		Ok(InAndOut { foo: 1u64 })
	}

	fn set_two(&self, _x: In) -> Result<()> {
		Ok(())
	}

	fn get_three(&self) -> Result<Out> {
		Ok(Out {})
	}

	fn call(&self, _num: InAndOut) -> BoxFuture<Result<String>> {
        Box::pin(async {
            // Asynchronous logic goes here...
            
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            // Return the result of the async operation
            Ok("Async method completed".to_owned())
        })
	}
}


fn main() {
	let mut io = jsonrpc_core::IoHandler::new();
	io.extend_with(RpcImpl.to_delegate());
    
    let _server: jsonrpc_http_server::Server = ServerBuilder::new(io)
    .threads(3)
    .rest_api(RestApi::Unsecure)
    .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
    .start_http(&"127.0.0.1:3039".parse().unwrap())
    .expect("Unable to start RPC server");

    _server.wait();
}