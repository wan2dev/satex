#![doc = include_str!("../docs/serve_dir.md")]

use crate::make::MakeRouteService;
use http::Extensions;
use satex_core::Error;
use satex_core::component::{Args, Configurable};
use satex_macro::make;

pub use tower_http::services::ServeDir;

#[make(kind = ServeDir)]
struct MakeServeDirRouteService {
    path: String,
    buf_chunk_size: Option<usize>,
    append_index_html_on_directories: Option<bool>,
}

impl MakeRouteService for MakeServeDirRouteService {
    type Service = ServeDir;

    fn make(&self, args: Args, _: &Extensions) -> Result<Self::Service, Error> {
        Config::with_args(args).map(|config| {
            let mut serve_dir = ServeDir::new(config.path);
            if let Some(buf_chunk_size) = config.buf_chunk_size {
                serve_dir = serve_dir.with_buf_chunk_size(buf_chunk_size);
            }
            if let Some(append_index_html_on_directories) = config.append_index_html_on_directories
            {
                serve_dir =
                    serve_dir.append_index_html_on_directories(append_index_html_on_directories);
            }
            serve_dir
        })
    }
}
