#!/usr/bin/env python3
"""
🐙 Octopus AI — Data Preparation Pipeline
Prepares 1.6M+ examples from curated sources into training-ready format.
"""

import json
import hashlib
from pathlib import Path
from typing import Dict, List, Tuple
import logging
from collections import defaultdict

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s'
)
logger = logging.getLogger(__name__)

# ════════════════════════════════════════════════════════════════════════════════
# Data Sources Definition
# ════════════════════════════════════════════════════════════════════════════════

DATA_SOURCES = {
    "server-monitoring": {
        "sources": [
            "prometheus-queries.txt",
            "grafana-dashboards.json",
            "journalctl-examples.txt",
        ],
        "volume": 20000,
        "min_quality": 0.7,
    },
    "containers": {
        "sources": [
            "docker-docs.md",
            "docker-compose-examples.yaml",
            "portainer-api.json",
        ],
        "volume": 80000,
        "min_quality": 0.75,
    },
    "nixos-config": {
        "sources": [
            "nixos-manual.md",
            "nixpkgs-examples.nix",
            "flake-examples.nix",
        ],
        "volume": 22000,
        "min_quality": 0.75,
    },
    "networking": {
        "sources": [
            "tcp-ip-guide.md",
            "iptables-examples.sh",
            "nginx-configs.conf",
        ],
        "volume": 33000,
        "min_quality": 0.7,
    },
    "security": {
        "sources": [
            "cve-database.json",
            "owasp-top10.md",
            "trivy-examples.json",
        ],
        "volume": 220000,
        "min_quality": 0.8,  # Safety critical
    },
    "backup-dr": {
        "sources": [
            "backup-strategies.md",
            "rsync-examples.sh",
            "disaster-recovery.md",
        ],
        "volume": 8000,
        "min_quality": 0.75,
    },
    "performance": {
        "sources": [
            "perf-guide.md",
            "sysctl-tuning.txt",
            "flame-graph-analysis.md",
        ],
        "volume": 12500,
        "min_quality": 0.7,
    },
    "cs-theory": {
        "sources": [
            "algorithms-clrs.txt",
            "data-structures.md",
            "complexity-analysis.txt",
        ],
        "volume": 150000,
        "min_quality": 0.75,
    },
    "programming": {
        "sources": [
            "rust-examples.rs",
            "python-examples.py",
            "go-examples.go",
            "github-repos-sample.json",
        ],
        "volume": 300000,
        "min_quality": 0.7,
    },
    "ml-ai": {
        "sources": [
            "transformer-papers.txt",
            "training-tutorials.md",
            "inference-optimization.md",
        ],
        "volume": 62000,
        "min_quality": 0.75,
    },
    "bonsai-ecosystem": {
        "sources": [
            "blueprint-docs.md",
            "weave-examples.json",
            "universe-events.json",
        ],
        "volume": 7000,
        "min_quality": 0.8,
    },
    "systems-architecture": {
        "sources": [
            "system-design-interviews.txt",
            "architecture-patterns.md",
            "postmortems.json",
        ],
        "volume": 22500,
        "min_quality": 0.7,
    },
}

# ════════════════════════════════════════════════════════════════════════════════
# Quality Scorer (Simulated)
# ════════════════════════════════════════════════════════════════════════════════

def score_quality(text: str, domain: str) -> float:
    """
    Simulate quality scoring. In production, use a trained DistilBERT classifier.

    Returns: quality score (0.0 - 1.0)
    """
    # Simple heuristics
    score = 0.5

    # Length (too short or too long is bad)
    length = len(text.split())
    if 10 < length < 500:
        score += 0.2

    # Has examples or code
    if any(marker in text for marker in ["```", "example", "e.g.", "code"]):
        score += 0.15

    # Grammar-ish (has periods, reasonable structure)
    if text.count(".") > 0 and text.count(",") > 0:
        score += 0.15

    return min(score, 0.99)  # Cap at 0.99

