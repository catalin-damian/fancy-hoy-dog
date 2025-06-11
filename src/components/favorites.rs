use crate::backend;
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    let favorites: Resource<Result<Vec<(usize, String)>, ServerFnError>> =
        use_resource(backend::list_dogs);

    // Suspend the component until the resource is resolved
    let result: MappedSignal<Result<Vec<(usize, String)>, ServerFnError>> = favorites.suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                match result() {
                    Ok(dogs) => rsx! {
                        for (id, url) in dogs {
                            div {
                                key: "{id}",
                                class: "favorite-dog",
                                img { src: "{url}" }

                                button {
                                    class: "delete-button",
                                    onclick: move |_| {
                                        let mut favorites = favorites.clone();
                                        spawn(async move {
                                            if backend::delete_dog(id).await.is_ok() {
                                                favorites.restart();
                                            }
                                        });
                                    },
                                    "Delete"
                                }
                            }
                        }
                    },
                    Err(e) => rsx! {
                        div { class: "error-message",
                            h3 { "Failed to load favorites 😞" }
                            p { "Error: {e}" }
                        }
                    }
                }
            }
        }
    }
}
