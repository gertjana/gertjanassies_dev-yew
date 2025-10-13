use yew::prelude::*;

#[allow(dead_code)]
struct CertificationBadge {
    id: &'static str,
    title: &'static str,
    issuer: &'static str,
    image_path: &'static str,
}

#[function_component(Certifications)]
pub fn certifications() -> Html {
    let certifications: Vec<CertificationBadge> = vec![
        CertificationBadge {
            id: "e429c916-eca4-4e1a-99f0-7b0035d0984e",
            title: "AWS Solutions Architect - Associate",
            issuer: "Amazon Web Services (AWS)",
            image_path: "/static/images/aws-solutions-architect-associate.png",
        },
        CertificationBadge {
            id: "a7e6f1ec-d156-43a3-a711-1e782cf17c41",
            title: "HashiCorp Certified: Terraform Associate",
            issuer: "HashiCorp",
            image_path: "/static/images/terraform-associate.png",
        },
        // Add more badges here as needed
    ];

    html! {
        <div class="certifications">
            {
                for certifications.iter().map(|cert| {
                    let badge_url = format!("https://www.credly.com/badges/{}", cert.id);
                    html! {
                        <div class="certification-badge">
                            <a href={badge_url} target="_blank" rel="noopener noreferrer" title={format!("{} - {}", cert.title, cert.issuer)}>
                                <img
                                    src={cert.image_path}
                                    alt={cert.title}
                                    width="150"
                                    height="150"
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
