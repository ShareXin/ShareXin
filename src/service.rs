use notification;
use twitter;
use mastodon;
use Destination;

pub fn thread(service: Destination, image: bool, message: String)
{
            //service is struct
            if service.mastodon {
                //image is true for yes image, false for no image
                if image {mastodon::image(message.clone());}
                //if false, then if its not empty, send
                else if !message.is_empty() {mastodon::toot(message.clone());}
                //if empty, cancel and notify
                else {notification::empty(service);}}
            else if service.twitter {
                if image {twitter::image(message.clone());}
                else if !message.is_empty() {twitter::tweet(message.clone());}
                else {notification::empty(service);}}
}