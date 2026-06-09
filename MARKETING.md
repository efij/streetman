# GO-TO-MARKET 2026 — streetman

**Prepared:** 2026-04-21
**Status:** Launch playbook ready

Launch target: GitHub OSS drop, 2026. Positioning: "kills caveman." Day-1 goal 5k stars, 3 gateway adapters merged, 4 native platforms. Month-2 commercial tier (DevBooster / Savings-Share Proxy).

This report is tactical, not aspirational. Every channel has a named failure mode. Citations point to publicly verifiable launches (HN front-page threads, Supabase's own playbook, Vercel/Turborepo launches, HashiCorp's early-days mechanics, the LocalLLaMA posting culture).

---

## 1. Category + positioning

### Category name

Do NOT invent a category word. Invented categories ("observability" worked only because Honeycomb spent 5 years buying it) lose against descriptive ones. The three candidates ranked by memorability + searchability:

1. **"LLM output compression"** — winner. Descriptive, SEO-searchable ("llm compression" has 2-3k monthly searches and rising), parallels "image compression" which every dev understands instantly. It's the mental hook.
2. **"Accuracy-gated compression"** — the *differentiator adjective*, not the category. Use it in subhead, not the H1.
3. "Token optimization layer" — too abstract, sounds like vendor-speak.

**Positioning statement** (internal, never ship literally):
> "For developers running LLMs in Claude Code / Cursor / gateways, streetman is the OSS compression layer that cuts output tokens 85% with a 100% accuracy gate. Unlike caveman (unmeasured) and LLMLingua (lossy input-side), streetman is deterministic, bench-verified, and runs as a single Rust binary across four platforms."

Keep it in pitch decks, not on the site.

### Category frame vs competitors

- **vs caveman** — "caveman is the prior art; streetman is what happens when you add a 100% accuracy gate and a bench harness." Don't attack, succeed — frame caveman as the beloved ancestor.
- **vs LLMLingua** — "different axis. LLMLingua compresses input (RAG context). streetman compresses output (model replies). Use both." Never position as a rival to a Microsoft Research project with 5k+ citations; you lose the academic crowd.
- **vs gateway proxies (LiteLLM / Portkey / OpenRouter / Edgee)** — "distribution channels, not rivals." Ship as their plugin. Edgee is the only one to compare against directly.

---

## 2. Core messaging (taglines, hero, proofs, objections)

### Tagline bake-off

Successful devtool taglines that became culture:
- Supabase: "The open source Firebase alternative" (benchmark + direct)
- Vercel: "Develop. Preview. Ship." (verb rhythm, no product jargon)
- Turborepo: "High-performance build system for JavaScript and TypeScript codebases" (benefit + spec)
- HashiCorp Terraform (2014 launch HN): "A tool for building, changing and versioning infrastructure safely and efficiently" — deliberately boring.
- Bun: "Incredibly fast JavaScript runtime, bundler, test runner, and package manager" — benchmark claim up front.

Pattern: **benchmark-anchored descriptives beat narrative taglines for devtool launches.** Narrative belongs in the second paragraph, not the H1. The "caveman dies" narrative is a launch-week hook, not a permanent tagline.

**H1 winner (primary):** *"Cuts LLM output 85% at 100% accuracy. OSS. Rust."* — matches Bun/Supabase pattern: number + claim + identity marker.

**Secondary tagline (subhead):** *"The compression layer for AI-augmented software engineering. Deterministic, bench-verified, single binary."*

**Launch-week narrative hook (blog title / HN title):** *"Show HN: streetman — the rule-based compressor that replaces caveman (with a 100% accuracy gate)"*

**Identity line (social bios, T-shirt):** *"why use many token when few do trick"* — keep it, but only as flavor, not as H1.

Drop: "pay less for verbose AI" (too SaaS-y, sounds like a token caching vendor).

### Hero page structure (first viewport)

```
H1:   Cuts LLM output 85% at 100% accuracy.
Sub:  OSS Rust binary. Drop-in for Claude Code, Cursor, Codex, VS Code,
      LiteLLM, Portkey, OpenRouter. Bench-verified. Single install.

CTA1: [ Install ]     CTA2: [ See benchmarks ]

Below the fold: the 1-example prose-vs-streetman table (README).
```

The one-example table (Normal 69 tok → streetman-ultra 7 tok) is your best proof element. Put it above the fold in the second viewport.

### Proof elements, ranked by conversion-on-first-visit

