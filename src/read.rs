extern crate bayespam;

use bayespam::classifier::Classifier;
use csv;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut classifier: Classifier = Classifier::new();
    let mut rdr = csv::Reader::from_path("spam.csv")?;
    for result in rdr.records() {
        match result {
            Ok(record) => {
                let msg = record.get(0).unwrap();
                if msg.eq("spam") {
                    let train_msg = record.get(1).unwrap();
                    classifier.train_spam(train_msg);
                }

                println!("train spam {:#?}", record);
            }
            Err(err) => println!("{:#?}", err),
        }
    }
    // Serialize the model and save it as JSON into a file
    let mut file = File::create("my_super_model.json")?;
    classifier.save(&mut file, false)?;
    Ok(())
}
