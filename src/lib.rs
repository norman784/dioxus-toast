#![allow(non_snake_case)]

// mod style;

use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus::fermi::UseAtomRef;

#[derive(Debug, Clone)]
struct ToastManagerItem {
    info: ToastInfo,
    timestamp: i64,
    hide_after: usize,
}

#[derive(Default, Debug)]
pub struct ToastManager {
    list: BTreeMap<u8, ToastManagerItem>,
    id_index: u8,
}

impl ToastManager {
    pub fn popup(&mut self, option: ToastInfo) -> u8 {
        self.id_index += 1;
        let toast_id = self.id_index;

        let hide_after = option.hide_after.unwrap_or(0);
        let timestamp = chrono::Local::now().timestamp();

        self.list.insert(
            toast_id,
            ToastManagerItem {
                info: option,
                timestamp,
                hide_after,
            },
        );

        toast_id
    }

    pub fn remove(&mut self, id: u8) {
        self.list.remove(&id);
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.id_index = 0;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Position {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Icon {
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Debug, Clone)]
pub struct ToastInfo {
    pub heading: Option<String>,
    pub context: String,
    pub allow_toast_close: bool,
    pub position: Position,
    pub icon: Option<Icon>,
    pub hide_after: Option<usize>,
}

impl ToastInfo {
    pub fn simple(text: &str) -> Self {
        Self {
            heading: None,
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: None,
            hide_after: Some(6),
        }
    }

    pub fn success(text: &str, heading: &str) -> Self {
        Self {
            heading: Some(heading.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: Some(Icon::Success),
            hide_after: Some(6),
        }
    }

    pub fn warning(text: &str, heading: &str) -> Self {
        Self {
            heading: Some(heading.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: Some(Icon::Warning),
            hide_after: Some(6),
        }
    }

    pub fn info(text: &str, heading: &str) -> Self {
        Self {
            heading: Some(heading.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: Some(Icon::Info),
            hide_after: Some(6),
        }
    }

    pub fn error(text: &str, heading: &str) -> Self {
        Self {
            heading: Some(heading.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: Some(Icon::Error),
            hide_after: Some(6),
        }
    }
}

#[derive(Props)]
pub struct ToastFrameProps<'a> {
    manager: &'a UseAtomRef<ToastManager>,

    #[props(default = 5)]
    maximum: u8,
}

pub fn ToastFrame<'a>(cx: Scope<'a, ToastFrameProps<'a>>) -> Element {
    // println!("{:?}", manager.read());

    let manager = cx.props.manager;

    let toast_list = &manager.read().list;

    let mut bottom_left_ele: Vec<LazyNodes> = vec![];
    let mut bottom_right_ele: Vec<LazyNodes> = vec![];
    let mut top_left_ele: Vec<LazyNodes> = vec![];
    let mut top_right_ele: Vec<LazyNodes> = vec![];

    for (current_num, (id, item)) in toast_list.iter().enumerate() {
        let current_id = *id;

        let icon_class = if let Some(icon) = &item.info.icon {
            let mut class = String::from("has-icon ");

            match icon {
                Icon::Success => class.push_str("icon-success"),
                Icon::Warning => class.push_str("icon-warning"),
                Icon::Error => class.push_str("icon-error"),
                Icon::Info => class.push_str("icon-info"),
            }

            class
        } else {
            String::new()
        };

        let element = rsx! {
            div {
                class: "toast-single {icon_class}",
                id: "{id}",
                if item.info.allow_toast_close {
                    cx.render(rsx! {
                        div {
                            class: "close-toast-single",
                            onclick: move |_| {
                                manager.write().list.remove(&current_id);
                            },
                            "×",
                        }
                    })
                } else {
                    None
                }
                if let Some(v) = &item.info.heading {
                    cx.render(rsx! {
                        h2 {
                            class: "toast-heading",
                            "{v}"
                        }
                    })
                } else {
                    None
                }

                span {
                    dangerous_inner_html: "{item.info.context}",
                }
            }
        };

        if current_num >= cx.props.maximum.into() {
            break;
        }

        if item.info.position == Position::BottomLeft {
            bottom_left_ele.push(element);
        } else if item.info.position == Position::BottomRight {
            bottom_right_ele.push(element);
        } else if item.info.position == Position::TopLeft {
            top_left_ele.push(element);
        } else if item.info.position == Position::TopRight {
            top_right_ele.push(element);
        }
    }

    use_future(&cx, (), |_| {
        let toast_manager = manager.clone();
        async move {
            loop {
                let timer_list = toast_manager.read().list.clone();
                for (id, time) in &timer_list {
                    let time = (time.timestamp, time.hide_after);
                    let current_time = chrono::Local::now().timestamp();
                    let expire_time = time.0 + time.1 as i64;
                    // println!("{:?} -> {:?}", current_time, expire_time);
                    if current_time >= expire_time && time.1 != 0_usize {
                        toast_manager.write().list.remove(id);
                    }
                }
                if toast_manager.read().list.is_empty() {
                    toast_manager.write().id_index = 0;
                }
                time_sleep(100).await;
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "toast-scope",
            style { [ include_str!("./assets/toast.css") ] },
            div {
                class: "toast-wrap bottom-left",
                id: "wrap-bottom-left",
                bottom_left_ele
            }
            div {
                class: "toast-wrap bottom-right",
                id: "wrap-bottom-right",
                bottom_right_ele
            }
            div {
                class: "toast-wrap top-left",
                id: "wrap-top-left",
                top_left_ele
            }
            div {
                class: "toast-wrap top-right",
                id: "wrap-top-right",
                top_right_ele
            }
        }
    })
}

#[cfg(feature = "web")]
async fn time_sleep(interval: usize) {
    gloo_timers::future::TimeoutFuture::new(interval as u32).await;
}

#[cfg(feature = "desktop")]
async fn time_sleep(interval: usize) {
    tokio::time::sleep(tokio::time::Duration::from_millis(interval as u64)).await;
}