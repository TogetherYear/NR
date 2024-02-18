use std::thread;

use napi::{
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
    JsFunction, Result,
};

#[napi(js_name = "CreateStaticFileServe")]
pub fn CreateStaticFileServe(path: String, onOpen: JsFunction) -> Result<()> {
    let tsfn: ThreadsafeFunction<(), ErrorStrategy::CalleeHandled> =
        onOpen.create_threadsafe_function(0, |_ctx| Ok(String::from("").into()))?;
    thread::Builder::new()
        .name(String::from("HttpServe"))
        .spawn(move || {
            let serve = file_serve::ServerBuilder::new(&path).port(8676).build();
            tsfn.call(Ok(()), ThreadsafeFunctionCallMode::Blocking);
            serve.serve().unwrap();
        })
        .unwrap();
    Ok(())
}
