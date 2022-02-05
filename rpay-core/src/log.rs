use tracing_subscriber::FmtSubscriber;

pub fn setup_trace() {
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::TRACE)
    //     .finish();
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}