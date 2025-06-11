use crate::backend;
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src: Resource<String> = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            div { id: "image-card",
                img {
                    src: img_src.cloned().unwrap_or_default(),
                    alt: "dog image"
                }
            }
        }

        div { id: "buttons",
        button {
            id: "skip",
            onclick: move |_| img_src.restart(),
            svg {
                width: "75",
                height: "75",
                view_box: "0 0 506.4 506.4",
                xmlns: "http://www.w3.org/2000/svg",
                stroke_width: "0",
                fill: "#000000",

                g {
                    id: "SVGRepo_iconCarrier",

                    circle {
                        style: "fill:#FFC52F;",
                        cx: "253.2",
                        cy: "253.2",
                        r: "249.2",
                    }

                    g {
                        path {
                            style: "fill:#F4EFEF;",
                            d: "M187.2,340c-12.8,8.8-22,3.2-22-12.4V179.2c0-15.6,9.2-21.2,22-12.4
                                l101.2,70.8c12.8,8.8,12.8,23.2,0.4,32L187.2,340z",
                        }

                        path {
                            style: "fill:#F4EFEF;",
                            d: "M321.2,177.2v155.2c0,6.4,8.8,12.8,15.2,12.8h8.4c6.4,0,8.8-6.4,
                                8.8-12.8V174c0-6.4-2.4-12.8-8.8-12.8h-8.4c-6.4,0-14.4,9.6-14.4,16",
                        }
                    }

                    path {
                        d: "M253.2,506.4C113.6,506.4,0,392.8,0,253.2S113.6,0,253.2,0s253.2,
                            113.6,253.2,253.2S392.8,506.4,253.2,506.4z M253.2,8C118,8,8,118,8,
                            253.2s110,245.2,245.2,245.2s245.2-110,245.2-245.2S388.4,8,253.2,8z",
                    }

                    path {
                        d: "M173.6,348c-2.4,0-5.2-0.4-7.2-1.6c-6-3.2-9.2-10-9.2-18.8V179.2c0-8.8,
                            3.2-15.6,9.2-18.8c6-3.2,13.6-2,21.2,3.2L290,234.4c7.2,4.8,11.2,12,11.2,
                            19.2s-4,14.4-11.2,19.6L188,344c-5.2,2.4-10,4-14.4,4z M173.6,166.4c-1.2,
                            0-2.4,0.4-3.6,0.8c-3.2,1.6-5.2,6-5.2,11.6v148.4c0,6,2,10,5.2,11.6s8,0.8,
                            12.8-2.8l0,0l102-70.8c5.2-3.6,8-8,8-12.8s-2.8-9.2-7.6-12.4l-102.4-70.8
                            c-3.2-2.4-6.4-3.6-9.2-3.6z",
                    }

                    path {
                        d: "M341.2,350.4h-6.8c-7.2,0-13.2-5.6-13.2-12.8V169.2c0-6.8,6.4-12.8,
                            13.6-12.8h5.6c6.8,0,12.8,6,12.8,13.2V338c0,6.8-5.2,12.4-12,12.4z
                            M334.8,164c-2.8,0-5.6,2.4-5.6,4.8v168.4c0,2.4,2.4,4.8,5.2,4.8h6.8
                            c2.8,0,4-2.4,4-4.8v-168c0-2.8-2-5.2-4.8-5.2h-5.6z",
                    }
                }
            }
        }
            button {
                id: "save",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = backend::save_dog(current).await;
                },
                svg {
                    width: "75",
                    height: "75",
                    view_box: "0 0 60 60",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "#ffffff",
                    stroke: "#ffffff",
                    stroke_width: "0.0006",
                    defs {
                        style { r#"
                            .cls-1 {{ fill: #699f4c; }}
                            .cls-1, .cls-2 {{ fill-rule: evenodd; }}
                            .cls-2 {{ fill: #ff4013; }}
                        "# }
                    }
                    path {
                        class: "cls-1",
                        d: "M817.875,126a12,12,0,1,1-12,12A12,12,0,0,1,817.875,
                        126ZM812,140h4v4a2,2,0,0,0,4,0v-4h4a2,2,0,0,0,0-4h-4v-4a2,2,0,0,0-4,
                        0v4h-4A2,2,0,0,0,812,140Z",
                        transform: "translate(-770 -90)",
                    }
                    path {
                        class: "cls-2",
                        d: "M825.88,121.835A17.975,17.975,0,0,0,803.4,148.5l-0.216.177a4.509,
                        4.509,0,0,1-6.375,0S770,127.085,770,107a17,17,0,0,1,17-17c7.625,0,11.562,
                        6.057,13,6.057S805.375,90,813,90a17,17,0,0,1,17,17C830,111.97,828.351,117.031,
                        825.88,121.835Z",
                        transform: "translate(-770 -90)",
                    }
                }
            }

        }
    }
}
