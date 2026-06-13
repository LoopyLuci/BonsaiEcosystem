# Omnisystem Documentation Guide

**Version**: 1.0.0  
**Last Updated**: June 12, 2026  
**Maintained By**: Omnisystem Team

## Overview

This guide establishes standards for all Omnisystem documentation to ensure consistency, clarity, and accuracy across all documents.

## Core Principles

### 1. Accuracy First
- ✅ Only include verified, tested information
- ✅ Cross-reference code when making technical claims
- ✅ Update documentation when code changes
- ❌ Never include speculative or untested information
- ❌ Never claim features that don't exist
- ❌ Never use outdated information

### 2. Clarity and Precision
- Use clear, professional language
- Define technical terms on first use
- Avoid jargon without explanation
- Use active voice
- Be specific (avoid "might", "could", "possibly")

### 3. Consistency
- Use consistent terminology throughout all documents
- Follow the same formatting style
- Use consistent cross-referencing
- Maintain the same structure for similar documents

### 4. Completeness
- Provide context for all claims
- Include relevant examples
- Link to related documentation
- Include code references where applicable

### 5. Maintainability
- Update documentation with code changes
- Mark outdated information clearly
- Version all major documents
- Keep an update log

## Document Structure

### Standard Header
Every document should start with:

```markdown
# Document Title

**Repository**: Omnisystem  
**Team**: Omnisystem Team  
**Version**: X.Y.Z  
**Last Updated**: YYYY-MM-DD  
**Status**: ✅ Production Ready (or 📋 In Progress)
```

### Standard Sections
1. **Overview** - 1-2 sentence summary
2. **Table of Contents** (for documents >500 words)
3. **Getting Started** - Quick start for the topic
4. **Core Concepts** - Fundamental information
5. **Implementation** - How things work
6. **Configuration** (if applicable) - Settings and options
7. **Examples** - Code and usage examples
8. **Troubleshooting** (if applicable) - Common issues
9. **Related Documentation** - Links to other docs
10. **Glossary** (if needed) - Defined terms

### Formatting Standards

**Headings**
- Use # for main title (h1)
- Use ## for major sections (h2)
- Use ### for subsections (h3)
- Maximum nesting: h4

**Code Blocks**
```markdown
```language
code here
```
```

**Lists**
- Unordered: Use `-` or `*`
- Ordered: Use `1.`, `2.`, etc.
- Nested: Indent 2 spaces

**Emphasis**
- Bold: `**text**` for important terms
- Italics: `*text*` for emphasis
- Code: `\`code\`` for inline code

**Links**
- Format: `[text](url)` or `[text](path/to/file.md)`
- Relative links preferred for internal docs
- Always link to related documentation

**Tables**
- Use markdown table syntax
- Include header row separator
- Left-align by default, right-align for numbers

## Terminology Standards

### Repository and Organization
- **Repository name**: Omnisystem (not "the Omnisystem project" or similar)
- **Team name**: Omnisystem Team (not "BonsAI" or other variants)
- **Organization**: Omnisystem Team

### Common Terms
| Term | Standard Usage | Example |
|------|---|---|
| **Omnisystem** | The complete system | "Omnisystem is a three-layer operating system" |
| **UOSC** | Microkernel (Layer 1) | "UOSC provides kernel-level services" |
| **Kernel** | UOSC subsystem | "The kernel manages memory allocation" |
| **Layer 1/2/3** | System layers | "Layer 1 is the UOSC kernel" |
| **Service** | OS service component | "The TransferDaemon service handles P2P" |
| **Module** | Software module | "The module system enables composition" |

### Consistency Rules
- Always use "Omnisystem" (not "omnisystem")
- Always use "Omnisystem Team" (not "Omnisystem team" or "team")
- Spell out acronyms on first use: "Omnisystem Unified Operating System (UOSC)"
- Use consistent capitalization in code examples

## Accuracy Requirements

### Technical Claims
- Must cite code locations
- Must include version/date context
- Must be testable/verifiable
- Must include examples when possible

### Example Format
```markdown
### Feature X
Omnisystem provides feature X through the [FeatureModule](path/to/file.rs).

**Code Location**: `Omnisystem/crates/feature/src/lib.rs:42-87`  
**Status**: Production-ready since v1.0.0  
**Example**:
\`\`\`rust
let feature = Feature::new();
feature.do_something();
\`\`\`
```

## Document Types

### Guides
- Step-by-step instructions
- Focus on "how-to"
- Include examples
- Anticipate common issues

### References
- Comprehensive information on a topic
- Lookup documents
- Include examples
- Organized for quick scanning

### Architecture Documents
- High-level overview
- Design rationale
- Include diagrams
- Reference implementations

### API Documentation
- Function/method signatures
- Parameters and return types
- Error conditions
- Usage examples

### Status Reports
- Current state
- Completion percentage
- Notable changes
- Next steps
- Date stamp

## Quality Checklist

Before publishing any documentation:

- [ ] Header section complete and accurate
- [ ] No placeholder or "TODO" text
- [ ] All code examples tested
- [ ] All links verified (internal docs exist)
- [ ] Terminology consistent throughout
- [ ] No outdated information
- [ ] Spelling and grammar checked
- [ ] Code formatted correctly
- [ ] Headings use consistent style
- [ ] Related docs cross-referenced
- [ ] No speculative claims
- [ ] All claims verifiable
- [ ] Repository name correct (Omnisystem)
- [ ] Team name correct (Omnisystem Team)

## Cross-Referencing Standards

### Internal Links
Use relative links within docs:
```markdown
[Launcher System Blueprint](guides/LAUNCHER_SYSTEM_BLUEPRINT.md)
[Architecture Overview](ARCHITECTURE.md)
```

### Code References
Include file path and line numbers:
```markdown
**Code**: [launcher-core](../crates/launcher-core/src/lib.rs:1-50)
**Location**: Line 42-87 in src/main.rs
```

### External References
- GitHub: Include full repository URL
- Docs: Use full path
- Issues: Reference with number

## Maintenance

### Update Frequency
- Guides: Update with each major feature
- References: Update with each change
- Architecture: Update quarterly or with major changes
- Status Reports: Update weekly during active work
- API Docs: Update with each API change

### Version Control
- Date all documents
- Note changes in headers
- Keep edit history for major docs
- Archive old versions

### Deprecation
Mark deprecated content clearly:
```markdown
⚠️ **DEPRECATED**: This feature is no longer supported as of v2.0.0.
Use [Feature Y](path) instead.
```

## Examples

### Good Documentation
- ✅ "The launcher daemon listens on port 9000 (configurable via launcher.conf)"
- ✅ "Omnisystem Team maintains this module"
- ✅ Includes code examples
- ✅ Links to related docs
- ✅ States when information was last verified

### Bad Documentation
- ❌ "The launcher might listen on port 9000"
- ❌ "Some people maintain this"
- ❌ No examples or links
- ❌ Outdated without marking
- ❌ Speculative or untested claims

## Getting Help

For documentation issues:
1. Check this guide first
2. Review related documents for style patterns
3. Ask on GitHub discussions
4. Submit a PR with improvements

---

**Last Updated**: June 12, 2026  
**Status**: ✅ Production Ready
