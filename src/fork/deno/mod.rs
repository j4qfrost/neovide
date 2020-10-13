use futures::future::{abortable, AbortHandle};
use deno_core::JsRuntime;
use deno_core::error::AnyError;
use deno_core::url::Url;
use tokio::runtime;
use tokio::runtime::Runtime;
use std::path::{PathBuf, Path};

pub struct Deno {
    runtime: Runtime,
}

struct ForkPermissions {
}

impl deno_fetch::FetchPermissions for ForkPermissions {
    fn check_net_url(&self, url: &Url) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_read(&self, p: &PathBuf) -> Result<(), AnyError> {
        Ok(())
    }
}

fn create_isolate(
  // files: Vec<PathBuf>,
) -> JsRuntime {
    let mut isolate = JsRuntime::new(Default::default());
    isolate.register_op("fetch", deno_core::json_op_async(deno_fetch::op_fetch::<ForkPermissions>));
    isolate.register_op("fetch_read", deno_core::json_op_async(deno_fetch::op_fetch_read));
    isolate.register_op("create_http_client", deno_core::json_op_sync(deno_fetch::op_create_http_client::<ForkPermissions>));
    isolate
}

impl Deno {
    pub fn new() -> Self {
        let mut runtime = runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        let hello = async move {
            let mut isolate = create_isolate();
            isolate.execute(
                "scripts/hello_world.js",
                include_str!("scripts/hello_world.js")
                ).unwrap();

            loop {
                                use std::time;
            let ten_millis = time::Duration::from_millis(500);
            std::thread::sleep(ten_millis);
            }

            // isolate.await
        };
        let (abort_hello, handle) = abortable(hello);

        runtime.spawn(abort_hello);
        println!("test");
                    use std::time;
            let ten_millis = time::Duration::from_millis(5000);
            std::thread::sleep(ten_millis);
            println!("tesasdasd");

        Self {
            runtime,
        }
    }
}