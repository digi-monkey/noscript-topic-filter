mod classifier;
use classifier::Classifier;
use std::fs::File;

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
	classifier.save_event().unwrap();
	Ok(())
}
