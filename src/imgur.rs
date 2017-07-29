use notification;
use Destination;

pub fn send() {
    let imgur = Destination {
        twitter: false, mastodon: false,
        imgur: true };
    notification::image_sent(imgur, "hi", "/tmp/hi");
}