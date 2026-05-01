// Copyright 2026 TreeCloud AI
// Licensed under the Apache License, Version 2.0. See LICENSE-SAURON.

//! `view-source:` protocol handler for Sauron.
//!
//! Visiting a URL of the form `view-source:https://example.com` fetches the
//! underlying resource as text and renders it inside a `<pre>` element so
//! the user can inspect the raw HTML/CSS/JS source. This is the standard
//! Firefox/Chromium developer affordance and is wired to `Ctrl+U` in
//! [`crate::desktop::headed_window`].
//!
//! Unlike the upstream Servo protocol handlers (`urlinfo`, `resource`,
//! `servo`), this one performs a blocking outbound HTTP request via `ureq`
//! on a dedicated thread (`tokio::task::spawn_blocking`) so it never blocks
//! the async runtime.

use std::future::Future;
use std::pin::Pin;

use headers::{ContentType, HeaderMapExt};
use servo::protocol_handler::{
    DoneChannel, FetchContext, HttpStatus, ProtocolHandler, Request, ResourceFetchTiming, Response,
    ResponseBody,
};

#[derive(Default)]
pub struct ViewSourceProtocolHandler {}

impl ProtocolHandler for ViewSourceProtocolHandler {
    fn load(
        &self,
        request: &mut Request,
        _done_chan: &mut DoneChannel,
        _context: &FetchContext,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let url = request.current_url();
        let timing_type = request.timing_type();

        // `view-source:https://example.com` parses with scheme = "view-source"
        // and the rest of the URL is opaque, so we have to reconstruct the
        // inner target by stripping the "view-source:" prefix from the
        // serialized form.
        let serialized = url.as_str().to_owned();
        let inner_target = serialized
            .strip_prefix("view-source:")
            .unwrap_or(&serialized)
            .to_owned();
        let inner_for_render = inner_target.clone();

        Box::pin(async move {
            let mut response = Response::new(url, ResourceFetchTiming::new(timing_type));

            let fetch_target = inner_target.clone();
            let fetch_result: std::result::Result<std::result::Result<String, String>, _> =
                tokio::task::spawn_blocking(move || {
                    match ureq::get(&fetch_target)
                        .timeout(std::time::Duration::from_secs(15))
                        .call()
                    {
                        Ok(response) => response.into_string().map_err(|e| e.to_string()),
                        Err(e) => Err(e.to_string()),
                    }
                })
                .await;

            let body = match fetch_result {
                Ok(Ok(text)) => render_source_page(&inner_for_render, &text),
                Ok(Err(e)) => render_error_page(&inner_for_render, &e),
                Err(join_err) => {
                    render_error_page(&inner_for_render, &format!("internal: {join_err}"))
                },
            };

            *response.body.lock() = ResponseBody::Done(body.into_bytes());
            response.headers.typed_insert(ContentType::html());
            response.status = HttpStatus::default();
            response
        })
    }

    fn is_fetchable(&self) -> bool {
        true
    }

    fn is_secure(&self) -> bool {
        // The handler only renders fetched bytes inside a <pre>, so the
        // content is never executed; treating the response as secure keeps
        // the browser chrome from flagging it.
        true
    }
}

fn html_escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

fn render_source_page(target: &str, body: &str) -> String {
    let target_escaped = html_escape(target);
    let body_escaped = html_escape(body);
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Source: {target_escaped}</title>
<style>
  body {{ margin: 0; font-family: ui-monospace, Menlo, Consolas, monospace;
          background: #1e1e1e; color: #d4d4d4; }}
  header {{ padding: 0.6em 1em; background: #252526; border-bottom: 1px solid #333;
            font-size: 0.9em; word-break: break-all; }}
  header a {{ color: #4ec9b0; text-decoration: none; }}
  header a:hover {{ text-decoration: underline; }}
  pre {{ margin: 0; padding: 1em; white-space: pre-wrap; word-break: break-word;
         font-size: 0.9em; line-height: 1.4; }}
  @media (prefers-color-scheme: light) {{
    body {{ background: #fafafa; color: #222; }}
    header {{ background: #f0f0f0; border-bottom: 1px solid #ddd; }}
    header a {{ color: #0a7; }}
  }}
</style>
</head>
<body>
<header>view-source: <a href="{target_escaped}">{target_escaped}</a></header>
<pre>{body_escaped}</pre>
</body>
</html>"#,
    )
}

fn render_error_page(target: &str, error: &str) -> String {
    let target_escaped = html_escape(target);
    let error_escaped = html_escape(error);
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Source: {target_escaped} (failed)</title>
<style>
  body {{ font-family: system-ui, sans-serif; padding: 2em; max-width: 720px;
          margin: 0 auto; }}
  h1 {{ color: #c33; }}
  code {{ background: #eee; padding: 2px 4px; border-radius: 3px; }}
</style>
</head>
<body>
<h1>Could not fetch source</h1>
<p><strong>Target:</strong> <code>{target_escaped}</code></p>
<p><strong>Error:</strong> {error_escaped}</p>
</body>
</html>"#,
    )
}
