use std::{convert::Infallible, process::Command};

use async_stream::try_stream;
use axum::{
    extract::{
        ws::{self, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, Response},
};

use axum::response::sse::{Event, KeepAlive, Sse};
use dioxus::prelude::*;
use futures::stream::Stream;

use crate::AppState;

pub fn example() -> Html<String> {
    let test = dioxus_ssr::render_lazy(rsx! {
        p {
            "clas": "hey",
            "hej!"
        }
    });
    Html(test)
}

pub fn get_distro() -> Html<String> {
    let cmd_stdout = Command::new("lsb_release")
        .arg("-id")
        .output()
        .expect("failed to execute process")
        .stdout;
    let distro_info = std::str::from_utf8(&cmd_stdout).unwrap().trim().split("\n");

    let doc = dioxus_ssr::render_lazy(rsx! {
        div {
            distro_info.map(|i| rsx!{ p {"{i}"} })
        }
    });
    Html(doc.to_string())
}

pub async fn get_date(State(state): State<AppState>) -> Html<String> {
    let mut rx = state.tx.subscribe();

    while let Ok(date) = rx.recv().await {
        let doc = dioxus_ssr::render_lazy(rsx!{
            p {"{date}"}
        });
        return Html(doc);
    }
    let doc = dioxus_ssr::render_lazy(rsx!{
        p {"failed to retrieve data"}
    });
    Html(doc)
}

pub async fn get_datetime_realtime(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|ws: WebSocket| async { get_date_realtime_stream(state, ws).await })
}

async fn get_date_realtime_stream(app_state: AppState, mut ws: WebSocket) {
    let mut rx = app_state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        let test = dioxus_ssr::render_lazy(rsx!{
            div {
                id: "date",
                "The current date/time (using Dioxus SSR) is {msg}"
            }
        });

        ws.send(ws::Message::Text(test.to_string())).await.unwrap();
    }
}

pub async fn get_datetime_realtime_sse(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut rx = state.tx.subscribe();

    Sse::new(try_stream! {
        while let Ok(msg) = rx.recv().await {
            let doc = dioxus_ssr::render_lazy(rsx!{
                div {
                    "The current date/time (SSE) is {msg}"
                }
            });
            let event = Event::default().event("curDate").data(doc.to_string());

            yield event;
        };
    })
    .keep_alive(KeepAlive::default())
}
