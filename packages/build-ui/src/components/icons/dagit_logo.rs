use dioxus::prelude::*;

#[component]
pub fn ServiceLogo(
    #[props(default = "170".to_string())] width: String,
    #[props(default = "85".to_string())] height: String,
    #[props(default = "fill-[#555462]".to_string())] class: String,
) -> Element {
    rsx! {
        svg {
            width,
            height,
            fill: "none",
            view_box: "0 0 170 85",
            xmlns: "http://www.w3.org/2000/svg",
            g { filter: "url(#filter0_d_656_39073)",
                path {
                    d: "M69.7942 41.2628H74.8835L72.3549 32.3482L69.7942 41.2628ZM68.4976 26.9674H77.0878L84.446 50.1445H77.4122L76.1801 45.8332H68.4976L67.2659 50.1445H61.0742L68.4976 26.9674Z",
                    fill: "#040707",
                }
                path {
                    d: "M84.8087 39.9156L84.782 39.6577C84.0523 32.6289 88.7192 27.3538 95.6511 26.6345C101.1 26.0688 106.194 28.0492 107.268 33.9996L100.723 34.6787C100.176 32.5524 98.6728 31.5678 96.2222 31.8219C92.9657 32.1601 91.2598 34.8787 91.6915 39.0383L91.7182 39.2963C92.1066 43.0361 93.7943 45.7941 97.7921 45.3791C101.017 45.0443 102.071 43.2732 102.01 41.4218L97.689 41.8705L97.2102 37.2595L108.044 36.1352L108.338 38.9727C108.947 44.8408 105.569 49.6564 98.1207 50.4294C90.2861 51.2426 85.5249 46.8154 84.8087 39.9156Z",
                    fill: "#040707",
                }
                path {
                    d: "M111.602 26.9693H118.311V50.1465H111.602V26.9693Z",
                    fill: "#040707",
                }
                path {
                    d: "M127.325 32.0888H121.328V26.967H139.999V32.0888H134.003V50.1445H127.325V32.0888Z",
                    fill: "#040707",
                }
                path {
                    d: "M57.7411 50.5332H52.4688V45.2613H57.7411V50.5332Z",
                    fill: "#30D5A0",
                }
                path {
                    d: "M43.1572 44.6556V37.9122C42.429 37.5079 41.8598 37.2187 41.4498 37.0457C41.0393 36.8724 40.5571 36.7855 40.0022 36.7855C39.02 36.7855 38.2022 37.2013 37.5497 38.0338C36.8964 38.8659 36.5701 39.9983 36.5701 41.4312C36.5701 42.9911 36.9137 44.127 37.6014 44.8375C38.2891 45.5483 39.0891 45.9035 40.0022 45.9035C40.7532 45.9035 41.3602 45.7854 41.8224 45.5483C42.2845 45.3116 42.7297 45.0139 43.1572 44.6556ZM49.4153 50.272H43.1572V48.677C42.4985 49.2322 41.8023 49.6886 41.0683 50.0465C40.3343 50.4048 39.3204 50.584 38.0261 50.584C36.7435 50.584 35.4836 50.2662 34.2472 49.6307C33.0105 48.9951 31.9935 48.004 31.1962 46.658C30.3989 45.3116 30 43.6154 30 41.5702C30 39.964 30.3004 38.4581 30.9016 37.0542C31.5024 35.6503 32.4445 34.509 33.7271 33.6306C35.0098 32.7525 36.5813 32.3132 38.4423 32.3132C39.5976 32.3132 40.5339 32.4406 41.2505 32.6942C41.9668 32.9491 42.6023 33.2726 43.1572 33.6653V26.2458H49.4153V50.272Z",
                    fill: "#040707",
                }
            }
            defs {
                filter {
                    color_interpolation_filters: "sRGB",
                    filterUnits: "userSpaceOnUse",
                    height: "84.3379",
                    id: "filter0_d_656_39073",
                    width: "170",
                    x: "0",
                    y: "0.246094",
                    feFlood { flood_opacity: "0", result: "BackgroundImageFix" }
                    feColorMatrix {
                        "in": "SourceAlpha",
                        r#type: "matrix",
                        result: "hardAlpha",
                        values: "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0",
                    }
                    feOffset { dy: "4" }
                    feGaussianBlur { std_deviation: "15" }
                    feComposite { in2: "hardAlpha", operator: "out" }
                    feColorMatrix {
                        r#type: "matrix",
                        values: "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.1 0",
                    }
                    feBlend {
                        in2: "BackgroundImageFix",
                        mode: "normal",
                        result: "effect1_dropShadow_656_39073",
                    }
                    feBlend {
                        "in": "SourceGraphic",
                        in2: "effect1_dropShadow_656_39073",
                        mode: "normal",
                        result: "shape",
                    }
                }
            }
        }
    }
}