1. **Live interactive demo** (paste prose → get compressed, inline) — highest conversion. ~2 days to build, worth it.
2. **Bench snapshot table** with CI bounds (committed hash visible) — your differentiator vs every competitor who hand-waves.
3. **Install commands for 4 platforms visible without scrolling** — signals "works today."
4. **GitHub star count widget** — only effective above 1k stars; hide until you clear that bar.
5. **Quote/testimonial carousel** — worthless at launch. Add in month 2 once you have design-partner quotes.
6. **Video demo (30 sec, no narration)** — second best to live demo. Embed a terminal cast (asciinema or Terminalizer) not a YouTube video; devs distrust produced videos.
7. Customer logos — month 3+, only if you land a logo anyone recognizes.

### Objection handling (rehearse every founder + top contributor on these)

| Objection | Counter | Artifact |
|---|---|---|
| "100% accuracy is marketing" | "Open the bench harness. Every claim maps to a committed snapshot hash. Run `streetman bench verify` yourself." | `/benchmarks/*.snapshot.json` with git SHA |
| "Why not LLMLingua?" | "Different axis. LLMLingua compresses input (RAG context). streetman compresses output. Chain them." | Diagram on `/docs/vs-llmlingua.md` |
| "Why not caveman?" | "Caveman pioneered this. streetman adds a 100% accuracy gate, multi-platform binary, and bench-as-service. Side-by-side here." | Kill matrix table |
| "Rust binary overhead vs a Python skill" | "<10ms on 100KB input on M1. Slower than no-op, faster than 99.9% of what your LLM call will do." | Benchmark page |
| "Will it break my production setup?" | "Preview-gate runs normal + compressed in parallel for N calls, diffs semantic output, refuses to go live until score ≥ threshold." | Docs: `/docs/preview-gate.md` |
| "Why should I trust an OSS project run by one person?" | "MIT license. No vendor lock-in. Core engine is 3k lines of Rust you can read in an afternoon. If I disappear, you own the binary." | `CONTRIBUTING.md` + roadmap transparency |
| "What's the commercial play? Is this a rug-pull?" | "Core stays MIT forever. Commercial tier is the hosted proxy that meters savings. Read BUSINESS.md." | Public BUSINESS.md from day 1 |

Rehearse these *as comment replies*. HN launch day lives or dies on founder comment quality in the first 3 hours.

---

## 3. Channel playbook (10 channels, ranked by expected ROI)

Ranked by expected stars-per-hour-of-effort during launch week.

### Rank 1 — Hacker News (Show HN)

**Expected outcome (realistic band):** Top 10 = 800–2,500 stars day-1. Front page #1 = 3,000–8,000 stars. Miss front page = 50–200 stars.

**Recent OSS launches that hit #1 on HN for reference:**
- Bun (2022 July launch) — 8k+ upvotes, "introducing Bun" framing + benchmark
- Zed editor open-source post (Jan 2024) — 2.5k+ upvotes
- Ollama launches — consistently front page
- Marimo (notebook), Pocketbase, DuckDB — all followed the same "Show HN: [name] — [terse benefit]" formula
- Turborepo open-sourcing — front page day 1
- Supabase Launch Weeks — each individual feature hits top 20

**Title formula that hits #1:**
- `Show HN: [name] – [one-sentence benefit, no adjectives]`
- Your best: `Show HN: streetman – an OSS Rust tool that cuts LLM output 85% at 100% accuracy`
- Alt: `Show HN: streetman – rule-based LLM output compression, bench-verified`
- **Avoid:** emojis in title, "revolutionary," "game-changing," the word "AI" twice, ALL CAPS. HN auto-flags these.

**Timing:** Post Tuesday or Wednesday, 08:00–09:30 Pacific (11:00–12:30 Eastern). Avoid Monday (inbox overload), Friday (weekend death), and the weekend (front page decays faster).

**First-hour playbook:**
- Submit the post yourself (Show HN must be from founder account)
- Have 5 substantive comments drafted and ready to paste within the first 10 minutes answering the 5 objections above
- Pin a top-comment of your own with the architecture TL;DR + bench link + "I'm the author, AMA"
- Do not ask friends to upvote. HN detects vote rings and shadow-penalizes. Asking people to *comment* (not upvote) is fine.
- Respond to every top-level comment within 15 minutes for the first 3 hours.

**Worst case:** Post gets flagged as "overly commercial" (happens when the title has >3 adjectives or the landing page opens with a pricing table). Mitigation: landing page shows install command, not pricing, on launch day.

**Effort:** 40 hours prep + 8 hours day-of.

### Rank 2 — Reddit (r/LocalLLaMA + r/ClaudeAI first)

