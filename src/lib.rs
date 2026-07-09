wit_bindgen::generate!({
    path: "wit",
    world: "http-webhook",
    generate_all,
});

use crate::exports::wasi::http::incoming_handler::Guest;
use crate::wasi::http::types::{IncomingRequest, ResponseOutparam};

pub struct MyWebhook;

// 3. 实现 Guest trait，这是 wit-bindgen 导出 handler 的标准方式
impl Guest for MyWebhook {
    fn handle(_request: IncomingRequest, _response_out: ResponseOutparam) {
        // 这里对接你的 Webhook 逻辑
        let _ = executor::block_on(async {
            // 你的异步逻辑
        });
    }
}

pub struct WebhookRequest {
    pub path: String,
    pub signature: String,
    pub timestamp: u64,
    pub body: Vec<u8>,
}

pub struct WebhookResponse {
    pub status: u16,
    pub content_type: String,
    pub body: String,
}

// 🛠️ 工业级轻量异步驱动器
pub mod executor {
    use std::future::Future;
    use std::task::{Context, Poll, Wake, Waker};
    use std::sync::Arc;

    struct DummyWaker;
    impl Wake for DummyWaker { fn wake(self: Arc<Self>) {} }

    /// 在同步的 WASI 0.2.0 线程上强行驱动异步的路由
    pub fn block_on<F: Future>(mut future: F) -> F::Output {
        let mut future = unsafe { std::pin::Pin::new_unchecked(&mut future) };
        let waker = Waker::from(Arc::new(DummyWaker));
        let mut cx = Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => {
                    // 在 WASI 0.2.0 的纯同步沙箱中，如果遇到了未准备好的 I/O
                    // 我们可以让出 CPU 时钟或通过轮询直接让它空转推进状态
                    std::hint::spin_loop(); 
                }
            }
        }
    }
}