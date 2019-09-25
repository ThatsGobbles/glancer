markup::define! {
    Dropbox<'a>(img_url: &'a str) {
        {markup::doctype()}
        html {
            head {
                title { "Dropbox Images" }
            }
            body {
                #main.container {
                    img[src = img_url, alt = ""] {}
                    // br;
                    // p.url_info {
                    //     "Link: " {img_url}
                    // }
                }
            }
        }
    }
}
