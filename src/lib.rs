use std::fs;

const DATA_DIR: &str = "data/";

pub fn read_json(path_to_json: &str) -> anyhow::Result<String> {
    let json_string = fs::read_to_string(path_to_json)?;
    Ok(json_string)
}

pub mod channel {
    use reqwest;
    use serde::{Deserialize, Serialize};
    use std::fs;

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
        pub feed_path: String,
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

        pub fn set_feed_path(&mut self) -> anyhow::Result<()> {
            if self.channel_id.is_empty() {
                anyhow::bail!("Channel id must be set before setting a feed path");
            }
            self.feed_path = DATA_DIR.to_owned() + &self.channel_id;
            Ok(())
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

        pub fn write_feed_to_file(&self) -> anyhow::Result<()> {
            if self.feed_path.is_empty() {
                anyhow::bail!("Feed path not set!")
            }
            if let Some(f) = &self.feed {
                fs::write(&self.feed_path, f)?;
                Ok(())
            } else {
                anyhow::bail!("Feed not set, cannot write to file")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::DATA_DIR;

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
    #[ignore]
    fn set_feed() {
        // Torsten Heinrich
        let url = "https://www.youtube.com/channel/UC9kZ6FlOQfusBV8LS2x2fAA/";
        let name = "Torsten Heinrich";
        let mut c = channel::Channel::new(0, url.into(), name.into());

        assert_eq!(None, c.feed);

        let _ = c.set_feed();

        assert_ne!(None, c.feed);
    }

    #[test]
    fn set_feed_path() {
        let url = "https://www.youtube.com/channel/UC9kZ6FlOQfusBV8LS2x2fAA/";
        let name = "Torsten Heinrich";
        let mut c = channel::Channel::new(0, url.into(), name.into());

        assert!(c.feed_path.is_empty());

        let e = c.set_feed_path();

        assert!(e.is_err());

        let i = c.set_id();
        assert!(i.is_ok());

        let e = c.set_feed_path();

        assert!(e.is_ok());
        assert_eq!(c.feed_path, DATA_DIR.to_owned() + "UC9kZ6FlOQfusBV8LS2x2fAA");
    }

    #[test]
    fn write_feed_to_file() {
        let url = "https://www.youtube.com/channel/UC9kZ6FlOQfusBV8LS2x2fAA/";
        let name = "Torsten Heinrich";
        let mut c = channel::Channel::new(0, url.into(), name.into());
        let feed = "This is some very interesting feed\n".to_owned();
        c.feed = Some(feed);

        let e = c.write_feed_to_file();

        assert!(e.is_err());

        let _ = c.set_id();
        let _ = c.set_feed_path();

        let w = c.write_feed_to_file();

        assert!(w.is_ok());

        let file_content = fs::read_to_string(c.feed_path).expect("Reading gone wrong");

        assert_eq!(c.feed.unwrap(), file_content);
    }
}