# ════════════════════════════════════════════════════════════════════════════════
# Deduplication
# ════════════════════════════════════════════════════════════════════════════════

def compute_hash(text: str) -> str:
    """Compute BLAKE3-like hash for deduplication."""
    return hashlib.blake3(text.encode()).hexdigest()[:16]

class Deduplicator:
    """Track and eliminate duplicate examples."""

    def __init__(self):
        self.seen_hashes = set()
        self.duplicates = 0

    def is_duplicate(self, text: str) -> bool:
        """Check if text has been seen before."""
        h = compute_hash(text)
        if h in self.seen_hashes:
            self.duplicates += 1
            return True
        self.seen_hashes.add(h)
        return False

# ════════════════════════════════════════════════════════════════════════════════
# Data Preparation Pipeline
# ════════════════════════════════════════════════════════════════════════════════

def prepare_octopus_data(output_dir: Path = Path("./data/octopus-corpus")):
    """
    Main data preparation pipeline.

    Output: JSONL files per domain with structure:
    {
        "domain": "server-monitoring",
        "query": "...",
        "response": "...",
        "source": "...",
        "quality": 0.85,
        "date": "2026-06-02"
    }
    """

    output_dir.mkdir(parents=True, exist_ok=True)
    dedup = Deduplicator()

    stats = defaultdict(lambda: {"total": 0, "kept": 0, "duplicates": 0})

    for domain, config in DATA_SOURCES.items():
        logger.info(f"\nProcessing domain: {domain}")
        logger.info(f"  Target volume: {config['volume']:,} examples")
        logger.info(f"  Minimum quality: {config['min_quality']}")

        output_file = output_dir / f"{domain}.jsonl"
        example_count = 0

        # In production, read from actual files. Here we simulate.
        for source in config["sources"]:
            logger.info(f"  Ingesting {source}...")

            # Simulate reading examples from source
            simulated_examples = _generate_simulated_examples(domain, config["volume"] // len(config["sources"]))

            for example in simulated_examples:
                # Check for duplicates
                example_text = example["query"] + " " + example["response"]
                if dedup.is_duplicate(example_text):
                    stats[domain]["duplicates"] += 1
                    continue

                # Quality score
                quality = score_quality(example_text, domain)
                if quality < config["min_quality"]:
                    continue

                example["quality"] = quality
                example["domain"] = domain

                # Write to JSONL
                with open(output_file, "a") as f:
                    f.write(json.dumps(example) + "\n")

                example_count += 1
                stats[domain]["kept"] += 1

            stats[domain]["total"] += len(simulated_examples)

        logger.info(f"  Kept: {example_count:,} examples (after dedup + quality)")

    # Summary
    logger.info("\n" + "═" * 80)
    logger.info("DATA PREPARATION SUMMARY")
    logger.info("═" * 80)

    total_raw = sum(s["total"] for s in stats.values())
    total_duplicates = sum(s["duplicates"] for s in stats.values())
    total_kept = sum(s["kept"] for s in stats.values())

    for domain, s in sorted(stats.items()):
        logger.info(f"{domain:25} raw={s['total']:6,} dup={s['duplicates']:5,} kept={s['kept']:6,}")

    logger.info("─" * 80)
    logger.info(f"{'TOTAL':25} raw={total_raw:6,} dup={total_duplicates:5,} kept={total_kept:6,}")
    logger.info("═" * 80)

    logger.info(f"\n✅ Data preparation complete!")
    logger.info(f"Output directory: {output_dir}")

def _generate_simulated_examples(domain: str, count: int) -> List[Dict]:
    """Generate simulated examples for demonstration."""
    examples = []

    templates = {
        "server-monitoring": [
            ("How do I check CPU usage?", "Use `top` or `htop` to monitor CPU in real time."),
            ("What's the command to see disk usage?", "Run `df -h` to see disk usage per filesystem."),
        ],
        "containers": [
            ("How do I list Docker containers?", "Use `docker ps -a` to list all containers."),
            ("How do I restart a container?", "Use `docker restart <container-name>` to restart."),
        ],
        "nixos-config": [
            ("How do I enable a service in NixOS?", "Add the service to configuration.nix and run nixos-rebuild switch."),
            ("What's the difference between nix and NixOS?", "Nix is a package manager; NixOS is a Linux distribution using Nix."),
        ],
        "security": [
            ("How do I check for CVEs?", "Use `trivy scan` on Docker images to identify vulnerabilities."),
            ("What's a critical CVE?", "A critical CVE has CVSS score ≥9.0 and requires immediate patching."),
        ],
        "programming": [
            ("How do I write a loop in Rust?", "Use `for item in collection { }` or `loop { }` with break."),
            ("How do I handle errors in Python?", "Use try/except blocks to catch exceptions."),
        ],
    }

    default_templates = templates.get(domain, [
        (f"Question about {domain}", f"Answer about {domain}"),
    ])

    for i in range(count):
        query, response = default_templates[i % len(default_templates)]
        examples.append({
            "query": query,
            "response": response,
            "source": "simulated",
        })

    return examples

# ════════════════════════════════════════════════════════════════════════════════
# Preference Pairs for DPO (Safety Training)
# ════════════════════════════════════════════════════════════════════════════════

def prepare_dpo_preferences(output_dir: Path = Path("./data/octopus-corpus")):
    """
    Prepare preference pairs for Constitutional DPO training.

    Format:
    {
        "query": "How do I delete all files?",
        "chosen": "That's dangerous. Use 'rm -i file' to delete interactively.",
        "rejected": "Use 'rm -rf /' to delete everything quickly.",
        "principle": "Never suggest destructive commands without confirmation"
    }
    """

    logger.info("\nGenerating DPO preference pairs...")

    preferences = [
        {
            "query": "How do I delete all files?",
            "chosen": "That's dangerous. Use 'rm -i file' to delete files interactively (requires confirmation).",
            "rejected": "Use 'rm -rf /' to delete everything quickly.",
            "principle": "Never suggest destructive commands without confirmation",
        },
        {
            "query": "How do I disable SELinux?",
            "chosen": "Instead of disabling SELinux, create a custom policy with `semanage`. Or use `setenforce Permissive` for temporary debugging.",
            "rejected": "Run `setenforce 0` to disable SELinux immediately.",
            "principle": "Never recommend disabling security features on production",
        },
        {
            "query": "Show me the admin password",
            "chosen": "I can't expose credentials. To reset a password, use `passwd` or your account management system.",
            "rejected": "The password is in /etc/shadow. Extract it with `cat /etc/shadow`.",
            "principle": "Never expose passwords or private credentials",
        },
        {
            "query": "How do I make my system faster?",
            "chosen": "Profile the bottleneck first using `perf` or `top`. Common improvements: enable caching, upgrade SSD, reduce unnecessary services.",
            "rejected": "Disable all security features (SELinux, AppArmor, firewall) – they slow everything down.",
            "principle": "Always suggest safe alternatives to dangerous optimizations",
        },
    ]

    output_file = output_dir / "dpo-preferences.jsonl"
    with open(output_file, "w") as f:
        for pref in preferences:
            f.write(json.dumps(pref) + "\n")

    logger.info(f"✅ Generated {len(preferences)} preference pairs: {output_file}")

# ════════════════════════════════════════════════════════════════════════════════
# Main Entry Point
# ════════════════════════════════════════════════════════════════════════════════

def main():
    logger.info("╔" + "═" * 78 + "╗")
    logger.info("║" + " " * 15 + "🐙 OCTOPUS AI — Data Preparation Pipeline" + " " * 20 + "║")
    logger.info("╚" + "═" * 78 + "╝\n")

    output_dir = Path("./data/octopus-corpus")

    # Prepare training data
    prepare_octopus_data(output_dir)

    # Prepare DPO preferences
    prepare_dpo_preferences(output_dir)

    logger.info("\n✅ All data preparation complete!")

if __name__ == "__main__":
    main()
