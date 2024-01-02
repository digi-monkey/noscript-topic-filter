extern crate bayespam;

use bayespam::classifier::Classifier;
use csv;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut classifier: Classifier = Classifier::new();
    let mut rdr = csv::Reader::from_path("train.csv")?;
    for result in rdr.records() {
        match result {
            Ok(record) => {
                let msg = record.get(1).unwrap();
                if msg.eq("4") {
                    let train_msg = record.get(2).unwrap();
                    classifier.train_spam(train_msg);

                    let s1 =record.get(2).unwrap(); 
                    classifier.train_spam(s1);

                    let s2 =record.get(3).unwrap(); 
                    classifier.train_spam(s2);

                    println!("train spam {:#?}", s1);
                }else {
                    let train_msg = record.get(2).unwrap();
                    classifier.train_ham(train_msg);

                    let s1 =record.get(2).unwrap(); 
                    classifier.train_ham(s1);

                    let s2 =record.get(3).unwrap(); 
                    classifier.train_ham(s2); 
                }
            }
            Err(err) => println!("{:#?}", err),
        }
    }
    // Serialize the model and save it as JSON into a file
    let mut file = File::create("my_super_model.json")?;
    classifier.save(&mut file, false)?;
    Ok(())
}
