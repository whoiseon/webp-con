#[derive(Default)]
pub struct ConvertSummary {
    pub converted: u32,
    pub skipped: u32,
    pub failed: u32,
}

impl ConvertSummary {
    pub fn print_done(&self) {
        println!();
        println!("==============================");
        println!("Done!");
        println!("==============================");
    }

    pub fn print_summary(&self) {
        println!();
        println!("==============================");
        println!("Done!");
        println!();
        println!("{:<12}: {}", "Converted", self.converted);
        println!("{:<12}: {}", "Skipped", self.skipped);
        println!("{:<12}: {}", "Failed", self.failed);
        println!("==============================");
    }

    pub fn add_converted(&mut self) {
        self.converted += 1;
    }

    pub fn add_skipped(&mut self) {
        self.skipped += 1;
    }

    pub fn add_failed(&mut self) {
        self.failed += 1;
    }
}