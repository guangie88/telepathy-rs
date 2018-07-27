pub const TELEGRAM_BOT_API: &str = "https://api.telegram.org/bot";
pub const ADD_STICKER_TO_SET_EP: &str = "/addStickerToSet";
pub const CREATE_NEW_STICKER_SET_EP: &str = "/createNewStickerSet";
pub const UPLOAD_STICKER_FILE_EP: &str = "/uploadStickerFile";

use error::Result;
use url::Url;

pub fn create_url<T, S>(token: T, endpoint: S) -> Result<Url>
where
    T: AsRef<str>,
    S: AsRef<str>,
{
    Ok(Url::parse(&format!(
        "{}{}{}",
        TELEGRAM_BOT_API,
        token.as_ref(),
        endpoint.as_ref(),
    ))?)
}