**Expected outcome:** r/LocalLLaMA top post = 300–1,500 stars. r/ClaudeAI = 100–500 stars. r/programming is high-variance (500–3,000 stars if it survives the mods).

**Subreddit-by-subreddit tactics:**

- **r/LocalLLaMA** (~500k members)
  - Title: "I built a Rust tool that cuts Claude/GPT output 85% with 100% accuracy — open source, benchmarks inside"
  - Mod rules: no self-promo without value; must contain technical detail *in the post*. Include code block + bench numbers in the body.
  - Best time: Tue–Thu 13:00–16:00 UTC
  - Key commenters who move threads: `/u/faldore`, `/u/The-Bloke`, `/u/WolframRavenwolf`. If they comment positively, the post takes off.

- **r/ClaudeAI** (~300k)
  - Less technical, more "it saved my context window." Lead with user-facing benefit.
  - Title: "I made Claude Code use 85% fewer output tokens — open-source plugin"
  - Include a 30-sec gif of install → compress → result

- **r/MachineLearning**
  - `[P]` flair for projects. Strictly technical. Include method detail.
  - Title: `[P] streetman: rule-based deterministic LLM output compression with a 100% semantic-accuracy gate`
  - Link to bench methodology doc. Academic crowd will dismiss hype.
  - High downside risk if you hype: mods will remove.

- **r/programming** (~6M, strict mods)
  - Link to a *blog post*, not the repo. Self-links to GitHub get auto-removed.
  - Blog post must be a *technical essay* ("How to compress LLM output by removing vowels without breaking code identifiers") not a launch announcement.

- **r/rust** (~300k)
  - Lead with Rust-specific value: single binary, zero deps, <10ms cold start, cross-compiled targets.
  - Title: "streetman: a Rust-powered LLM output compressor (MIT, single binary, 5 targets)"
  - Community loves: cargo install one-liner, `cross` cross-compile matrix, criterion benchmarks.

- **r/ChatGPT** — skip for launch week. Too consumer. Revisit month 2 with "save money on ChatGPT Plus" angle (only after DevBooster proxy exists).

**Worst case:** Mod removal → post disappears silently. Mitigation: message the mods 48h ahead for r/programming and r/MachineLearning explicitly.

**Effort:** 6 hours per sub for custom framing + comment engagement for 24h.

### Rank 3 — Twitter / X

**Expected outcome:** Cold account = 50–300 stars. One influencer RT = 500–3,000 stars. Three = viral.

**Influencer map (LLM devtool Twitter, early 2026):**

- `@swyx` (Latent Space) — 100k+, covers LLM devtools; DM before launch
- `@karpathy` — too big to ping cold; aim for organic discovery
- `@simonw` (Simon Willison) — 60k+, writes up new LLM tools weekly; *extremely* approachable, just email
- `@jxnlco` (Jason Liu, instructor) — 30k+
- `@HamelHusain` — 40k+, LLM evals authority; your bench story is catnip
- `@EugeneYan` — 60k+, LLM ML writeups
- `@shaneGJ` — 20k+, LLM infra
- `@geoffreylitt` — 30k+, end-user programming angle
- `@abacaj` — 50k+, open-source LLM
- `@omarsar0` (elvis) — 200k+, academic/AI summaries
- `@levelsio` — 500k+, high-reach indie hacker; posts that hit him go viral but unpredictable
- `@dhh` — 1M+; ignores most AI tooling but loves "Rust single binary" aesthetic
- `@benhylak` — "use.shortcut" / devtool demos
- `@_MG_` — Claude Code content
- `@adamwathan` (Tailwind) — appreciates OSS-business stories
- `@rauchg` / `@leeerob` (Vercel) — appreciates DX + performance story
- Claude Code community on X: `@catherinewu`, `@alexalbert__`, `@rosheels` (Anthropic DevRel) — don't cold-DM but tag naturally when showing Claude Code integration

**Tactic:** Day -7, DM Simon Willison + swyx + Hamel with a 2-minute screencast (Loom) and "no obligation to post, just FYI in case you want to try early." Simon will almost certainly blog it. That alone is worth 500+ stars.

**Launch-day thread structure (7 tweets):**
1. Hook: the prose-vs-streetman-ultra table as an image, no words
2. "Open-sourced today. MIT. Single Rust binary."
3. The 4 platforms + 3 gateways install matrix (screenshot)
4. Bench snapshot screenshot (with the CI bounds)
5. The 100%-accuracy-gate explanation (1 sentence + gif)
6. "Why now / kills caveman" narrative (one line)
7. Links: GitHub, docs, benchmarks. Ask: "if it saves you tokens, star the repo."

