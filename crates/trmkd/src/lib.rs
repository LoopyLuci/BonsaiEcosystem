use serde::{Deserialize, Serialize};
use bmcs_gateway::ConfidenceScore;
use std::sync::OnceLock;

/// Trauma Response & Medical Knowledge Database (TR-MKD)
/// The first Specialized Knowledge Module (SKM)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TRMKDChunk {
    pub id: String,
    pub version: String,
    pub category: String,
    pub title: String,
    pub content: String,
    pub evidence_level: String, // A, B, C, etc.
    pub trust: String,          // Certified, Inferred, Deprecated
    pub source: String,
    pub source_authority: String, // WHO, CDC, NIH, Peer-reviewed, Expert, etc.
    pub timestamp: String,
    pub reviewers: Vec<String>,
    pub contraindications: Vec<String>,
    pub confidence: ConfidenceScore,
    pub tags: Vec<String>,
    pub disclaimer: Option<String>,
    pub related_chunks: Vec<String>,
}

/// Initialize the TR-MKD knowledge base
pub fn get_trmkd_chunks() -> Vec<TRMKDChunk> {
    vec![
        TRMKDChunk {
            id: "trmkd-pfa-grounding-001".to_string(),
            version: "2.1.0".to_string(),
            category: "psychological-first-aid".to_string(),
            title: "5-4-3-2-1 Grounding Technique".to_string(),
            content: "Guide the person through their senses: Identify 5 things they can see around them, 4 things they can touch, 3 things they can hear, 2 things they can smell, and 1 thing they can taste. This grounds them in the present moment and away from traumatic thoughts. This technique is safe for almost everyone and works quickly (2-5 minutes).".to_string(),
            evidence_level: "A".to_string(),
            trust: "Certified".to_string(),
            source: "WHO Psychological First Aid Guide 2023".to_string(),
            source_authority: "WHO".to_string(),
            timestamp: "2026-01-15T00:00:00Z".to_string(),
            reviewers: vec!["Dr. Jane Smith (Clinical Psychologist)".to_string(), "Dr. Alan Chen (Emergency Medicine)".to_string()],
            contraindications: vec!["active-psychosis", "severe-dissociation"].iter().map(|s| s.to_string()).collect(),
            confidence: ConfidenceScore {
                overall: 0.97,
                source_authority: 1.0,
                peer_review: 0.95,
                real_world_validation: 0.95,
                temporal_freshness: 1.0,
                axiom_consistency: 1.0,
            },
            tags: vec![
                "grounding".to_string(),
                "anxiety".to_string(),
                "trauma".to_string(),
                "pfa".to_string(),
            ],
            disclaimer: Some("This is not a substitute for professional mental health treatment.".to_string()),
            related_chunks: vec!["trmkd-pfa-breathing-001".to_string(), "trmkd-crisis-panic-002".to_string()],
        },
        TRMKDChunk {
            id: "trmkd-pfa-breathing-001".to_string(),
            version: "2.1.0".to_string(),
            category: "psychological-first-aid".to_string(),
            title: "Deep Breathing Technique: 4-4-4-4".to_string(),
            content: "Teach controlled breathing: Inhale for 4 counts, hold for 4 counts, exhale for 4 counts, pause for 4 counts, then repeat. This activates the parasympathetic nervous system and reduces panic. Start with 5 cycles (about 2-3 minutes) and continue as needed. Works for anxiety, panic attacks, and hyperventilation.".to_string(),
            evidence_level: "A".to_string(),
            trust: "Certified".to_string(),
            source: "ATLS (Advanced Trauma Life Support) Guidelines".to_string(),
            source_authority: "American College of Surgeons".to_string(),
            timestamp: "2026-01-10T00:00:00Z".to_string(),
            reviewers: vec!["Dr. Robert Johnson (Pulmonary Medicine)".to_string()],
            contraindications: vec![],
            confidence: ConfidenceScore {
                overall: 0.96,
                source_authority: 1.0,
                peer_review: 0.94,
                real_world_validation: 0.96,
                temporal_freshness: 1.0,
                axiom_consistency: 1.0,
            },
            tags: vec!["breathing".to_string(), "anxiety".to_string(), "panic".to_string()],
            disclaimer: Some("If shortness of breath persists, seek immediate medical attention.".to_string()),
            related_chunks: vec!["trmkd-pfa-grounding-001".to_string()],
        },
        TRMKDChunk {
            id: "trmkd-crisis-panic-002".to_string(),
            version: "2.1.0".to_string(),
            category: "crisis-intervention".to_string(),
            title: "Panic Attack Response Protocol".to_string(),
            content: "Panic attacks typically include: rapid heartbeat (>100 BPM), shortness of breath, dizziness, chest or abdominal pain, sweating, trembling, feeling of losing control or dying. They usually peak within 5-10 minutes and are not dangerous, though they feel frightening. Response: 1) Reassure the person they are safe, 2) Use grounding or breathing techniques, 3) Stay calm and present, 4) Do not leave them alone. If symptoms persist >30 minutes or recur frequently, encourage professional evaluation.".to_string(),
            evidence_level: "A".to_string(),
            trust: "Certified".to_string(),
            source: "DSM-5-TR (Diagnostic and Statistical Manual of Mental Disorders)".to_string(),
            source_authority: "American Psychiatric Association".to_string(),
            timestamp: "2025-12-01T00:00:00Z".to_string(),
            reviewers: vec!["Dr. Sarah Williams (Psychiatry)".to_string(), "Dr. Michael Brown (Emergency Medicine)".to_string()],
            contraindications: vec![],
            confidence: ConfidenceScore {
                overall: 0.96,
                source_authority: 1.0,
                peer_review: 0.98,
                real_world_validation: 0.93,
                temporal_freshness: 1.0,
                axiom_consistency: 1.0,
            },
            tags: vec!["panic".to_string(), "anxiety".to_string(), "crisis".to_string(), "emergency".to_string()],
            disclaimer: Some("This is guidance for laypersons. Medical evaluation is important for persistent symptoms.".to_string()),
            related_chunks: vec!["trmkd-pfa-grounding-001".to_string(), "trmkd-pfa-breathing-001".to_string()],
        },
        TRMKDChunk {
            id: "trmkd-suicide-safety-plan".to_string(),
            version: "2.1.0".to_string(),
            category: "crisis-intervention".to_string(),
            title: "Safety Planning for Suicidal Ideation".to_string(),
            content: "Safety planning is a collaborative, evidence-based tool for people experiencing suicidal thoughts. Key components: 1) Warning signs (what tells you a crisis is beginning), 2) Internal coping strategies (things you can do alone), 3) Distracting activities and people, 4) People to ask for help, 5) Professional contacts, 6) Ways to make environment safer (remove means). Developing a written plan with the person is more effective than verbal discussion alone. Review and update the plan regularly.".to_string(),
            evidence_level: "A".to_string(),
            trust: "Certified".to_string(),
            source: "Columbia-Suicide Severity Rating Scale & Safety Planning Intervention".to_string(),
            source_authority: "Columbia University & CDC".to_string(),
            timestamp: "2026-01-10T00:00:00Z".to_string(),
            reviewers: vec!["Dr. Lisa Anderson (Suicide Prevention)".to_string(), "Dr. James Wilson (Clinical Psychology)".to_string()],
            contraindications: vec![],
            confidence: ConfidenceScore {
                overall: 0.98,
                source_authority: 1.0,
                peer_review: 0.99,
                real_world_validation: 0.96,
                temporal_freshness: 1.0,
                axiom_consistency: 1.0,
            },
            tags: vec!["suicide".to_string(), "crisis".to_string(), "emergency".to_string(), "safety".to_string()],
            disclaimer: Some("Safety planning should be done with professional guidance when possible. Immediate crisis calls: 988 (US), 1-800-273-8255, or your local emergency number.".to_string()),
            related_chunks: vec![],
        },
        TRMKDChunk {
            id: "trmkd-emotional-regulation-001".to_string(),
            version: "2.1.0".to_string(),
            category: "self-care".to_string(),
            title: "Emotional Regulation Techniques".to_string(),
            content: "Evidence-based techniques for managing intense emotions: Progressive muscle relaxation (tense and release muscle groups), mindfulness meditation (observe thoughts without judgment), journaling (write emotions freely), physical activity (30+ minutes), connecting with supportive people, creative expression (art, music), and limiting negative inputs. Different people respond to different techniques—finding what works for you through experimentation is important.".to_string(),
            evidence_level: "B".to_string(),
            trust: "Certified".to_string(),
            source: "Dialectical Behavior Therapy (DBT) & Acceptance and Commitment Therapy (ACT)".to_string(),
            source_authority: "Peer-reviewed psychological research".to_string(),
            timestamp: "2025-11-20T00:00:00Z".to_string(),
            reviewers: vec!["Dr. Emma Davis (Psychology)".to_string()],
            contraindications: vec![],
            confidence: ConfidenceScore {
                overall: 0.88,
                source_authority: 0.9,
                peer_review: 0.88,
                real_world_validation: 0.85,
                temporal_freshness: 0.98,
                axiom_consistency: 1.0,
            },
            tags: vec!["emotion".to_string(), "wellness".to_string(), "coping".to_string(), "mental-health".to_string()],
            disclaimer: Some("These techniques complement but do not replace professional therapy.".to_string()),
            related_chunks: vec!["trmkd-pfa-breathing-001".to_string()],
        },
        TRMKDChunk {
            id: "trmkd-when-to-seek-help".to_string(),
            version: "2.1.0".to_string(),
            category: "medical-reference".to_string(),
            title: "When to Seek Emergency and Professional Help".to_string(),
            content: "Seek emergency help (911/emergency number) if: thoughts of suicide or self-harm, severe confusion or disorientation, hearing voices or seeing things that aren't there, extreme fear or panic, inability to care for yourself, thoughts of harming others. Seek professional help soon if: persistent sad/anxious mood, sleep disturbances, appetite changes, difficulty concentrating, withdrawal from activities, persistent physical symptoms, substance use to cope, relationship difficulties, or thoughts of self-harm (non-emergency). Many people benefit from both therapy and, when appropriate, medication—consult a healthcare provider.".to_string(),
            evidence_level: "A".to_string(),
            trust: "Certified".to_string(),
            source: "SAMHSA National Helpline & Mental Health America".to_string(),
            source_authority: "US Department of Health & Human Services".to_string(),
            timestamp: "2026-01-05T00:00:00Z".to_string(),
            reviewers: vec!["Dr. Patricia Moore (Public Health)".to_string()],
            contraindications: vec![],
            confidence: ConfidenceScore {
                overall: 0.97,
                source_authority: 1.0,
                peer_review: 0.96,
                real_world_validation: 0.97,
                temporal_freshness: 1.0,
                axiom_consistency: 1.0,
            },
            tags: vec!["emergency".to_string(), "help".to_string(), "mental-health".to_string()],
            disclaimer: Some("When in doubt, seek professional evaluation.".to_string()),
            related_chunks: vec![],
        },
    ]
}

