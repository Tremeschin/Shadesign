use crate::*;

#[derive(clap::Args)]
pub struct CrunchCommand {

    #[command(subcommand)]
    words: WordFactory,

    /// How many words to score
    #[arg(short='t', long, default_value_t=50000)]
    total: usize,

    /// Minimum overlap to store results
    #[arg(short='m', long, default_value_t=3)]
    min: usize,
}

impl CrunchCommand {
    pub fn run(&self) {
        let words: Vec<String> = self.words.get()
            .into_iter().take(self.total)
            .filter(|w| w.chars().all(|c| c.is_alphabetic()))
            .map(|s| s.to_lowercase())
            .collect();

        let progress = ProgressBar::new(words.len() as u64)
            .with_style(utils::progress("Searching"));

        // Note: Comparison order matters!
        let mut scores: Vec<Score> =
            words.clone()
            .into_par_iter()
            .progress_with(progress)
            .map(|A| {
                let mut scores: Vec<Score> = Vec::new();

                for B in words.clone() {
                    let score = Score::compute(A.clone(), B.clone());

                    // Skip boring overlaps
                    if score.overlap < self.min {
                        continue;
                    }

                    scores.push(score);
                }

                scores
            })
            .flatten()
            .collect();

        scores.sort();

        for score in scores {
            println!("{}", serde_json::to_string(&score).unwrap());
        }
    }
}