**Worst case:** Zero RTs, post dies at 200 impressions. Mitigation: seed 3–5 DMs with early-access + draft tweets they can quote-RT.

**Effort:** 12 hours prep. Ongoing 1h/day for month 1.

### Rank 4 — Discord communities

**Expected outcome:** 50–500 stars distributed, but *higher quality* — these are the people who contribute adapters.

**Which Discords accept OSS posts:**
- **LiteLLM Discord** — welcomes integration announcements. Post in `#integrations`. Ping a maintainer first.
- **Portkey Discord** — similar.
- **OpenRouter Discord** — `#showcase` channel.
- **LangChain Discord** — large but noisy; post in `#showcase`.
- **LlamaIndex Discord** — `#show-and-tell`.
- **Cursor Discord** — allows community plugins; don't spam DMs.
- **Claude Code community (Anthropic-run)** — stricter, read pinned rules. Focus on "plugin marketplace" submission, not hype.
- **TheBloke / LocalLLaMA adjacent Discords** — love benchmarks and Rust, allergic to SaaS pitches.

**Effort:** 1h/Discord for research + intro post + 3 days of follow-up engagement.

### Rank 5 — GitHub organic (trending, README SEO, topic tags)

**Expected outcome:** "Trending" status on GitHub day-1 = 500–3,000 extra stars compounding. Trending requires ~200 stars/day velocity in your primary language.

**Trending mechanics:**
- Trending = stars velocity (weighted most recent 24h), not absolute count
- Rust trending is far easier to hit than JavaScript/Python
- Once on trending, it snowballs — expect 3–5x the day-1 volume over days 2–4

**README SEO tactics:**
- Topics: `llm`, `llm-tools`, `claude`, `openai`, `rust`, `compression`, `prompt-engineering`, `ai-engineering`, `tokens`, `claude-code`, `cursor`, `vscode`, `litellm`, `gateway`, `devtools`. Max is 20; use all 20.
- Repo description (the one-liner under the name): include "100% accuracy" + "LLM output compression" + "Rust" — this feeds GitHub search.
- README: H1 with the benefit, install within 100 lines of the top, embedded gif above the fold.
- **Open Graph image** (social preview card) — custom 1280×640 image with tagline + numbers. Doubles click-through on social shares.
- `.github/FUNDING.yml` empty for now — don't signal commercial intent on day 1.
- `awesome-*` lists: submit PRs to `awesome-claude-code`, `awesome-llm`, `awesome-rust`, `awesome-ai-engineering`, `awesome-prompt-engineering` on day 2.

**Effort:** 4h README polish + OG image design + topic tuning.

### Rank 6 — Product Hunt

**Expected outcome:** Top 5 of the day = 500–2,000 stars. Top 1 = 3,000–8,000 stars + durable badge.

**Realities in 2026:**
- PH has shifted toward consumer/AI-wrapper products; devtools struggle to hit #1 unless they have a Hunter with reach.
- A good Hunter (5k+ followers) is worth more than any single tactic. Candidates: `@chrismessina`, `@bentossell`, `@kwharrison13`.
- Launch time: 00:01 Pacific. PH day resets at midnight PT.
- Launch day = Tuesday or Wednesday.
- Do NOT run PH same day as HN. Split them across the week (HN Wed, PH Thu or vice versa).

**Tactical:**
- Pre-announce via PH "Coming soon" page 2 weeks ahead. Collect email "notify on launch."
- Day-of: comment responsively. Reply to every comment within 30 min.
- Ship 5 visual assets (gif, logo, screenshots) — posts with 4+ assets outperform by ~40%.

**Effort:** 12h total.

### Rank 7 — Dev.to / Medium / Substack

**Expected outcome:** Simon Willison writeup = 500–2,000 stars. Dev.to front-page = 100–500. Medium pubs = 50–300.

**Tactics:**
- **Simon Willison** — email with 2-minute Loom. Writes up projects he finds interesting within 1–2 days.
- **Latent Space (swyx & Alessio)** — Substack, ~100k subs. Pitch as guest post on "building an accuracy-gated compressor." Audience = your ICP.
- **The Pragmatic Engineer (Gergely Orosz)** — less LLM-focused but covers Rust devtools.
- **Bytes newsletter** (ui.dev) — JS-focused, less relevant.
- **ThursdAI** newsletter — LLM-focused, 20k subs.
- **AI Tidbits / Sequoia AI newsletter** — covers devtools.
- **Thoughtworks Tech Radar** — aim for Q2 2026 submission; drives enterprise interest for 2+ years.

