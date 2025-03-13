# Axum Body Split

Tiny library to fix my annoyance with axum, where I sometimes just want a quick and dirty way to use an extractor implementing `FromRequest` twice.

## Usage

Usage is like of any other extractor, but you need to add your state type as a type parameter (because `FromRequest` has access to the state, so it needs to know what type it is)

Example without state:

```rs
SplitBody(Json(json), text, _): SplitBody<Json<RequestJson>, String, ()>
```

Example with state:

```rs
SplitBody(Json(json), text, _): SplitBody<Json<RequestJson>, String, AppState>
```

## Examples

check out [examples](examples/) for examples of how to use this
