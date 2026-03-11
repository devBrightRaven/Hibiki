use super::types::RefinedTranscript;
use std::collections::HashMap;

/// Language-specific filler words to remove.
struct FillerRules {
    fillers: Vec<String>,
}

pub struct RuleBasedRefiner {
    rules_by_language: HashMap<String, FillerRules>,
}

impl RuleBasedRefiner {
    pub fn new() -> Self {
        let mut rules_by_language = HashMap::new();

        // English fillers
        rules_by_language.insert(
            "en".to_string(),
            FillerRules {
                fillers: vec![
                    "um", "uh", "uhh", "umm", "hmm", "like,", "you know,", "I mean,", "so,",
                    "basically,", "actually,", "literally,",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
            },
        );

        // Traditional Chinese / Mandarin fillers
        rules_by_language.insert(
            "zh".to_string(),
            FillerRules {
                fillers: vec![
                    "那個", "就是", "然後", "嗯", "啊", "對", "就是說", "怎麼說", "反正", "所以說",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
            },
        );

        Self { rules_by_language }
    }

    pub fn refine(&self, text: &str, language: &str) -> RefinedTranscript {
        let lang_key = if language.starts_with("zh") {
            "zh"
        } else {
            language
        };

        let refined = match self.rules_by_language.get(lang_key) {
            Some(rules) => {
                let mut result = text.to_string();
                for filler in &rules.fillers {
                    result = result.replace(filler, "");
                }
                // Clean up double spaces left by removal
                while result.contains("  ") {
                    result = result.replace("  ", " ");
                }
                result.trim().to_string()
            }
            None => text.to_string(),
        };

        let refinement_applied = refined != text;

        RefinedTranscript {
            original_text: text.to_string(),
            refined_text: refined,
            refinement_applied,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_filler_removal() {
        let refiner = RuleBasedRefiner::new();
        let result = refiner.refine("um so I think uh we should do this", "en");
        assert!(result.refinement_applied);
        assert!(!result.refined_text.contains("um"));
        assert!(!result.refined_text.contains("uh"));
        assert!(result.refined_text.contains("I think"));
    }

    #[test]
    fn test_chinese_filler_removal() {
        let refiner = RuleBasedRefiner::new();
        let result = refiner.refine("嗯就是我覺得那個應該這樣做", "zh-TW");
        assert!(result.refinement_applied);
        assert!(!result.refined_text.contains("嗯"));
        assert!(!result.refined_text.contains("那個"));
        assert!(result.refined_text.contains("我覺得"));
    }

    #[test]
    fn test_no_refinement_needed() {
        let refiner = RuleBasedRefiner::new();
        let result = refiner.refine("This is a clean sentence.", "en");
        assert!(!result.refinement_applied);
        assert_eq!(result.refined_text, result.original_text);
    }

    #[test]
    fn test_unknown_language_passthrough() {
        let refiner = RuleBasedRefiner::new();
        let result = refiner.refine("um hello", "xx");
        assert!(!result.refinement_applied);
    }
}
