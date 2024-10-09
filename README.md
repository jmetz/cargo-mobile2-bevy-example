# cargo-mobile2 bevy example

This is a modified version of [NiklasEi](https://github.com/NiklasEi)'s 
excellent [`bevy_game_tempalate`](https://github.com/NiklasEi/bevy_game_template),
to use [`cargo-mobile2`](https://github.com/tauri-apps/cargo-mobile2).
Note that the official `bevy` template for that tool does not
work yet; this repo is a merger between working `cargo-mobile2` templates 
and `bevy_game_template`.  

It has been simplified in structure, to a single `lib.rs` for now, and 
the `ActionPlugin` system was replaced by 
[`Event`](https://docs.rs/bevy/latest/bevy/ecs/index.html#events)s. See 
[here](https://github.com/NiklasEi/bevy_game_template/issues/122) for a
 discussion about that.

## Screenshot (Android arm64)

<img src="screenshot.jpg" alt="screenshot" width="200"/>
