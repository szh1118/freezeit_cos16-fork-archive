<!--
Sync Impact Report
Version change: none -> 1.0.0
Modified principles: N/A (initial ratification)
Added principles:
- I. Simplicity and Necessity
- II. Verified Interfaces and Confirmed Requirements
- III. Reuse Existing Architecture, Skills, MCP, and Tools
- IV. Active Verification Before Completion
- V. Complete and Honest Delivery
Added sections:
- Eight Honors and Eight Shames
- Development Workflow and Quality Gates
Removed sections: N/A
Templates requiring updates:
- ✅ .specify/templates/constitution-template.md restored from the installed Spec Kit core pack
- ✅ .specify/templates/spec-template.md updated for verified increments and clarification discipline
- ✅ .specify/templates/plan-template.md restored and updated with constitution gates
- ✅ .specify/templates/tasks-template.md restored and updated for required verification tasks
- ✅ .specify/templates/checklist-template.md restored from the installed Spec Kit core pack
- ✅ .specify/templates/commands/*.md restored from the installed Spec Kit core pack and normalized to `.specify/memory/constitution.md`
- ✅ .specify/templates/commands/constitution.md updated for generic agent guidance
- ✅ .specify/templates/commands/specify.md updated for verified-default discipline
- ✅ .specify/templates/commands/tasks.md updated for verification gates and no MVP scope language
- ✅ .specify/scripts/{bash,powershell}/ restored from the installed Spec Kit core pack
Runtime guidance reviewed:
- ✅ README.md
- ✅ freezeitApp/README.md
- ✅ freezeitRelease/README.md
- ✅ freezeitVS/README.md
Follow-up TODOs: None
-->

# Freezeit Constitution

## Core Principles

### I. Simplicity and Necessity
The project MUST choose the simplest design that satisfies the validated need.
New entities, services, abstractions, files, dependencies, configuration keys, and
interfaces MUST NOT be added unless a concrete requirement or verified constraint
requires them. Any complexity that remains MUST be justified in the plan.

Rationale: simple systems are easier to verify, recover, and maintain on a
single-device Android/module target.

### II. Verified Interfaces and Confirmed Requirements
Contributors MUST query existing source, documentation, SDK/API references, CLI
help, runtime behavior, or other authoritative tools before relying on an
interface. Library, framework, SDK, API, CLI, or cloud-service questions MUST use
Context7 (`ctx7`) when current documentation is needed. Business intent and
user-specific scope MUST be confirmed by a human when local evidence is
insufficient.

Rationale: guessing interfaces, business rules, or target behavior creates
fragile work and false confidence.

### III. Reuse Existing Architecture, Skills, MCP, and Tools
Contributors MUST reuse existing project interfaces, modules, conventions,
skills, MCP tools, and local workflows before creating new ones. Code discovery
MUST prefer the project knowledge graph MCP where available; direct text search
is reserved for string literals, non-code files, configuration, or MCP
insufficient results. Architecture changes MUST preserve documented boundaries
unless an approved plan records why the boundary must change.

Rationale: this repository combines inherited Android app/module code with
project-specific Spec Kit workflow; new paths are higher risk than verified
local paths.

### IV. Active Verification Before Completion
Every change MUST define and run appropriate verification before completion is
claimed. Verification MAY be automated tests, build checks, static analysis,
manual device validation, artifact inspection, or a documented combination, but
it MUST produce concrete evidence. Before any task is reported complete,
`/brooks-review` and `/speckit-converge` MUST be run; any failing or unresolved
finding blocks completion until fixed or explicitly accepted by the human owner.

Rationale: skipped validation is indistinguishable from untested failure on a
ROM-specific Android module.

### V. Complete and Honest Delivery
Contributors MUST NOT present MVPs, placeholders, samples, mock behavior,
unsupported assumptions, or partial work as finished. Unknowns MUST be stated
plainly, with the next verification or human-confirmation step recorded.
Refactoring MUST be cautious, scoped, and justified by the current task.

Rationale: self-use system-modification work needs accurate status more than
optimistic delivery language.

## Eight Honors and Eight Shames

- Interface truth: be ashamed of guessing interfaces; be honored by serious
  lookup and verification.
- Execution clarity: be ashamed of vague execution; be honored by seeking
  confirmation when the next action is ambiguous.
- Business humility: be ashamed of inventing business intent; be honored by
  human confirmation of user-specific behavior.
- Interface reuse: be ashamed of creating new interfaces without need; be
  honored by reusing existing contracts and local workflows.
- Verification discipline: be ashamed of skipping verification; be honored by
  proactive testing and evidence collection.
- Architecture respect: be ashamed of breaking architecture; be honored by
  following project rules and documenting justified exceptions.
- Honest uncertainty: be ashamed of pretending to understand; be honored by
  stating ignorance and narrowing it with evidence.
- Cautious change: be ashamed of blind modification; be honored by careful,
  scoped refactoring.

## Development Workflow and Quality Gates

1. Discovery MUST begin with existing project context: specifications, README
   files, local workflows, source graph MCP results, and authoritative docs.
2. Scope MUST stay bounded to the validated user request. Additional targets,
   entities, dependencies, or public-release claims require explicit human
   confirmation.
3. Plans MUST record how each constitution principle is satisfied. Any
   exception MUST appear in complexity tracking with a simpler alternative that
   was rejected for a concrete reason.
4. Tasks MUST include verification work for every user-facing or system-facing
   behavior. If automation is not practical, manual validation steps and
   expected evidence are required.
5. Completion claims MUST include the verification performed and the results of
   `/brooks-review` and `/speckit-converge`. Open failures mean the task is not
   complete.

## Governance

This constitution supersedes conflicting local habits, generated templates, and
ad hoc instructions for feature work in this repository. Amendments require a
documented rationale, an explicit semantic-version bump, updates to dependent
Spec Kit templates or command guidance, and a Sync Impact Report in this file.

Versioning policy:
- MAJOR: principle removals, incompatible governance changes, or relaxed quality
  gates.
- MINOR: new principles, new required sections, or materially expanded
  governance.
- PATCH: wording clarifications, typo fixes, or non-semantic refinements.

Compliance review is mandatory during specification, planning, task generation,
implementation review, and convergence. Reviewers MUST treat unresolved
constitution violations as blockers unless the human owner explicitly accepts
the risk in writing.

**Version**: 1.0.0 | **Ratified**: 2026-07-03 | **Last Amended**: 2026-07-03
