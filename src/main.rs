fn main() {
    #[cfg(feature = "sentry")]
    let _guard = sentry::init((
        option_env!("SENTRY_DSN"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    // Sentry will capture this
    panic!("Everything is on fire!");
}
