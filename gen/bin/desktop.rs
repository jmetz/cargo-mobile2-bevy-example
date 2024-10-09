fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    hello_bevy::main();
}
