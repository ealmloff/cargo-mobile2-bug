
fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    dioxus_mobile_test::main().unwrap();
}
