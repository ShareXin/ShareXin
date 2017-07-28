use notification;
use send;

pub fn thread(service: bool, image: bool, tweet: String)
{
            //service is true for mastodon, false for twitter
            if service {
                //image is true for yes image, false for no image
                if image {send::toot_img(tweet.clone());}
                //if false, then if its not empty, send
                else if !tweet.is_empty() {send::toot(tweet.clone());}
                //if empty, cancel and notify
                else {notification::empty(service);}}
            else {
                if image {send::twitter_img(tweet.clone());}
                else if !tweet.is_empty() {send::twitter(tweet.clone());}
                else {notification::empty(service);}}}