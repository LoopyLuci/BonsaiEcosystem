# Governance – Bonsai Council & Decision Making

This document outlines the governance structure for the Bonsai Ecosystem and UOSC project.

---

## The Bonsai Council

The Bonsai Council is a decentralized decision-making body responsible for:

- **Architecture decisions**: Major design changes, new subsystems, deprecated features
- **Release management**: Versioning, stability guarantees, security patches
- **Contributor guidelines**: Code review standards, contribution policies
- **Dispute resolution**: Breaking ties, settling disagreements between contributors
- **Governance changes**: Amendments to this document

### Council Members

Council membership is determined by:
- **Active contribution** (code, documentation, testing over at least 3 months)
- **Community recognition** (nominated and voted in by existing council members)
- **Expertise** in key domains: cryptography, distributed systems, language design, formal verification

Council members serve **12-month terms** and may be re-elected. A council member may step down at any time.

**Current Council** (as of 2026-06-04):
- Luci (Project Lead, Founder)
- [Open seats: 2 of 5]

### Resignation & Replacement

If a council member resigns:
1. The remaining council votes on a replacement within 2 weeks
2. Replacement requires 2/3 majority
3. If no candidate emerges, the seat remains open for up to 3 months

---

## Decision Process

### Standard Decisions (Feature Additions, Minor Changes)

1. **Discussion Phase** (1–3 days)
   - Proposal posted in the GitHub Discussions or issue
   - Community feedback collected
   - Pros/cons documented

2. **Review Phase** (1–3 days)
   - Code review by at least 2 contributors
   - Architecture review if major changes
   - Tests and documentation must be included

3. **Merge Decision**
   - Requires approval from a council member or maintainer
   - Merged to main branch

### Major Decisions (Breaking Changes, New Subsystems, Deprecations)

1. **Formal Proposal** (1 week)
   - RFC document outlining design, rationale, impact
   - Posted in GitHub Discussions with `rfc` label
   - Community feedback period: 1 week

2. **Council Review** (1–2 weeks)
   - Proposal reviewed by all council members
   - Written feedback from each member
   - Revision period if consensus not reached

3. **Council Vote**
   - Each council member casts one vote: `Approve`, `Request Changes`, `Abstain`
   - Quorum required: 2/3 of council members voting
   - **Approval requires**: 2/3 majority vote to approve

4. **Implementation**
   - Approved proposal assigned to implementer
   - Code review follows standard process
   - Documentation and tests required

5. **Release**
   - Merged to main only after council approval
   - Included in next release with migration guide if breaking

### Urgent Security Issues

For security vulnerabilities:

1. **Private Disclosure** (24–72 hours)
   - Report to security@bonsai.ecosystem
   - Emergency council session called (all members notified)
   - Fix developed and reviewed in private

2. **Fast-Track Approval** (same-day vote if needed)
   - Council votes electronically to expedite
   - 2/3 majority required (same as standard)
   - Patch released immediately upon approval

3. **Public Disclosure**
   - CVE ID requested if applicable
   - Announcement posted 24 hours after patch release
   - Incident postmortem conducted

---

## Contribution & Voting Rights

### Contributors
- Anyone who submits code, documentation, or tests
- Can participate in discussions and propose features
- Code reviewed before merge (no direct commit access)

### Maintainers
- Long-term contributors with proven track record
- Have direct merge access for non-breaking changes
- Nominated by council members, require council vote

### Council Members
- Full governance authority
- Can call emergency sessions
- Can veto decisions (used sparingly, with explanation)

---

## Conflict Resolution

If contributors disagree on a decision:

1. **Good Faith Discussion** (1 week)
   - Both sides present arguments in issue/discussion
   - Community input welcomed
   - Goal: reach consensus

2. **Mediation** (1 week)
   - If no consensus, a neutral council member facilitates discussion
   - Both sides agree to accept council decision

3. **Council Vote**
   - If mediation fails, council votes
   - 2/3 majority required
   - Decision is final and binding for 6 months, after which it can be revisited

---

## Transparency & Record Keeping

All governance decisions are recorded publicly:

- **RFC Proposals**: GitHub Discussions (tagged `rfc`)
- **Council Votes**: Linked from the relevant decision/PR
- **Meeting Notes**: Posted in a public channel (monthly summary)
- **Amendments**: This document is version-controlled and changes are audited

---

## Amending This Charter

Changes to governance rules require:

1. **Formal Proposal** following the "Major Decisions" process
2. **Council Vote**: 2/3 majority required
3. **Public Notice**: 2-week review period before taking effect

---

## Code of Conduct

All participants (council members, contributors, users) must:

- **Be Respectful**: Treat all people with respect and kindness
- **Be Inclusive**: Welcome contributors of all backgrounds and experience levels
- **Act in Good Faith**: Honest communication, no bad-faith arguments
- **Avoid Harassment**: No discrimination, harassment, or abuse
- **Report Issues**: Violations reported to security@bonsai.ecosystem

**Enforcement**:
- First violation: Written warning from council
- Second violation: Temporary suspension (1–3 months)
- Third violation: Permanent ban from project

---

## License & Governance of Bonsai IP

- **Code**: Apache 2.0 / MIT (dual-licensed)
- **Documentation**: CC-BY-SA 4.0
- **Patents**: Bonsai Project grants royalty-free license to all contributors
- **Trademarks**: "Bonsai", "Bonsai Ecosystem", "UOSC" are project trademarks. Community forks must use different names.

---

## Succession Planning

If the project lead steps down:

1. **Interim Lead**: Senior council member assumes leadership (2 weeks)
2. **Election**: Council elects new lead (2 weeks)
3. **Transition**: Knowledge transfer documented (1 month)

If the entire council becomes inactive (no decisions for 6 months), the project reverts to community stewardship under a selected maintainer.

---

## Q&A

**Q: Can I propose a feature without being a council member?**  
A: Yes. Post in GitHub Discussions or open an issue. If adopted, it will go through the standard review process.

**Q: How do I become a council member?**  
A: Contribute consistently for at least 3 months, gain recognition from the community, and be nominated by an existing council member. Then pass a council vote (2/3 majority).

**Q: What if I disagree with a council decision?**  
A: You can challenge the decision in the next governance review cycle (every 6 months), or request a second vote if circumstances change significantly.

**Q: How often does the council meet?**  
A: Monthly (virtual). Emergency sessions can be called for security or urgent decisions.

---

**Last Updated**: 2026-06-04  
**Next Review**: 2026-12-04 (annual governance audit)  
**Contact**: governance@bonsai.ecosystem
