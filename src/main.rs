#![allow(non_snake_case)]
mod client;
mod evil_env;
mod model;

use std::ops::Deref;

use client::{get_data, send_query};
use dioxus::prelude::*;
use model::{Column, TableView, User, Users, NUM_COLUMNS};
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/", Menu)]
    Menu {},
    #[route("/edit", Editor)]
    Editor {},
    #[route("/view/:filter", View)]
    View { filter: String },
    #[route("/queryview")]
    QueryView { data: Users },
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
                class: "m-8 text-8xl font-square tracking-wider",
                "KiggyMetric"
            }
            div {
                class: "flex flex-col space-y-4",
                button {
                    class: "px-6 py-3 border border-black text-black transition-all transition-500 ease-in-out hover:rounded-lg hover:border-double hover:border-4 hover:py-4",
                    Link {
                        class: "font-square text-xl",
                        to: Route::View { filter: "kiggysite".to_owned() },
                        "View Kiggy's site metrics" } },
                button {
                    class: "px-6 py-3 border border-black text-black transition-all transition-500 ease-in-out hover:rounded-lg hover:border-double hover:border-4 hover:py-4",
                    Link {
                        class: "font-square text-xl",
                        to: Route::View { filter: "faycarsons".to_owned() },
                        "View Fay's site metrics" } },
                button {
                    class: "px-6 py-3 border border-black text-black transition-all transition-500 ease-in-out hover:rounded-lg hover:border-double hover:border-4 hover:py-4",
                    Link {
                        class: "font-square text-xl",
                        to:  Route::View { filter: "all".to_owned() },
                        "View all"} },
                button {
                    class: "px-6 py-3 border border-black text-black transition-all transition-500 ease-in-out hover:rounded-lg hover:border-double hover:border-4 hover:py-4",
                    Link {
                        class: "font-square text-xl",
                        to: Route::Editor {},
                        "Query database" } }
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        div {
            class: "w-full h-8",
            button {
                class: "m-2 px-2 border border-black hover:bg-gray-200
                        transition-all transition-300 ease-in-out",
                Link {
                    class: "font-square",
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

    let mut datafilter = use_signal(|| Filter::All);

    let handle_filter = move |event: Event<FormData>| {
        let val = match event.value().as_str() {
            "All" => Filter::All,

            "Location" => Filter::Location,
            _ => unreachable!(),
        };
        datafilter.set(val);
    };

    match &*users.read_unchecked() {
        Some(Ok(data)) => {
            // Render data
            let data = data.clone();
            rsx! {
                div {
                    class: "",
                    Header {},
                    div {
                        class: "flex flex-row space-x-2 ml-2 mt-8 rounded-none",
                        label {
                            class: "text-md block text-md font-square",
                            "Filter: ",
                        },
                        select {
                            class: "arrow block rounded-none bg-gray-50
                                    font-square border border-black px-8",
                            onchange: handle_filter,
                            option { value: "All", "All" },
                            option { value: "Location", "Location" }

                        }
                    }
                    Table {
                        filter,
                        data,
                        datafilter: *datafilter.read()
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
            class: "grid place-items-center w-screen h-screen ",
            svg {
                class: "w-32 h-32 text-gray-300 animate-spin",
                view_box: "0 0 64 64",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                path {
                    d: "M32 3C35.8083 3 39.5794
                        3.75011 43.0978 5.20749C46.6163 
                        6.66488 49.8132 8.80101 52.5061 
                        11.4939C55.199 14.1868 57.3351 
                        17.3837 58.7925 20.9022C60.2499 
                        24.4206 61 28.1917 61 32C61 
                        35.8083 60.2499 39.5794 58.7925 
                        43.0978C57.3351 46.6163 55.199 
                        49.8132 52.5061 52.5061C49.8132 
                        55.199 46.6163 57.3351 43.0978 
                        58.7925C39.5794 60.2499 35.8083 
                        61 32 61C28.1917 61 24.4206 
                        60.2499 20.9022 58.7925C17.3837 
                        57.3351 14.1868 55.199 11.4939 
                        52.5061C8.801 49.8132 6.66487 
                        46.6163 5.20749 43.0978C3.7501 
                        39.5794 3 35.8083 3 32C3 28.1917 
                        3.75011 24.4206 5.2075 20.9022C6.66489 
                        17.3837 8.80101 14.1868 11.4939 
                        11.4939C14.1868 8.80099 17.3838 
                        6.66487 20.9022 5.20749C24.4206 
                        3.7501 28.1917 3 32 3L32 3Z",
                    stroke: "currentColor",
                    stroke_width: "5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round"
                },
                path {
                    d: "M32 3C36.5778 3 41.0906
                        4.08374 45.1692 6.16256C49.2477 
                        8.24138 52.7762 11.2562 55.466 
                        14.9605C58.1558 18.6647 59.9304 
                        22.9531 60.6448 27.4748C61.3591 
                        31.9965 60.9928 36.6232 59.5759 
                        40.9762",
                    stroke: "currentColor",
                    stroke_width: "5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    class: "text-gray-900"
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Filter {
    All = 0,
    Location = 5,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TableState {
    table: Vec<[String; NUM_COLUMNS]>,
    focus: Option<usize>,
}

const DATEFMT: &str = "%B %e, %Y at %H:%M:%S %p";

impl TableState {
    fn from_data(data: Users) -> Self {
        Self {
            table: data
                .into_iter()
                .map(User::to_row)
                .collect::<Vec<[String; NUM_COLUMNS]>>(),
            focus: None,
        }
    }

    fn sort_table(&mut self, column: Column) {
        let field = column as usize;
        let reverse = self.focus.is_some_and(|focus| focus == field);
        match column {
            Column::Resolution => {
                if reverse {
                    self.table.reverse()
                } else {
                    let parse_size = |size: &str| {
                        size.split_once('x').ok_or("").and_then(|(x, y)| {
                            match (str::parse::<usize>(x),str::parse::<usize>(y)) {
                                (Ok(x), Ok(y)) => Ok((x, y)),
                                _ => Err("")
                            }
                        }).unwrap_or((0, 0))
                    };
                    self.table.sort_by(|rowa, rowb| {
                        let (device_a, res_a) = rowa[field].split_once(':').unwrap_or(("",""));
                        let (device_b, res_b) = rowb[field].split_once(':').unwrap_or(("",""));

                        let device_ord = device_a.cmp(&device_b);
                        if let std::cmp::Ordering::Equal = device_ord {
                            let fst = parse_size(res_a);
                            let snd = parse_size(res_b);
                            fst.cmp(&snd)
                        } else {
                            device_ord
                        }
                    });
                    self.focus = Some(field);
                }
            }
            Column::Time => {
                use chrono::NaiveDateTime;
                if reverse {
                    self.table.reverse()
                } else {
                    self.table.sort_by(|rowa, rowb| {
                        let (a, b) = (
                            NaiveDateTime::parse_from_str(rowa[field].as_str(), DATEFMT)
                                .unwrap_or_default(),
                            NaiveDateTime::parse_from_str(rowb[field].as_str(), DATEFMT)
                                .unwrap_or_default(),
                        );
                        a.cmp(&b)
                    });
                    self.focus = Some(field);
                }
            }
            _ => {
                if reverse {
                    self.table.reverse();
                } else {
                    self.table
                        .sort_by(|rowa, rowb| rowa[field].cmp(&rowb[field]));
                    self.focus = Some(field);
                }
            }
        }
    }
}

#[component]
fn Table(filter: String, data: Users, datafilter: Filter) -> Element {
    let mut dataview = use_signal(|| TableState::from_data(data));

    let columns = match datafilter {
        Filter::All => model::COLUMNS.as_slice(),
        Filter::Location => {
            let columns = model::COLUMNS;
            &[[columns[0]].as_slice(), &columns[5..]].concat()
        }
    };

    let render_row = |row: &[String; NUM_COLUMNS]| match datafilter {
        Filter::All => rsx! {
            for (i, field) in row.iter().enumerate() {
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
        },
        Filter::Location => {
            rsx! {
                th {
                    scope: "row",
                    class: "px-6 py-4 font-medium whitespace-nowrap",
                    { row[0].clone() }
                }
                for field in &row[(datafilter as usize)..] {
                    td {
                        class: "px-6 py-4",
                        { field.clone() }
                    }
                }
            }
        }
    };

    let sort_icon = |field: Column| {
        rsx! {
            button {
                onclick: move |_| dataview.with_mut(
                    |table_state| table_state.sort_table(field)
                ),
                svg {
                    class: "w-3 h-3 ms-1.5",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        d: "M8.574 11.024h6.852a2.075 2.075 0 0 0 1.847-1.086
                            1.9 1.9 0 0 0-.11-1.986L13.736 2.9a2.122 2.122 0 0 
                            0-3.472 0L6.837 7.952a1.9 1.9 0 0 0-.11 1.986 2.074 
                            2.074 0 0 0 1.847 1.086Zm6.852 1.952H8.574a2.072 
                            2.072 0 0 0-1.847 1.087 1.9 1.9 0 0 0 .11 1.985l3.426 
                            5.05a2.123 2.123 0 0 0 3.472 0l3.427-5.05a1.9 1.9 0 
                            0 0 .11-1.985 2.074 2.074 0 0 0-1.846-1.087Z",
                    }
                }
            }
        }
    };

    rsx! {
        div {
            class: "my-2 relative overflow-x-auto overflow-y-auto p-2",
            table {
                class: "min-w-full text-sm text-left rtl:text-right
                        shadow-md",
                caption {
                    class: "border border-black border-b-0 p-5 text-lg
                            font-square text-left rtl:text-right",
                    { format!("{filter} data:") }
                }
                thead {
                    class: "border border-black",
                    tr {
                        for (idx, column) in columns.iter().enumerate() {
                            th {
                                scope: "col", class: "px-6 py-3",
                                div {
                                    class: "flex items-center font-square text-lg",
                                    { column }
                                    { sort_icon(Column::from(idx)) }
                                }
                            }
                        }
                    }
                },
                tbody {
                    for user in dataview.read().table.iter() {
                        tr {
                            class: "border border-black font-mono
                                    hover:bg-gray-200 hover:my-4",
                            { render_row(user) }
                        }
                    }
                }
            }
        }
    }
}

/* BASIC SQL SYNTAX HIGHLIGHTING - NOT WORKING W TEXTAREA
fn sql_syntax(statement: Signal<String>) -> Element {
    let statement = statement.read();
    let iter = statement.split(" ").map(|token| {
        match token {
            "SELECT" | "FROM" | "WHERE" | "AND" | "OR" | "INSERT" | "INTO" | "VALUES" | "UPDATE"
            | "SET" | "DELETE" | "JOIN" | "ON" | "AS" => {
                rsx! {
                    span {
                        class: "text-red-600",
                        { token }
                    }
                }
            }
            s => rsx! { { s } },
        }
    }).intersperse(rsx! { span { " " } });
    rsx! {
       { iter }
    }
}
*/

#[component]
fn Editor() -> Element {
    let mut query = use_signal(|| String::new());
    let mut query_result = use_signal(|| None);
    let navigator = use_navigator();

    // let div_inner = use_memo(use_reactive(&query, sql_syntax));

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
        let raw_query = query.take();
        let sent = send_query(raw_query.as_str()).await;
        query_result.set(Some(sent))
    };

    let handled_query = if let Some(res) = query_result.deref()() {
        match res {
            Ok(None) => rsx! {
                p {
                    class: "",
                    "OK 👍🏻"
                }
            },
            Ok(Some(data)) if data.is_empty() => {
                rsx! {}
            }
            Ok(Some(data)) => {
                // Query yielded data so we render it in QueryView
                println!("RECEIVED DATA!!!");
                navigator.push(Route::QueryView { data });
                rsx! {}
            }
            Err(e) => rsx! {
                p {
                    class: "text-red font-semibold",
                    { e }
                }
            },
        }
    } else {
        rsx! {}
    };

    rsx! {
        Header {},
        div {
            class: "w-full h-full flex flex-col justify-center items-center",
            div {
                class: "h-full w-3/4 flex flex-col space-y-4 justify-center items-center mx-auto",
                h1 {
                    class: "font-square text-6xl",
                    "Query Builder"
                },
                textarea {
                    class: "p-2 w-3/4 h-1/2 border border-black font-mono text-black",
                    placeholder: "Pls don't break prod!",
                    onkeydown: handle_keydown
                }
                button {
                    class: "px-3 py-2 border border-black font-square hover:bg-gray-200",
                    onclick: send_query,
                    "Send Query"
                },
                {
                   handled_query
                }

            }
        }
    }
}

#[component]
fn QueryView(data: Users) -> Element {
    let mut datafilter = use_signal(|| Filter::All);

    let handle_filter = move |event: Event<FormData>| {
        let val = match event.value().as_str() {
            "All" => Filter::All,

            "Location" => Filter::Location,
            _ => unreachable!(),
        };
        datafilter.set(val);
    };

    rsx! {
        div {
            class: "",
            Header {},
            div {
                class: "flex flex-row space-x-2 ml-2 mt-8 rounded-none",
                label {
                    class: "text-md block text-md font-square",
                    "Filter: ",
                },
                select {
                    class: "arrow block rounded-none bg-gray-50 font-square border border-black px-8",
                    onchange: handle_filter,
                    option { value: "All", "All" },
                    option { value: "Location", "Location" }

                }
            }
            Table {
                filter: "Query Result",
                data,
                datafilter: *datafilter.read()
            }
        }
    }
}
