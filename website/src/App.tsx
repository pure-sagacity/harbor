import { useEffect } from "react";
import "./index.css";
import harborGlyph from "../../assets/icon-nobg.png";

const featureHighlights = [
  {
    title: "Encrypted from commit to deploy",
    description:
      "Harbor encrypts secrets locally, moves ciphertext between peers, and keeps your pipelines sealed from source to runtime.",
    highlights: [
      "Hardware-backed sealing on every workstation",
      "Rotation reminders with typed audit entries",
      "Zero plaintext ever ships or rests at rest",
    ],
  },
  {
    title: "Project-aware sync lanes",
    description:
      "Model every environment once and let Harbor track which secrets belong to staging, preview, and production without the drift.",
    highlights: [
      "Selective sync per environment and repo",
      "Immediate conflict detection before merge",
      "Instant rollbacks to any earlier version",
    ],
  },
  {
    title: "Collaborate without drift",
    description:
      "Pair local iteration with server oversight so fast-moving teams stay aligned while shipping new features constantly.",
    highlights: [
      "Role-based scopes for every secret",
      "Signals when changes land for review",
      "Traceable history for compliance moments",
    ],
  },
];

const operatingModes = [
  {
    title: "Local-first workflows",
    badge: "Local",
    description:
      "Ship from the CLI, keep your repo the source of truth, and sync updates peer-to-peer when you are ready.",
    points: [
      "Works offline with cached ciphertext",
      "Git-friendly diffs that prevent merge pain",
      "Secrets injected into dev shells instantly",
    ],
  },
  {
    title: "Teamwide governance",
    badge: "Server",
    description:
      "Turn on Harbor Server when scale demands oversight — commands stay the same, observability levels up.",
    points: [
      "Policy templates for staging and production",
      "Expiration and rotation alerts on schedule",
      "Exportable compliance logs every time",
    ],
  },
];

const workflow = [
  {
    title: "Add Harbor as a flake input",
    description:
      "Pull in the CLI and API through Nix so every developer installs in one declarative move.",
  },
  {
    title: "Map environments to projects",
    description:
      "Describe staging, preview, and production once — Harbor stores encrypted specs and keeps them current.",
  },
  {
    title: "Sync and review with confidence",
    description:
      "Push updates to teammates or Harbor Server, review diffs, and land every change with full traceability.",
  },
];