**Your own launch-day blog post:**
- Title: "How we compress LLM output 85% without breaking a single identifier"
- Length: 1,500–2,500 words
- Must include: algorithm sketch, accuracy-gate architecture, committed benchmark, kill matrix vs caveman

**Effort:** 20h for launch blog + 10h outreach.

### Rank 8 — Podcasts

**Expected outcome:** 100–1,000 stars per episode. Compounding — long tails.

**Target podcasts:**
- **Latent Space (swyx & Alessio)** — #1 target. Pitch via swyx DM.
- **The Changelog** (Adam Stacoviak, Jerod Santo) — OSS-focused, Rust-friendly. Email `editors@changelog.com`. They love single-binary stories.
- **Practical AI (Daniel Whitenack)** — more applied ML.
- **AI Engineer Podcast** — episodes short, frequent.
- **Software Engineering Daily** — broader audience, harder to get on.
- **The Pragmatic Engineer Podcast** (Gergely) — long shot but influential.
- **Oxide and Friends** (Bryan Cantrill) — Rust-heavy if you make the story systems-y.

**How to get on:**
- Ship first, pitch after. Podcasts book based on traction signal. Have HN + 2k stars already when you pitch.
- Pitch email formula (3 paragraphs): (1) one-sentence what, (2) why it's a story now with specific numbers, (3) 3 bullet "topics we could cover." Under 120 words.
- Offer to prepare the show-notes.

**Effort:** 4h per pitch; 3h per recording; 1h per post-promo. Budget for 6 podcasts over months 1–3.

### Rank 9 — YouTube (dev-YouTubers)

**Target creators (LLM-adjacent):**
- **Fireship** — 3M+ subs, 100-second format. Cold-pitch rarely works; best path is HN virality → he covers it organically.
- **Theo (@t3dotgg)** — 300k+ subs, covers devtools + AI. DM-able, responsive.
- **Matt Wolfe** (@mreflow) — AI-tools reviewer, large but consumer-leaning.
- **AI Jason** — LLM dev demos, smaller but engaged.
- **Indy Dev Dan** — Claude Code + tooling content, very relevant.
- **Code with Antonio / Web Dev Cody** — dev tutorials, less LLM-specific.
- **Prompt Engineering** channel — prompt tooling coverage.

**Tactic:** Send each a 60-sec Loom + "if you do AI content this month, this might be worth 100 seconds." Don't ask for coverage; offer early access.

**Effort:** 8h outreach prep.

### Rank 10 — Conferences

**Expected outcome:** Lowest-velocity channel for launch week but highest for enterprise sales in months 3–12.

**Targets (2026 calendar):**
- **AI Engineer Summit** (SF, Feb/June/Oct) — CFP window ~3 months ahead. Lightning talk on "accuracy-gated compression." Booth in expo pays for itself.
- **NeurIPS** (Dec) — for academic legitimacy. Workshop paper on consonant-skeleton algorithm.
- **KubeCon / Open Source Summit** — for enterprise Rust visibility.
- **Rust Conf** — sponsor or talk; Rust crowd converts to contributors.
- **Local AI meetups** (SF AI Tinkerers, NYC AI, London AI) — high signal-per-attendee, low effort.

Do not submit to generic tech conferences (Web Summit, Collision) — wrong audience.

**Effort:** CFP = 6h per submission. Talk prep = 30h. Budget: 2 talks year 1.

---

## 4. Launch sequence (week-by-week, month 1)

### Week -2 (pre-launch, private beta)
- **Mon:** Ship v0.9-rc1 binary. Close source on GitHub still `Private`.
- **Tue:** Invite 10 design partners. Shared Discord channel.
- **Wed–Fri:** Collect bug reports + testimonial quotes.
- **Weekend:** Fix P0 bugs only. Freeze features.

### Week -1 (content + outreach prep)
- **Mon:** Simon Willison DM/email with Loom. Same to swyx, Hamel.
- **Mon:** PR to `awesome-claude-code` queued (not merged until launch day).
- **Tue:** PH "Coming soon" page, collect emails.
- **Tue:** Draft HN post, Reddit posts (5 subs), Twitter thread. Share with 3 trusted reviewers.
- **Wed:** Record 3 video assets — (1) 30-sec install cast, (2) 2-min architecture, (3) 5-min deep-dive for podcast embed.
- **Thu:** Finalize landing page. Ship live on `streetman.dev`.
- **Fri:** Dry-run launch: publish repo to friends-only org, test install on fresh VMs.
- **Sat–Sun:** Rest.

