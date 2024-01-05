mod bayes;
mod train;
mod types;

use crate::train::Train;

fn main() -> Result<(), std::io::Error> {
    let mut train = Train::new();
    train.train("train.csv").unwrap();
    train.save_vecs().unwrap();
    train.save_event().unwrap();

    let (tokens, spams, hams) = Train::from_local_vecs_to_args().unwrap();
    let content = "Noticed he said CONTENT.  Content is information aside from the website design and layout.\n\nContent on a website is best unique, as opposed to syndicated (taken from someone else).\n\nIf your site is uniquely written, that's good.  If it's well written, that's great.\n\nIf your site is writing about an interesting topic, uniquely written by you, and well-written, that's awesome -- That's the premisis for creating content for a good website.\n\nAlso, content can come in forms of raw data -- Like, if you wanted to make a travel site -- you can list the prices it takes to fly from Chicago to Pittsburg.  Or you can have a currency converter.\n\nAnother thing you can incorporate is a chatroom or message board for users to discuss the content and post their suggestions.";
    let result = bayes::identify(content, &tokens, &spams, &hams);
    println!("is spam: {:#?}", result);
    Ok(())
}
