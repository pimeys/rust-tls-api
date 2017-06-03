extern crate tls_api;
extern crate tls_api_native_tls;

#[test]
fn test_google() {
    tls_api::impl_test::test_google::<tls_api_native_tls::TlsConnector>();
}

#[test]
fn connect_bad_hostname() {
    tls_api::impl_test::connect_bad_hostname::<tls_api_native_tls::TlsConnector>();
}

#[test]
fn connect_bad_hostname_ignored() {
    tls_api::impl_test::connect_bad_hostname_ignored::<tls_api_native_tls::TlsConnector>();
}

#[test]
fn server() {
    tls_api::impl_test::server::<
        tls_api_native_tls::TlsConnector,
        tls_api_native_tls::TlsAcceptor>();
}