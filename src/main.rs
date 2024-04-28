#![allow(non_snake_case)]
mod client;
mod evil_env;
mod model;

use std::io::Read;

use client::{get_data, send_query};
use dioxus::prelude::*;
use model::{TableView, User, Users};
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/", Menu)]
    Menu {},
    #[route("/edit", Editor)]
    Editor {},
    #[route("/view/:filter", View)]
    View { filter: String },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="main.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "w-screen h-screen bg-gray-50",
            Router::<Route> {}
        }
    }
}

#[component]
fn Menu() -> Element {
    rsx! {
            div {
                class: "flex flex-col items-center justify-center h-screen",
                header {
                    class: "m-8 text-2xl",
                    "Welcome to KiggyMetric"
                }
                div {
                    class: "flex flex-col space-y-4",
                    button { class: "px-6 py-3 border border-black text-black", Link { class: "text-lg text-black", to: Route::View { filter: "kiggysite".to_owned()  }, "View Kiggy's site metrics" } },
                    button { class: "px-6 py-3 border border-black text-black", Link { class: "text-lg text-black", to: Route::View { filter: "faycarsons".to_owned()  }, "View Fay's site metrics" } },
                    button { class: "px-6 py-3 border border-black text-black", Link { class: "text-lg text-black", to:  Route::View { filter: "all".to_owned()  }, "View all"} },
                    button { class: "px-6 py-3 border border-black text-black", Link { class: "text-lg text-black", to: Route::Editor {}, "Query database" } }
                }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        div {
            class: "w-full my-2",
            button {
                class: "mx-2 px-2 border border-black text-black",
                Link {
                    to: Route::Menu {},
                    "Menu"
                }
            }
        }
    }
}

#[component]
fn View(filter: String) -> Element {
    let users = {
        let filter = filter.clone();
        use_resource(move || get_data(filter.clone()))
    };

    match &*users.read_unchecked() {
        Some(Ok(data)) => {
            // Render data
            let data = data.clone();
            rsx! {
                div {
                    class: "py-4",
                    Header {},
                    Table {
                        filter,
                        data
                    }
                }
            }
        }
        Some(Err(e)) => {
            // Render the error
            rsx! {
                { e.clone() }
            }
        }
        None => {
            // loading screen
            rsx! {
                Loading { }
            }
        }
    }
}

#[component]
fn Loading() -> Element {
    rsx! {
        div {
            class: "w-full h-full",
            svg {
                class: "animate-spin w-16 h-16 black"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Filter {
    All,
    Location,
}

#[component]
fn Table(filter: String, data: Users) -> Element {
    let dataview = use_signal(|| data.clone());
    let datafilter = use_signal(|| Filter::All);
    let offset = match datafilter.read().clone() {
        Filter::All => 0,
        Filter::Location => 5,
    };
    rsx! {
        div {
            class: "relative overflow-x-auto overflow-y-auto shadow-md px-2",
            table {
                class: "min-w-full text-sm text-left rtl:text-right text-gray-600 rounded-sm",
                caption {
                    class: "p-5 text-lg font-semibold text-left rtl:text-right text-gray-900 dark:text-white",
                    { format!("{filter} data:") }
                }
                thead {
                    class: "border border-black",
                    tr {
                        for column in data.columns()[offset..] {
                            th {
                                scope: "col", class: "px-6 py-3",
                                div {
                                    class: "flex items-center font-semibold text-black",
                                    { column }
                                }
                            }
                        }
                    }
                },
                tbody {
                    class: "rounded-md",
                    for user in dataview.read().iter() {
                        Row {
                            data: user.clone().to_row()[offset..].to_vec()
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Row(data: Vec<String>) -> Element {
    rsx! {
            tr {
                class: "border border-black hover:bg-gray-100",
                for (i, field) in data.iter().enumerate() {
                    if i == 0 {
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium whitespace-nowrap",
                            { field.clone() }
                        }
                    } else {
                        td {
                            class: "px-6 py-4",
                            { field.clone() }
                        }
                }
                }
            }

    }
}

#[component]
fn Editor() -> Element {
    let mut query = use_signal(|| String::new());
    let mut query_result: Signal<Option<Result<(), String>>> = use_signal(|| None);

    let handle_keydown = move |event: Event<KeyboardData>| async move {
        match event.data.key() {
            Key::Backspace => {
                query.with_mut(|s| {
                    let len = s.len();
                    if len > 0 {
                        s.remove(s.len() - 1);
                    }
                });
            }
            Key::Character(str) => {
                query.with_mut(|s| s.push_str(&str));
            }
            _ => {}
        }
    };

    let send_query = move |_| async move {
        let sent = send_query(query.take().as_str()).await;
        query_result.set(Some(sent))
    };
    rsx! {
        div {
            class: "w-screen",
            Header {},
            div {
                class: "w-full p-6",
                div {
                    class: "flex flex-col space-y-4 w-full max-w-4xl mx-auto",
                    div {
                        class: "flex flex-col justify-between items-center",
                        h1 {
                            class: "text-lg text-gray-800",
                            "Query Builder"
                        },
                        textarea {
                            class: "w-full h-96 p-4 border border-black text-black",
                            placeholder: "Pls don't break prod!",
                            onkeydown: handle_keydown
                        },
                        button {
                            class: "my-2 px-6 py-3 border border-black text-black bg-white hover:bg-gray-200",
                            onclick: send_query,
                        },
                        QueryResult {
                            res: query_result.read().clone()
                        }
                    }

                }
            }
        }

    }
}

#[component]
fn QueryResult(res: Option<Result<(), String>>) -> Element {
    if let Some(res) = res {
        match res {
            Ok(()) => rsx! {
                p {
                    class: "",
                    "Succesfully executed query!"
                }
            },
            Err(e) => rsx! {
                p {
                    class: "text-red font-semibold",
                    { e }
                }
            },
        }
    } else {
        rsx! {}
    }
}
