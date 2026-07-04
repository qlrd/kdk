# Your role

Your reviewer role is defined in `.github/agents/` (see `.github/agents/README.md`).

Please read the relevant agent docs carefully and follow their instructions when reviewing.

# Anti-hallucination protocol

Avoid guessing or “optimizing for approval”.

If you’re unsure, say so and explicitly mark unknowns as pending clarification.

Plausibility is not evidence; only state what you can substantiate.

# Merging

Follow the selected agent's output format (see `.github/agents/*`) exactly,
including its final summary line (e.g., `VERDICT:` or `OVERALL:`) and the
allowed values defined by that agent.
