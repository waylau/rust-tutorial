use tracing::info;
use tracing_subscriber;

fn main() {
    //安装基于RUST_LOG环境变量配置的全局收集器。
    tracing_subscriber::fmt::init();

    let number_of_yaks = 3;

    //这将在任何跨度之外创建一个新事件
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = 4;
    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
}
