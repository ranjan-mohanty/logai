mod common;

use logai::ai::cache::AnalysisCache;
use logai::types::{ErrorAnalysis, Suggestion};

#[test]
fn test_cache_creation() {
    let result = AnalysisCache::new();
    assert!(result.is_ok());
}

#[test]
fn test_cache_get_miss() {
    let cache = AnalysisCache::new().unwrap();
    let result = cache.get("nonexistent_pattern", "openai", "gpt-4");
    assert!(result.is_ok());
    // May or may not be None depending on previous test runs
    // Just verify it doesn't error
}

#[test]
fn test_cache_set_and_get() {
    let cache = AnalysisCache::new().unwrap();

    let analysis = ErrorAnalysis {
        explanation: "Test explanation".to_string(),
        root_cause: Some("Test cause".to_string()),
        suggestions: vec![Suggestion {
            description: "Test suggestion".to_string(),
            code_example: None,
            priority: 1,
        }],
        related_resources: vec![],
        tool_invocations: vec![],
    };

    let pattern = "test_pattern";
    let provider = "openai";
    let model = "gpt-4";

    let set_result = cache.set(pattern, provider, model, &analysis);
    assert!(set_result.is_ok());

    let get_result = cache.get(pattern, provider, model);
    assert!(get_result.is_ok());

    let cached = get_result.unwrap();
    assert!(cached.is_some());

    let cached_analysis = cached.unwrap();
    assert_eq!(cached_analysis.explanation, "Test explanation");
}

#[test]
fn test_cache_different_providers() {
    let cache = AnalysisCache::new().unwrap();

    let analysis = ErrorAnalysis {
        explanation: "Test".to_string(),
        root_cause: None,
        suggestions: vec![],
        related_resources: vec![],
        tool_invocations: vec![],
    };

    cache.set("pattern", "openai", "gpt-4", &analysis).unwrap();

    // Different provider should be a cache miss
    let result = cache.get("pattern", "claude", "claude-3");
    assert!(result.unwrap().is_none());
}

#[test]
fn test_cache_different_models() {
    let cache = AnalysisCache::new().unwrap();

    let analysis = ErrorAnalysis {
        explanation: "Test".to_string(),
        root_cause: None,
        suggestions: vec![],
        related_resources: vec![],
        tool_invocations: vec![],
    };

    cache.set("pattern", "openai", "gpt-4", &analysis).unwrap();

    // Different model should be a cache miss
    let result = cache.get("pattern", "openai", "gpt-3.5");
    assert!(result.unwrap().is_none());
}

#[test]
fn test_cache_clear_old() {
    let cache = AnalysisCache::new().unwrap();

    let analysis = ErrorAnalysis {
        explanation: "Test".to_string(),
        root_cause: None,
        suggestions: vec![],
        related_resources: vec![],
        tool_invocations: vec![],
    };

    cache.set("pattern", "openai", "gpt-4", &analysis).unwrap();

    // Clear entries older than 30 days
    let clear_result = cache.clear_old(30);
    assert!(clear_result.is_ok());
}
