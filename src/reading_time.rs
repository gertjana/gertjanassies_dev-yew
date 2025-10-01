const WPM: u16 = 200;

pub fn calculate_reading_time(content: &str) -> u64 {
    let word_count = count_words(content);
    if word_count == 0 {
        return 0;
    }

    let minutes = word_count as f64 / WPM as f64;
    (minutes * 60.0) as u64
}

fn count_words(content: &str) -> usize {
    content
        // Remove markdown headers
        .lines()
        .map(|line| {
            let line = line.trim();
            // Skip lines that are purely markdown syntax
            if line.starts_with('#')
                || line.starts_with("```")
                || line.starts_with("---")
                || line.is_empty()
            {
                String::new()
            } else {
                // Remove inline markdown formatting
                line.replace("**", "") // Bold
                    .replace("*", "") // Italic
                    .replace("`", "") // Code
                    .replace("_", "") // Underscore formatting
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
        // Split by whitespace and filter empty strings
        .split_whitespace()
        .filter(|word| !word.is_empty())
        .count()
}

/// Format reading time in a human-readable way
pub fn _format_reading_time(seconds: u64) -> String {
    format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
}
