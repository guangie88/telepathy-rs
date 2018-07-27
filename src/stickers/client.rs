use reqwest::{self, multipart::Form, RequestBuilder};
use std::path::Path;

use error::Result;
use file::File;
use util::Emoji;

use stickers::api::{
    create_url, ADD_STICKER_TO_SET_EP, CREATE_NEW_STICKER_SET_EP,
    UPLOAD_STICKER_FILE_EP,
};
use stickers::mask::MaskPosition;
use stickers::sticker::StickerFile;

fn add_png_sticker_from_path<P>(
    req: &mut RequestBuilder,
    image_path: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let form = Form::new().file("png_sticker", image_path.as_ref())?;
    req.multipart(form);
    Ok(())
}

fn add_png_sticker(
    req: &mut RequestBuilder,
    png_sticker: &StickerFile,
) -> Result<()> {
    match png_sticker {
        StickerFile::FileId(ref id) => {
            req.query(&[("png_sticker", id)]);
            Ok(())
        }
        StickerFile::Url(ref url) => {
            req.query(&[("png_sticker", url.as_str())]);
            Ok(())
        }
        StickerFile::InputFile(ref file) => {
            req.query(&[("png_sticker", file.file_id.as_str())]);
            Ok(())
        }
        StickerFile::LocalFile(ref path) => {
            add_png_sticker_from_path(req, path)
        }
    }
}

#[derive(Debug)]
pub struct Client {
    pub client: reqwest::Client,
    pub token: String,
    pub user_id: String,
    pub default_emoji: Emoji,
}

impl Client {
    pub fn add_sticker_to_set(
        &self,
        name: &str,
        png_sticker: &StickerFile,
        _emojis: &[Emoji],
        _mask_position: &Option<MaskPosition>,
    ) -> Result<()> {
        let url = create_url(&self.token, ADD_STICKER_TO_SET_EP)?;
        let mut req = self.client.get(url);
        add_png_sticker(&mut req, png_sticker)?;

        req.query(&[
            ("user_id", self.user_id.as_str()),
            ("name", name),
            ("emojis", self.default_emoji.to_string().as_str()),
        ]);

        req.send()?;
        Ok(())
    }

    pub fn create_new_sticker_set(
        &self,
        name: &str,
        title: &str,
        png_sticker: &StickerFile,
        _emojis: &[Emoji],
        _contains_masks: Option<bool>,
        _mask_position: &Option<MaskPosition>,
    ) -> Result<()> {
        let url = create_url(&self.token, CREATE_NEW_STICKER_SET_EP)?;

        let mut req = self.client.get(url);
        add_png_sticker(&mut req, png_sticker)?;

        req.query(&[
            ("user_id", self.user_id.as_str()),
            ("name", name),
            ("title", title),
            ("emojis", self.default_emoji.to_string().as_str()),
        ]);

        req.send()?;
        Ok(())
    }

    pub fn upload_sticker_file(&self, png_sticker: &Path) -> Result<File> {
        let url = create_url(&self.token, UPLOAD_STICKER_FILE_EP)?;
        let mut req = self.client.get(url);
        add_png_sticker_from_path(&mut req, png_sticker)?;
        req.query(&[("user_id", &self.user_id)]);
        Ok(req.send()?.json()?)
    }
}
