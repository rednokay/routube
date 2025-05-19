use routube::channel::Channel;

fn main() {
    let url = "https://www.youtube.com/channel/C87nsu23lL2/";
    let name = "Best Channel";
    let c = Channel::new(0, url.into(), name.into());

    let id = c.parse_id();
    println!("{}", id.expect("Not parsed correctly"));
}
