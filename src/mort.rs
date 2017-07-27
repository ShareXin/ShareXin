use notification;
use send;

pub fn thread(mort: bool, morti: bool, tweet: String)
{
            if mort {
                if morti {
                    send::toot_img(tweet.clone());
                }
                else if !tweet.is_empty() {
                    send::toot(tweet.clone());
                }
                else {
                    notification::empty(mort);
                }
            }
            else {
                if morti {
                    send::twitter_img(tweet.clone());
                }
                else if !tweet.is_empty() {
                    send::twitter(tweet.clone());
                }
                else {
                    notification::empty(mort);
                }
            }
}