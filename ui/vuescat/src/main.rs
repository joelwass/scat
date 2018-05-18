extern crate web_view;

use web_view::*;

fn main() {
    let size = (700, 700);
    let resizable = true;
    let debug = true;
    let titlebar_transparent = true;
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();

    let html = format!(r#"
      <!DOCTYPE html><html><head><meta charset=utf-8><meta http-equiv=X-UA-Compatible content="IE=edge"><meta name=viewport content="width=device-width,initial-scale=1"><link rel=icon href=/dist/favicon.ico><title>vuescat</title><link as=style href=/dist/css/app.ba3d1dc6.css rel=preload><link as=script href=/dist/js/app.49004367.js rel=preload><link as=script href=/dist/js/vendors~app.8ed9a393.js rel=preload><link href=/dist/css/app.ba3d1dc6.css rel=stylesheet>
      <style>{css}</style>
      <style>{css2}</style>
      </head><body>hi<noscript><strong>We're sorry but vuescat doesn't work properly without JavaScript enabled. Please enable it to continue.</strong></noscript><div id=app></div><script type=text/javascript>{js}</script><script type=text/javascript>{js2}</script></body></html>
    "#,
    js = include_str!("../dist/js/vendors~app.8ed9a393.js"),
    js2 = include_str!("../dist/js/app.49004367.js"),
    css = include_str!("../dist/css/app.ba3d1dc6.css"),
    css2 = include_str!("../dist/css/app.ba3d1dc6.css"));

    run(
        "",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        titlebar_transparent,
        move |mut webview| {
            webview.set_background_color(0.11, 0.12, 0.13, 1.0);
        },
        frontend_cb,
        userdata
    );
}
