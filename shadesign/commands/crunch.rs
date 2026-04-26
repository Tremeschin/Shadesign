use crate::*;

#[derive(clap::Args)]
pub struct CrunchCommand {

    #[command(subcommand)]
    input: WordFactory,

    /// (Filter) Minimum overlap to store results
    #[arg(short, long, default_value_t=3)]
    overlap: usize,

    /// (Filter) Comparison must contain these words
    #[arg(short, long)]
    word: Option<Vec<String>>,

    /// (Filter) Only keep even length words
    #[arg(short, long)]
    even: bool,

    /// (Filter) Only keep half-word overlaps
    #[arg(short, long)]
    symmetric: bool,
}

impl CrunchCommand {
    pub fn run(&self) {
        let words: Vec<String> = self.input.get().into_iter()
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

                    // Skip non matching words early
                    if let Some(filter) = &self.word {
                        if !(filter.contains(&A) || filter.contains(&B)) {
                            continue;
                        }
                    }

                    let score = Score::compute(A.clone(), B.clone());

                    // Apply result rejection filters
                    if self.overlap > score.overlap {continue}
                    if self.symmetric && !score.symmetric {continue}
                    if self.even && !score.even {continue}

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