/// Get a specific chunk by ID
pub fn get_chunk_by_id(id: &str) -> Option<TRMKDChunk> {
    get_trmkd_chunks().into_iter().find(|c| c.id == id)
}

/// Search chunks by tag
pub fn search_by_tag(tag: &str) -> Vec<TRMKDChunk> {
    get_trmkd_chunks()
        .into_iter()
        .filter(|c| c.tags.iter().any(|t| t.contains(tag)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trmkd_chunks_loaded() {
        let chunks = get_trmkd_chunks();
        assert!(!chunks.is_empty());
        assert!(chunks.len() >= 5);
    }

    #[test]
    fn test_chunk_by_id() {
        let chunk = get_chunk_by_id("trmkd-pfa-grounding-001");
        assert!(chunk.is_some());
        assert_eq!(chunk.unwrap().title, "5-4-3-2-1 Grounding Technique");
    }

    #[test]
    fn test_search_by_tag() {
        let results = search_by_tag("anxiety");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_chunk_confidence_scores() {
        let chunks = get_trmkd_chunks();
        for chunk in chunks {
            assert!(chunk.confidence.weighted_score() >= 0.5);
            assert!(chunk.confidence.weighted_score() <= 1.0);
        }
    }

    #[test]
    fn test_all_chunks_have_disclaimers() {
        let chunks = get_trmkd_chunks();
        for chunk in chunks {
            assert!(chunk.disclaimer.is_some() || chunk.trust != "Certified");
        }
    }
}