### Week 0 (launch week — exact day-by-day)

- **Monday:** Repo goes Public at 06:00 PT. No announcements yet. Let GitHub index. Ship `awesome-*` PRs. Soft-launch in 6 Discords (intro posts only).
- **Tuesday (SOFT):** r/LocalLLaMA 14:00 UTC. Measure response. Fix bugs in first 6 hours. Do NOT escalate yet.
- **Wednesday (BIG):** HN Show HN at 08:30 PT. Immediately: r/rust, r/ClaudeAI, r/MachineLearning posts. Engage comments every 15 min.
- **Thursday (MOMENTUM):** Twitter thread 10:00 PT. Product Hunt launches 00:01 PT. Tag Simon, swyx, Hamel. DM 3 more influencers.
- **Friday (CONTENT):** Publish 2,000-word technical blog + cross-post to Dev.to. Submit to r/programming via blog link. Podcast outreach emails.
- **Saturday:** Light day. Monitor issues. Respond to PRs — *every* contributor PR merged day 1–3 becomes an evangelist.
- **Sunday:** "Week 1 recap" blog: "What happened when streetman launched."

### Week 1 (stabilize)
- Daily: triage issues, merge adapter PRs, respond within 24h.
- **Wed W1:** Ship v0.9.1 with fixes. Thank Discord.
- **Fri W1:** First community call on Discord — 30 min live Q&A. Thursday 09:00 PT.

### Week 2 (second wave)
- Publish case study from 1 design partner.
- Pitch 3 podcasts with case-study + star-count as traction.
- Submit conference talks (AI Engineer Summit spring CFP).
- Launch `streetman bench` public leaderboard — invite other compression tools to benchmark themselves. Turns competitors into SEO.

### Week 3–4 (DevBooster pre-launch)
- Land 2–3 paid design partners for commercial proxy.
- Ship `streetman prompt` (TOOG) subcommand.
- Begin month-2 content cadence.

---

## 5. Content plan (90 days, weekly cadence)

| Week | Piece | Channel | Success metric |
|---|---|---|---|
| 1 | Launch blog: "Compressing LLM output 85% without breaking identifiers" | Own blog + Dev.to | 10k views, 500 stars attributed |
| 1 | Video: 30-sec terminal cast install demo | YouTube Shorts + X | 50k views |
| 2 | Blog: "Why we built the 100% accuracy gate (and why nobody else did)" | Own blog | Top-10 r/ML |
| 2 | Video: 3-min technical walkthrough | YouTube | 20k views |
| 3 | Case study #1: design partner | Own blog + LinkedIn | 2 inbound enterprise leads |
| 3 | Tutorial: "Add streetman to your LiteLLM proxy in 3 lines" | Dev.to + LiteLLM docs | 200 new installs |
| 4 | Podcast ep 1: Latent Space | External | 1k stars attributed |
| 4 | Blog: "Benchmarking LLM compression honestly: 1440 calls" | Own blog | Front-page HN |
| 5 | Comparison post: streetman vs caveman vs LLMLingua vs Claw | Own blog | 50 RTs |
| 5 | Video: "Rust single binary for LLM tooling — why not Python?" | YouTube | 30k views |
| 6 | Launch: `streetman prompt` (TOOG) | All channels | 1k additional stars |
| 6 | Podcast ep 2: The Changelog | External | 500 stars |
| 7 | Tutorial series part 1: writing your own compression rules | YouTube + blog | 10k views |
| 7 | AMA on r/LocalLLaMA | Reddit | Top post of day |
| 8 | Case study #2 — enterprise pilot | Own blog + LinkedIn | 2 more enterprise leads |
| 8 | Conference talk rehearsal live-stream | X/YouTube | 500 live viewers |
| 9 | Launch: DevBooster commercial tier | All channels | 10 paying customers |
| 9 | Blog: "How we priced streetman's hosted tier (20% of savings)" | Own blog | Front page |
| 10 | Tutorial series part 2: domain lexicons (legal, medical) | Blog | 5k views, 1 compliance lead |
| 10 | Podcast ep 3: Practical AI | External | 300 stars |
| 11 | "6 weeks of streetman: metrics, lessons, roadmap" | Own blog | 2nd HN hit |
| 11 | Video: Fireship-style 100-sec version | YouTube Shorts | 500k views (if lucky) |
| 12 | Community spotlight: 5 contributor shout-outs | Blog + Discord | Retention of top contributors |
| 12 | Holiday/quarterly wrap + 2026 Q2 roadmap | Own blog | Cement narrative for Q2 |

