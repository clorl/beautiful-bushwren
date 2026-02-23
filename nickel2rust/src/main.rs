use nickel2rust::{ generate, GenConfig
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match generate(GenConfig {
        root: "./",
        patterns: vec! [
            "**/example_01.ncl",
        ]
    }) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{:?}", e);
            Ok(())
        }
    }
}
