use glib::prelude::*;

#[test]
#[ignore] // Requires actual GPU backend
fn test_context_creation() {
    let ctx = Context::builder().build().expect("Context creation failed");
    assert_eq!(ctx.device.info.name, "Null Device");
}