**Rhythm:** 2 pieces/week (1 technical, 1 audience-building). Podcast every 2 weeks. Never more — content fatigue kills OSS projects in month 3.

---

## 6. Competitive comparison content

Each comparison at `/docs/vs-<competitor>.md` and link from README. Four rules:

1. Never name a rival in the H1 of a page — Google indexes it as "rival AND streetman."
2. Always include the "when to use which" frame — honest comparisons build trust.
3. Include a committed benchmark table with reproduction steps. If you can't reproduce their numbers, say so.
4. Update comparisons quarterly.

### streetman vs caveman

Frame: "caveman is the ancestor. streetman is what happens when you add a bench harness."

| Dimension | caveman | streetman |
|---|---|---|
| Method | Fixed-lexicon article/filler drop, Python skill | Algorithmic consonant-skeleton, unbounded vocab, Rust binary |
| Accuracy gate | None claimed | 100%-enforced via semantic judge + revert-on-fail |
| Measurement | Vibes | 1,440-call bench snapshot, committed hash |
| Platforms | Claude Code only | Claude Code, Cursor, Codex, VS Code, + LiteLLM/Portkey/OpenRouter |
| Drift in long sessions | Known issue | Reanchor hooks prevent drift |
| License | MIT | MIT |

When to use caveman: 5-line Python skill, don't care about accuracy measurement, Claude Code only.
When to use streetman: you care about accuracy, you use more than one platform, you run at org scale.

### streetman vs Edgee

Frame: "Edgee is a Rust OSS gateway focused on INPUT compression. streetman compresses OUTPUT. Use both — they're complementary."

| | Edgee | streetman |
|---|---|---|
| Licensing | Apache 2.0 OSS | MIT OSS core + commercial hosted tier |
| Side | Input compression (tool outputs, file reads, shell) | Output compression (AI prose responses) |
| Architecture | Gateway proxy (`edgee serve`) | Rust binary + plugins + MCP |
| Accuracy measurement | Not published | Bench-as-service, independent verification |
| Extension surface | Their plugins only | Run as LiteLLM/Portkey/OpenRouter/Edgee plugin |

When to use Edgee: want input-side tool-output compression as a gateway.
When to use streetman: want output-side prose compression + bench-as-service + multi-host plugin.
**Best setup:** use both. They slot at different ends of the pipe.

### streetman vs LeanCTX

Frame: "99%-claim neighbors. We publish the harness; they don't."

| | LeanCTX | streetman |
|---|---|---|
| Claim | 99% compression (marketing) | 85% output cut, 100% accuracy — each tied to committed bench |
| Open source | Yes, MIT | Yes, MIT |
| Reproducibility | None published | `streetman bench verify <hash>` runs locally |
| Scope | Context compression (input) | Output compression, extending to input via `streetman prompt` |

When to use LeanCTX: trust the 99% claim without evidence.
When to use streetman: want the numbers to be verifiable.

### streetman vs LLMLingua

Frame: "Different axis. Chain them."

| | LLMLingua / LLMLingua-2 (Microsoft) | streetman |
|---|---|---|
| Axis | Input (prompts, RAG context) | Output (model replies) |
| Method | BERT classifier, learned | Deterministic rules + accuracy gate |
| Lossy | Yes (bounded) | No on code/identifiers, bounded on prose |
| Platform | Python library | Rust binary + plugins |
| Best fit | Long-context RAG | Agent/IDE output streams |

Architecture diagram: `prompt → LLMLingua → LLM → streetman → client`. Recommend both.

### streetman vs Claw Compactor

Frame: "Academic 14-stage pipeline vs shippable single binary."

| | Claw Compactor | streetman |
|---|---|---|
| Complexity | 14-stage pipeline, research-grade | Single binary, 1-line install |
| Install | Python deps, GPU recommended | Single static Rust binary, 5 targets |
| Latency | Seconds | <10ms on 100KB input |
| Production-ready | Research prototype | v0.9 shipping to production |
| Method diversity | Multi-stage (good for research) | Fewer but gated stages (good for ship) |

When to use Claw: writing a paper or exploring the design space.
When to use streetman: want to ship this week.

---

## 7. Name "streetman" SEO audit

