use web_sys::window;
use yew::prelude::*;

// This is a test comment to trigger rust-analyzer refresh

#[allow(dead_code)]
struct CertificationBadge {
    id: &'static str,
    width: &'static str,
    height: &'static str,
}

#[function_component(Certifications)]
pub fn certifications() -> Html {
    let certifications: Vec<CertificationBadge> = vec![
        CertificationBadge {
            id: "a7e6f1ec-d156-43a3-a711-1e782cf17c41",
            width: "150",
            height: "270",
        },
        CertificationBadge {
            id: "e429c916-eca4-4e1a-99f0-7b0035d0984e",
            width: "150",
            height: "270",
        },
        // Add more badges here as needed
    ];

    // Load the Credly embed script when component mounts
    use_effect_with((), |_| {
        // Check if script is already loaded
        if let Some(document) = window().and_then(|w| w.document()) {
            let scripts = document.get_elements_by_tag_name("script");
            let mut script_exists = false;

            for i in 0..scripts.length() {
                if let Some(script) = scripts.item(i) {
                    if let Some(src) = script.get_attribute("src") {
                        if src.contains("cdn.credly.com/assets/utilities/embed.js") {
                            script_exists = true;
                            break;
                        }
                    }
                }
            }

            if !script_exists {
                if let Ok(script) = document.create_element("script") {
                    let _ = script.set_attribute("type", "text/javascript");
                    let _ = script.set_attribute("async", "");
                    let _ =
                        script.set_attribute("src", "//cdn.credly.com/assets/utilities/embed.js");

                    if let Some(head) = document.head() {
                        let _ = head.append_child(&script);
                    }
                }
            }
        }

        || {}
    });

    html! {
        <div>
            <h3>{"Certifications"}</h3>
            <div class="certifications">
                {
                    for certifications.iter().map(|cert| {
                        html! {
                            <div class="certification-badge">
                                <div
                                    data-iframe-width={cert.width}
                                    data-iframe-height={cert.height}
                                    data-share-badge-id={cert.id}
                                    data-share-badge-host="https://www.credly.com"
                                >
                                </div>
                            </div>
                        }
                    })
                }
            </div>
        </div>
    }
}
