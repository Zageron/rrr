#![allow(dead_code)]

use futures::StreamExt;
use js_sys::{Array, Uint8Array};
use std::fmt::Debug;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use wasm_streams::{readable::IntoStream, ReadableStream};
use web_sys::{window, Response};

#[wasm_bindgen]
pub struct Fetcher {
    url: String,
    stream: IntoStream<'static>,
    len: usize,
}

impl Debug for Fetcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fetcher")
            .field("url", &self.url)
            .field("stream", &self.stream)
            .field("len", &self.len)
            .finish()
    }
}

#[wasm_bindgen]
impl Fetcher {
    #[wasm_bindgen]
    pub async fn new(url: String) -> Result<Fetcher, JsValue> {
        let window = window().unwrap_throw();
        let resp_value = JsFuture::from(window.fetch_with_str(url.as_str()))
            .await
            .map_err(|_| "fetch failed")?;
        let resp: Response = resp_value.dyn_into().unwrap_throw();
        let raw_body = resp.body().unwrap_throw();
        let body = ReadableStream::from_raw(raw_body.dyn_into().unwrap_throw());
        let stream = body.into_stream();
        let len = resp
            .headers()
            .get("Content-Length")
            .unwrap()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(Self { url, stream, len })
    }

    #[wasm_bindgen]
    pub async fn fetch(self) -> Array {
        let mut stream = self.stream;
        let mut bytes: Vec<u8> = Vec::with_capacity(self.len);
        while let Some(Ok(chunk)) = stream.next().await {
            let buffer = Uint8Array::new(&chunk);
            bytes.extend(buffer.to_vec());
        }

        bytes.into_iter().map(JsValue::from).collect()
    }
}

// // rust tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::fetch::PlaylistPayload;
//     use wasm_bindgen_test::wasm_bindgen_test;
//     wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

//     #[wasm_bindgen_test(async)]
//     async fn test_fetch() {
//         let test_result = fetch::<PlaylistPayload>(
//             "https://www.flashflashrevolution.com/game/r3/r3-playlist.v2.php".to_string(),
//         )
//         .await;
//         assert!(test_result.is_ok());

//         if let Ok(result) = test_result {
//             if let Some(payload) = result {
//                 assert!(!payload.songs.is_empty());
//                 let song_result: Result<Option<Vec<u8>>, JsValue> = fetch_data(format!("https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&mode=2&type=ChartFFR_music", payload.songs[0].previewhash).to_string()).await;
//                 assert!(song_result.is_ok());
//                 if let Ok(song) = song_result {
//                     assert!(song.is_some());
//                     assert!(song.unwrap().len() > 0);
//                 }
//             }
//         }
//     }
// }
