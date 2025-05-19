use routube::channel::Channel;

fn main() {
    // Torsten Heinrich
    let url = "https://www.youtube.com/channel/UC9kZ6FlOQfusBV8LS2x2fAA/";
    let name = "Torsten Heinrich";
    let c = Channel::new(0, url.into(), name.into());

    let feed = c.pull_feed();
    println!("{}", feed.expect("Not parsed correctly"));
}
