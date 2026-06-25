# Bevy 0.18 → 0.19 migration notes

This fork branch (`feat/bevy-0.19`) migrates the published crates of
YarnSpinner-Rust from Bevy 0.18 to Bevy 0.19.

## Scope

In scope (migrated, building, tests green):

- `bevy_yarnspinner` (crates/bevy_plugin)
- `bevy_yarnspinner_example_dialogue_view` (crates/example_dialogue_view)
- Pure crates (`yarnspinner`, `runtime`, `compiler`, `core`, `codegen`,
  `internal_shared`) — they only depend on `bevy_platform` and built without
  source changes after the version bump.

Out of scope for this pass (excluded from `default-members`, still in
`members` for future work):

- `demo/` — uses many Bevy renderer/scene APIs that were reshaped in 0.19
  (`SceneRoot`, `Hdr`, `Sprite3d`, `DirectionalLight::shadows_enabled`,
  `Bundle` derivation for tuples containing `Sprite3d`, `AlphaMode` reexport
  path, ...). 15 errors at last count; mechanical but tedious. See follow-up.
- `examples/bevy_yarnspinner` — same family of issues as `demo/`.

`examples/yarnspinner_without_bevy` is unaffected and stays in
`default-members`.

## Source changes

All Cargo manifests bumped from `bevy = "0.18"`/`bevy_platform = "0.18"` to
`"0.19"`.

Source changes were limited to:

1. `SystemState::get_mut` now returns `Result`. Call sites updated with `?`
   (in fallible system fns) or `.unwrap()` (in tests that drive the world
   directly).
2. `Assets::get_mut` now returns `AssetMut<'_, T>` (a smart-pointer wrapper)
   rather than `&mut T`. Receiving bindings need `let mut`, and code that
   previously held an immutable borrow into the asset across a mutating call
   needs to clone the borrowed data first (see
   `tests/test_strings_tables.rs`).
3. `Font::try_from_bytes(Vec<u8>) -> Result<Font, _>` was replaced by
   `Font::from_bytes(Vec<u8>) -> Font`.
4. `TextFont.font` is now `FontSource` (with `From<Handle<Font>>`) and
   `TextFont.font_size` is `FontSize` (with `From<f32>`). Existing literals
   updated with `.into()`.
5. Deprecation cleanups: `SystemCondition::and` → `and_then`,
   `AssetServer::load_untyped` → `load_builder().load_untyped`.

## Test status

- `cargo build` (default members): clean.
- `cargo test -p bevy_yarnspinner -- --skip panics_on_localization_without_line_ids_in_production`:
  66/66 passing.
- The skipped `should_panic` test aborts with
  `fatal runtime error: failed to initiate panic, error 5` inside a Bevy
  worker thread on this host. The same failure occurs on upstream
  `0c851cee` (pre-migration), so this is not a regression of the migration.
- `dialogue_tests` in the `yarnspinner` crate have several pre-existing
  fixture-path failures on upstream `0c851cee`; not touched here.

## Follow-up

- Port `demo/` and `examples/bevy_yarnspinner` to 0.19, then put them back
  into `default-members`.
- Investigate the pre-existing `should_panic` runtime-abort and the
  `dialogue_tests` fixture-path issues; consider upstreaming fixes.
