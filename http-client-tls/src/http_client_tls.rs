use rustls::{ClientConfig, crypto::aws_lc_rs};
use rustls_platform_verifier::BuilderVerifierExt;
use std::sync::{Arc, LazyLock};

static RUSTLS_TLS_CONFIG: LazyLock<ClientConfig> = LazyLock::new(|| {
    let provider = Arc::new(aws_lc_rs::default_provider());
    ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_platform_verifier()
        .with_no_client_auth()
});

pub fn tls_config() -> ClientConfig {
    RUSTLS_TLS_CONFIG.clone()
}
