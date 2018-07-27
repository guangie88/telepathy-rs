use file::File;
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
pub enum StickerFile {
    FileId(String),
    Url(Url),
    InputFile(File),
    LocalFile(PathBuf),
}
