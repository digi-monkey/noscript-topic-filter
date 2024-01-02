extern crate bayespam;

use std::fs::File;

use bayespam::classifier::Classifier;


fn main() -> Result<(), std::io::Error> {
    let file_path = "my_super_model.json";
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error opening the file: {}", e);
        }
    };
    // Create a new classifier with an empty model
    let classifier: Classifier = Classifier::new_from_pre_trained(&mut file).unwrap();

    // Identify a typical spam message
    let spam = "I recommend looking into nspawn - it's a systemd enabled service that runs LXC containers

    here is an example of a deployment script i built most of:
    
    GitHub - relaytools/relay-tools-images: build and deploy repository for relay tools(build and deploy repository for relay tools. ..)
    
    it's a bit more manual than docker but you aren't forced into the use of aufs or whatever overlay filesystem so it's got a bit less overhead at that level
    
    performance is pretty much near the same as running the server not in a container, it mostly only controls access to kernel resources via namespaces";
    let score = classifier.score(spam);
    let is_spam = classifier.identify(spam);
    println!("{:.4}", score);
    println!("{}", is_spam);

    // Identify a typical ham message
    let ham = "Hi Bob, can you send me your machine learning homework?";
    let score = classifier.score(ham);
    let is_spam = classifier.identify(ham);
    println!("{:.4}", score);
    println!("{}", is_spam);

    Ok(())
}
