use http::{Request, Response, StatusCode, Version};

use crate::client::flow::CloseReason;
use crate::client::test::scenario::write_response;

use super::scenario::Scenario;

#[test]
fn reuse_without_send_body() {
    let scenario = Scenario::builder()
        .get("https://a.test")
        .response(Response::new(()))
        .recv_body("hello", false)
        .build();

    let flow = scenario.to_cleanup();

    assert!(!flow.must_close_connection());
}

#[test]
fn reuse_with_send_body() {
    let scenario = Scenario::builder()
        .post("https://a.test")
        .send_body("hi", false)
        .response(Response::new(()))
        .recv_body("hello", false)
        .build();

    let flow = scenario.to_cleanup();

    assert!(!flow.must_close_connection());
}

#[test]
fn reuse_without_recv_body() {
    let scenario = Scenario::builder()
        .head("https://a.test")
        .response(Response::new(()))
        .build();

    let flow = scenario.to_cleanup();

    assert!(!flow.must_close_connection());
}

#[test]
fn reuse_after_redirect() {
    let scenario = Scenario::builder()
        .get("https://a.test")
        .redirect(StatusCode::FOUND, "https://b.test")
        .build();

    let flow = scenario.to_cleanup();

    assert!(!flow.must_close_connection());
}

#[test]
fn close_due_to_http10() {
    let scenario = Scenario::builder()
        .request(
            Request::get("https://a.test")
                .version(Version::HTTP_10)
                .body(())
                .unwrap(),
        )
        .build();

    let flow = scenario.to_cleanup();
    let inner = flow.inner();
    assert_eq!(*inner.close_reason.first().unwrap(), CloseReason::Http10);

    assert!(flow.must_close_connection());
}

#[test]
fn close_due_to_client_connection_close() {
    let scenario = Scenario::builder()
        .get("https://a.test")
        .header("connection", "close")
        .build();

    let flow = scenario.to_cleanup();
    let inner = flow.inner();
    assert_eq!(
        *inner.close_reason.first().unwrap(),
        CloseReason::ClientConnectionClose
    );

    assert!(flow.must_close_connection());
}

#[test]
fn close_due_to_server_connection_close() {
    let scenario = Scenario::builder()
        .get("https://a.test")
        .response(
            Response::builder()
                .header("connection", "close")
                .body(())
                .unwrap(),
        )
        .build();

    let flow = scenario.to_cleanup();
    let inner = flow.inner();
    assert_eq!(
        *inner.close_reason.first().unwrap(),
        CloseReason::ServerConnectionClose
    );

    assert!(flow.must_close_connection());
}

#[test]
fn close_due_to_not_100_continue() {
    let scenario = Scenario::builder()
        .post("https://q.test")
        .header("expect", "100-continue")
        .send_body("hi", false)
        .build();

    let mut flow = scenario.to_await_100();

    let input = write_response(
        &Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(())
            .unwrap(),
    );
    flow.try_read_100(&input).unwrap();

    let inner = flow.inner();
    assert_eq!(
        *inner.close_reason.first().unwrap(),
        CloseReason::Not100Continue
    );
}

#[test]
fn close_due_to_close_delimited_body() {
    // no content-length or transfer-encoding
    let scenario = Scenario::builder().get("https://a.test").build();

    let flow = scenario.to_cleanup();
    let inner = flow.inner();
    assert_eq!(
        *inner.close_reason.first().unwrap(),
        CloseReason::CloseDelimitedBody
    );

    assert!(flow.must_close_connection());
}