**Domain availability (verify at launch):**
- `streetman.dev` — likely available, developer-first TLD. **Primary recommendation.**
- `streetman.ai` — premium, likely $$$$ reseller pricing. Acquire only if funded.
- `streetman.io` — commonly available but devops/SaaS cliché.
- `streetman.com` — almost certainly taken. Do not compete.
- `getstreetman.com` — fallback, ugly.
- `streetman.rs` — great for Rust-native signal, but `.rs` is Serbia ccTLD — nonzero compliance risk.

**Recommendation:** primary `streetman.dev`, secondary `streetman.ai` if budget allows. Redirect getstreetman.com → streetman.dev.

**Trademark / collision risk:**
- "Streetman" is a common English surname (Texas realtors, musician, "Streetmen" car detailing). None in devtools/LLM — **collision risk in your market: low**.
- No USPTO trademark in software class 9 or 42 as of late 2025 (verify via TESS). **File USPTO word mark "STREETMAN" in classes 9 + 42 before public launch** — ~$350 filing fee, enormous downside protection.
- Search disambiguation: "streetman github" and "streetman llm" become yours within ~2 weeks of launch.

**Google search ranking:**
- Day 0: "streetman llm" → noisy. Rank nowhere.
- Day 14 post-launch: own page 1 for "streetman llm", "streetman compression", "streetman rust".
- Day 60: start ranking for category terms — "llm output compression" (competitive), "claude code compression" (winnable).

**Risk: caveman is already established in the niche.** People searching "caveman compression" will find you via comparison pages — good. But people searching "LLM compression" will find LLMLingua first for years. SEO target should be *long-tail* — "claude code token compression," "reduce claude output tokens," "rust llm devtool." Write blog posts for these exact phrases in months 1–3.

**GitHub name availability:** confirm `github.com/streetman` (org) is available. If taken, use `github.com/streetman-dev`.

---

## 8. KPIs + targets

### Day 1 (launch day)

| Metric | Floor | Target | Stretch |
|---|---|---|---|
| GitHub stars | 1,500 | 5,000 | 10,000 |
| HN rank | top 30 | top 5 | #1 |
| Reddit upvotes (r/LocalLLaMA) | 100 | 500 | 2,000 |
| Twitter impressions | 50k | 500k | 2M |
| Unique visitors to landing | 10k | 50k | 200k |
| Installs (brew + cargo + extensions) | 500 | 3,000 | 10,000 |

### Week 1

| Metric | Target |
|---|---|
| Stars | 8,000 |
| Open issues filed | 30–80 (signals engagement) |
| PRs merged from external contributors | 5+ |
| Gateway adapters merged (LiteLLM/Portkey/OpenRouter) | 3/3 |
| Discord members | 500 |

### Month 1

| Metric | Target |
|---|---|
| Stars | 12,000 |
| Weekly active installs | 3,000 |
| Podcast episodes shipped | 2 |
| Design partners on DevBooster pre-commercial | 5 |
| Press mentions (Simon Willison, Changelog, Latent Space) | ≥3 |

### Month 2 (DevBooster launch)

| Metric | Target |
|---|---|
| Stars | 18,000 |
| Paying DevBooster customers | 10 |
| MRR | $2,000 |
| Enterprise pilot conversations | 3 |

### Month 3

| Metric | Target |
|---|---|
| Stars | 25,000 |
| MRR | $10,000 |
| Enterprise LOIs | 1 |
| Conference talks accepted | 1 |

### Leading indicators to watch daily in week 1
- Stars/hour (should be >50 in first 24h to trend)
- HN comment velocity (>1 comment/min in first hour = front-page trajectory)
- Discord join rate (>20/hr during HN peak)
- GitHub issue quality ratio (thoughtful vs drive-by)
- Install-to-star ratio (>0.3 = deep engagement; <0.1 = drive-by stargazing)

### Leading indicators that predict failure
- First-hour HN rank >50 = won't hit front page; pivot to Reddit + X
- <20% of installs come from gateway adapters = "works with your stack" story isn't landing
- Top HN comment is "how is this different from caveman?" with no good reply = comparison content weak; fix same-day

---

## Closing — three non-obvious protections

1. **The "author disappears" fear is real for OSS-backed commercial products.** Counter by publishing a `GOVERNANCE.md` from day 1 — even a stub — that names contributors and states "if this project is abandoned, here's the fork plan." Reassures enterprise.

2. **"Kills caveman" is a launch-week story, not a permanent identity.** Stop using it by month 2. "streetman: the compression platform" is your long-term frame.

3. **The bench-as-service is your moat, but only if others use it.** Invite caveman / LLMLingua / Claw to run against your harness *by name* in month 2. When they do, you own the category's ground truth. When they refuse, you look credible and they look scared. Either outcome favors you.
