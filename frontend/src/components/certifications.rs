use crate::traits::MarkdownRenderable;
use std::collections::HashMap;
use yew::prelude::*;

#[allow(dead_code)]
struct CertificationBadge {
    name: &'static str,  // Badge name for alt text and title
    url: &'static str,   // Link to certificate verification
    image: &'static str, // Path to badge image file
    width: &'static str,
    height: &'static str,
}

#[function_component(Certifications)]
pub fn certifications() -> Html {
    let certifications: Vec<CertificationBadge> = vec![
        CertificationBadge {
            name: "AWS Certified Solutions Architect Associate",
            url: "https://www.credly.com/badges/e429c916-eca4-4e1a-99f0-7b0035d0984e",
            image: "/static/images/badges/aws-certified-solutions-architect-associate.png",
            width: "150",
            height: "150",
        },
        CertificationBadge {
            name: "HashiCorp Certified: Terraform Associate",
            url: "https://www.credly.com/badges/a7e6f1ec-d156-43a3-a711-1e782cf17c41",
            image: "/static/images/badges/hashicorp-certified-terraform-associate-003.png",
            width: "150",
            height: "150",
        },
        CertificationBadge {
            name: "Kanban Management Professional",
            url: "https://edu.kanban.university/user/83575/8/qualification-certificate/R2VydGphbiBBc3NpZXM6ZzlUaTJEQ2pUUWIqM1llNjoxNzYwMzYwMTM2",
            image: "/static/images/badges/KMP_Badge.svg",
            width: "150",
            height: "150",
        },
        // Add more badges here - supports any certification provider!
        //
        // Just add the badge image to /static/images/badges/ and create an entry:
        //
        // CertificationBadge {
        //     name: "Your Certification Name",
        //     url: "https://verification-url.com/your-cert",
        //     image: "/static/images/badges/your-badge.png",
        //     width: "150",
        //     height: "150",
        // },
    ];

    html! {
        <div class="certifications">
            {
                for certifications.iter().map(|cert| {
                    html! {
                        <div class="certification-badge">
                            <a href={cert.url} target="_blank" rel="noopener noreferrer" class="certification-link" title={cert.name}>
                                <img
                                    src={cert.image}
                                    alt={cert.name}
                                    width={cert.width}
                                    height={cert.height}
                                    loading="lazy"
                                />
                            </a>
                        </div>
                    }
                })
            }
        </div>
    }
}

// Implement the trait for Certifications component
impl MarkdownRenderable for Certifications {
    fn render(_attributes: &HashMap<String, String>) -> Html {
        html! { <Certifications /> }
    }
}
