use custom_error::custom_error;
use std::error::Error;
use reqwest::{self, header};

custom_error! {
    pub ZoomError
    Networking{source: reqwest::Error} = "network error: {source}",
    Dezoomer{source: DezoomerError} = "Dezoomer error: {source}",
    NoLevels = "A zoomable image was found, but it did not contain any zoom level",
    NoTile = "Could not get any tile for the image",
    Image{source: image::ImageError} = "invalid image error: {source}",
    TileDownloadError{uri: String, cause: Box<ZoomError>} = "error with tile {uri}: {cause}",
    PostProcessing{source: Box<dyn Error>} = "unable to process the downloaded tile: {source}",
    Io{source: std::io::Error} = "Input/Output error: {source}",
    Yaml{source: serde_yaml::Error} = "Invalid YAML configuration file: {source}",
    TileCopyError{x:u32, y:u32, twidth:u32, theight:u32, width:u32, height:u32} =
                                "Unable to copy a {twidth}x{theight} tile \
                                 at position {x},{y} \
                                 on a canvas of size {width}x{height}",
    MalformedTileStr{tile_str: String} = "Malformed tile string: '{tile_str}' \
                                          expected 'x y url'",
    NoSuchDezoomer{name: String} = "No such dezoomer: {name}",
    InvalidHeaderName{source: header::InvalidHeaderName} = "Invalid header name: {source}",
    InvalidHeaderValue{source: header::InvalidHeaderValue} = "Invalid header value: {source}",
    AsyncError{source: tokio::task::JoinError} = "Unable get the result from a thread: {source}",
    BufferToImage{source: BufferToImageError} = "{}",
}

custom_error! {
    pub BufferToImageError
    Image{source: image::ImageError} = "invalid image error: {source}",
    PostProcessing{e: Box<dyn Error + Send>} = "unable to process the downloaded tile: {e}",
}

custom_error! {pub DezoomerError
    NeedsData{uri: String}           = "Need to download data from {uri}",
    WrongDezoomer{name:&'static str} = "The '{name}' dezoomer cannot handle this URI",
    Other{source: Box<dyn Error>}    = "Unable to create the dezoomer: {source}"
}

impl DezoomerError {
    pub fn wrap<E: Error + 'static>(err: E) -> DezoomerError {
        DezoomerError::Other { source: err.into() }
    }
}