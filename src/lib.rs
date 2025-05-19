use std::fs;

const DATA_DIR: &str = "data/";

pub fn read_json(path_to_json: &str) -> anyhow::Result<String> {
    let json_string = fs::read_to_string(path_to_json)?;
    Ok(json_string)
}

pub mod channel {
    use reqwest;
    use serde::{Deserialize, Serialize};

    use crate::DATA_DIR;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PipePipe {
        app_version: String,
        app_version_int: i32,
        channels: Vec<Channel>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Channel {
        service_id: u16,
        url: String,
        name: String,

        #[serde(skip)]
        feed_path: String,
        #[serde(skip)]
        channel_id: String,
        #[serde(skip)]
        pub feed: Option<String>,
    }

    impl Channel {
        // For debugging
        pub fn new(service_id: u16, url: String, name: String) -> Self {
            Channel {
                service_id,
                url,
                name,
                feed_path: String::new(),
                channel_id: String::new(),
                feed: None,
            }
        }

        fn set_feed_path(&mut self) {
            self.feed_path = DATA_DIR.to_owned() + &self.channel_id;
        }

        pub fn parse_id(&self) -> anyhow::Result<String> {
            const CHANNEL_PREFIX: &str = "/channel/";

            let start_id;

            if let Some(i) = self.url.find(CHANNEL_PREFIX) {
                start_id = i + CHANNEL_PREFIX.len();
            } else {
                anyhow::bail!("Could not parse channedl id");
            }

            if let Some(i) = self.url[start_id..].find('/') {
                Ok(self.url[start_id..start_id + i].into())
            } else {
                Ok(self.url[start_id..].into())
            }
        }

        pub fn set_id(&mut self) -> anyhow::Result<()> {
            self.channel_id = self.parse_id()?;
            Ok(())
        }

        pub fn pull_feed(&self) -> anyhow::Result<String> {
            let feed_url = format!(
                "https://www.youtube.com/feeds/videos.xml?channel_id={}",
                self.parse_id()?
            );

            let response = reqwest::blocking::get(&feed_url)?.text()?;

            Ok(response)
        }

        pub fn set_feed(&mut self) -> anyhow::Result<()> {
            let feed = self.pull_feed()?;
            self.feed = Some(feed);
            Ok(())
        }

        pub fn write_feed_to_file() {
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::channel;

    #[test]
    fn without_slash() {
        let name = "Cool Youtuber";
        let url = "https://www.youtube.com/channel/U8ecCwsd92";
        let c = channel::Channel::new(0, url.into(), name.into());
        assert_eq!(
            "U8ecCwsd92".to_owned(),
            c.parse_id().expect("Shout not error here")
        );
    }

    #[test]
    fn with_slash() {
        let name = "Cool Youtuber";
        let url = "https://www.youtube.com/channel/U8ecCwsd92/";
        let c = channel::Channel::new(0, url.into(), name.into());
        assert_eq!(
            "U8ecCwsd92".to_owned(),
            c.parse_id().expect("Shout not error here")
        );
    }

    #[test]
    fn set_feed() {
        // Torsten Heinrich
        let url = "https://www.youtube.com/channel/UC9kZ6FlOQfusBV8LS2x2fAA/";
        let name = "Torsten Heinrich";
        let mut c = channel::Channel::new(0, url.into(), name.into());

        assert_eq!(None, c.feed);

        let _ = c.set_feed();

        assert_ne!(None, c.feed);
    }
}
