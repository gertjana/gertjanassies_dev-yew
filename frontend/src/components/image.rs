use crate::traits::MarkdownRenderable;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ImageProps {
    pub path: String,
    #[prop_or_default]
    pub alt: String,
    #[prop_or(300)]
    pub thumbnail_width: u32,
    #[prop_or_default]
    pub class: String,
}

pub struct Image {
    modal_open: bool,
}

pub enum ImageMsg {
    OpenModal,
    CloseModal,
}

impl Component for Image {
    type Message = ImageMsg;
    type Properties = ImageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { modal_open: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ImageMsg::OpenModal => {
                self.modal_open = true;
                true
            }
            ImageMsg::CloseModal => {
                self.modal_open = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

        let thumbnail_style = format!("width: {}px; cursor: pointer;", props.thumbnail_width);
        let modal_class = if self.modal_open {
            "image-modal open"
        } else {
            "image-modal"
        };

        html! {
            <>
                // Thumbnail image
                <img
                    src={props.path.clone()}
                    alt={props.alt.clone()}
                    style={thumbnail_style}
                    class={format!("image-thumbnail {}", props.class)}
                    onclick={link.callback(|_| ImageMsg::OpenModal)}
                />

                // Modal overlay
                <div class={modal_class} onclick={link.callback(|_| ImageMsg::CloseModal)}>
                    <div class="image-modal-content" onclick={link.callback(|e: MouseEvent| {
                        e.stop_propagation();
                        ImageMsg::CloseModal
                    })}>
                        <img
                            src={props.path.clone()}
                            alt={props.alt.clone()}
                            class="image-modal-image"
                        />
                        <button
                            class="image-modal-close"
                            onclick={link.callback(|_| ImageMsg::CloseModal)}
                        >
                            {"×"}
                        </button>
                    </div>
                </div>
            </>
        }
    }
}

impl MarkdownRenderable for Image {
    fn render(attributes: &HashMap<String, String>) -> Html {
        let path = attributes.get("path").map_or("", |v| v).to_string();
        let alt = attributes.get("alt").map_or("", |v| v).to_string();
        let thumbnail_width = attributes
            .get("thumbnail_width")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(300);
        let class = attributes.get("class").map_or("", |v| v).to_string();

        html! {
            <Image {path} {alt} {thumbnail_width} {class} />
        }
    }
}