export function App() {
  useEffect(() => {
    const elements = Array.from(
      document.querySelectorAll<HTMLElement>(".reveal"),
    );
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add("is-visible");
            observer.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.2 },
    );

    elements.forEach((element) => observer.observe(element));

    return () => observer.disconnect();
  }, []);

  return (
    <div className="min-h-screen bg-background text-foreground">
      <div className="relative">
        <div className="pointer-events-none absolute inset-0 -z-10 overflow-hidden">
          <div className="absolute left-[12%] top-[-25%] h-72 w-72 rounded-full bg-primary/12 blur-[140px]" />
          <div className="absolute right-[10%] bottom-[-20%] h-64 w-64 rounded-full bg-sky-400/12 blur-[140px]" />
        </div>
        <main className="mx-auto flex max-w-6xl flex-col gap-16 px-6 pb-24 pt-12">
          <section
            className="reveal overflow-hidden rounded-3xl border border-white/6 bg-card/75 px-8 py-10 backdrop-blur-sm"
            data-reveal
          >
            <div className="grid gap-12 lg:grid-cols-[1.05fr_0.95fr] lg:items-center">
              <div className="space-y-6">
                <div className="flex items-center gap-3 text-xs uppercase tracking-[0.32em] text-muted-foreground">
                  <span className="inline-flex h-10 w-10 items-center justify-center rounded-full border border-white/12 bg-primary/10 p-2">
                    <img
                      src={harborGlyph}
                      alt="Harbor icon"
                      className="h-full w-full object-contain"
                    />
                  </span>
                  Harbor platform
                </div>
                <h1 className="font-display text-4xl leading-tight sm:text-5xl">
                  Secrets, synced securely.
                </h1>
                <p className="max-w-xl text-base text-muted-foreground sm:text-lg">
                  Keep every environment secret in lockstep. Harbor brings
                  local-first workflows and teamwide control together with
                  encrypted sync.
                </p>
                <div className="flex items-center gap-3 text-sm text-muted-foreground">
                  <span className="h-px w-10 bg-primary/50" />
                  <span>
                    Local-first sync that harmonizes with your server when you
                    are ready.
                  </span>
                </div>
              </div>
              <div
                id="install"
                className="rounded-3xl border border-white/8 bg-background/70 p-6 backdrop-blur-sm"
              >
                <div className="flex items-center justify-between gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                  <span className="text-foreground/80">Nix flake install</span>
                  <span className="rounded-full bg-white/10 px-3 py-1 text-[0.6rem] text-foreground/60">
                    Primary
                  </span>
                </div>
                <p className="mt-4 text-sm text-muted-foreground">
                  Add Harbor as a flake input to sync secrets without drift.
                </p>
                <pre className="mt-6 rounded-2xl border border-white/5 bg-black/50 p-5 text-xs text-sky-100/90">
                  <code>{`inputs = {
  harbor = {
    url = "git+https://gitea.maariz.org/pure_sagacity/harbor.git";
    inputs.nixpkgs.follows = "nixpkgs";
  };
};`}</code>
                </pre>
                <div className="mt-5 flex flex-wrap items-center justify-between gap-3 text-xs text-muted-foreground">
                  <span>Works with any flake-enabled project.</span>
                  <a
                    href="/install"
                    className="text-foreground transition hover:text-primary"
                  >
                    Explore other paths →
                  </a>
                </div>
              </div>
            </div>
          </section>

          <section className="space-y-10">
            <div className="reveal" data-reveal>
              <span className="text-xs uppercase tracking-[0.35em] text-muted-foreground">
                Why Harbor
              </span>
              <div className="mt-3 flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
                <h2 className="font-display text-3xl sm:text-4xl">
                  Operational calm for every secret.
                </h2>
                <p className="max-w-xl text-sm text-muted-foreground sm:text-base">
                  Harbor keeps configuration drift-free with encrypted sync,
                  versioned change history, and awareness of the environments
                  you deploy to.
                </p>
              </div>
            </div>
            <div className="mt-4 grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
              {featureHighlights.map((feature, index) => {
                const delay = (index % 3) + 1;
                return (
                  <article
                    key={feature.title}
                    className="reveal rounded-3xl border border-white/8 bg-card/70 p-6 backdrop-blur-sm transition hover:border-white/20"
                    data-reveal
                    data-delay={String(delay)}
                  >
                    <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                      <span className="h-[0.35rem] w-10 rounded-full bg-primary/50" />
                      Focus
                    </div>
                    <h3 className="font-display mt-6 text-xl">
                      {feature.title}
                    </h3>
                    <p className="mt-4 text-sm text-muted-foreground">
                      {feature.description}
                    </p>
                    <ul className="mt-5 space-y-2 text-sm text-muted-foreground/80">
                      {feature.highlights.map((highlight) => (
                        <li key={highlight} className="flex items-center gap-2">
                          <span className="h-1.5 w-1.5 rounded-full bg-primary/60" />
                          {highlight}
                        </li>
                      ))}
                    </ul>
                  </article>
                );
              })}
            </div>
          </section>

          <section className="grid gap-8 lg:grid-cols-[0.85fr_1.15fr]">
            <div
              className="reveal rounded-3xl border border-white/8 bg-card/75 p-6 backdrop-blur-sm"
              data-reveal
            >
              <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                <span className="h-[0.35rem] w-10 rounded-full bg-primary/50" />
                Sync modes
              </div>
              <h3 className="font-display mt-5 text-2xl">
                Tailored for solo builders and fast-moving teams.
              </h3>
              <p className="mt-4 text-sm text-muted-foreground">
                Start local-only and graduate to the Harbor server when
                collaboration demands. Your workflows stay consistent whichever
                mode you choose.
              </p>
              <div className="mt-6 space-y-5">
                {operatingModes.map((mode) => (
                  <div
                    key={mode.title}
                    className="rounded-2xl border border-white/5 bg-white/5 p-5"
                  >
                    <div className="flex items-center justify-between gap-3">
                      <h4 className="text-lg font-semibold text-foreground">
                        {mode.title}
                      </h4>
                      <span className="text-xs uppercase tracking-[0.3em] text-muted-foreground">
                        {mode.badge}
                      </span>
                    </div>
                    <p className="mt-3 text-sm text-muted-foreground">
                      {mode.description}
                    </p>
                    <ul className="mt-4 space-y-2 text-sm text-muted-foreground/80">
                      {mode.points.map((point) => (
                        <li key={point} className="flex items-center gap-2">
                          <span className="h-1.5 w-1.5 rounded-full bg-primary/60" />
                          {point}
                        </li>
                      ))}
                    </ul>
                  </div>
                ))}
              </div>
            </div>
            <div
              className="reveal rounded-3xl border border-white/8 bg-card/75 p-6 backdrop-blur-sm"
              data-reveal
            >
              <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                <span className="h-[0.35rem] w-10 rounded-full bg-primary/50" />
                Sync timeline
              </div>
              <h3 className="font-display mt-5 text-2xl">
                A predictable path to parity.
              </h3>
              <div className="mt-6 space-y-5">
                {workflow.map((item, index) => (
                  <div
                    key={item.title}
                    className="reveal rounded-2xl border border-white/5 bg-white/5 p-5"
                    data-reveal
                    data-delay={String(index + 1)}
                  >
                    <div className="flex items-start gap-4">
                      <span className="flex h-8 w-8 items-center justify-center rounded-full bg-primary/10 text-sm font-semibold text-primary">
                        {index + 1}
                      </span>
                      <div>
                        <h4 className="text-lg font-semibold text-foreground">
                          {item.title}
                        </h4>
                        <p className="mt-2 text-sm text-muted-foreground">
                          {item.description}
                        </p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </section>
        </main>

        <footer className="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-3 px-6 pb-10 text-xs text-muted-foreground">
          <span>Harbor • Local-first secrets orchestration</span>
          <span>Encrypted sync, human workflows, one source of truth.</span>
        </footer>
      </div>
    </div>
  );
}

export default App;
