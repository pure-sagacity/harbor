import { useEffect } from "react";
import "./index.css";
import harborGlyph from "../../assets/icon-nobg.png";

const readinessChecklist = [
  "Enable flakes in nix.conf (experimental-features = nix-command flakes)",
  "Ensure access to gitea.maariz.org/pure_sagacity/harbor",
  "Optional: configure a Harbor Server endpoint for teams",
];

const secondaryInstalls = [
  {
    title: "Cargo install",
    description:
      "Install the Harbor CLI from crates.io when you want a pure Rust pathway without flakes.",
    code: "cargo install harbor",
    footnote: "Requires Rust stable 1.75+",
  },
  {
    title: "Manual build",
    description:
      "Compile the CLI yourself when you need to pin features or embed Harbor into bespoke workflows.",
    code: "cargo build -p cli --release",
    footnote: "Outputs binaries in target/release",
  },
];

const postInstallSteps = [
  {
    title: "harbor init",
    description:
      "Create a new secrets manifest connected to your repo and environments.",
  },
  {
    title: "harbor pull",
    description:
      "Sync encrypted secrets locally so your dev shells stay current.",
  },
  {
    title: "harbor push",
    description:
      "Review, approve, and ship updates to teammates or Harbor Server.",
  },
];

export function InstallApp() {
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
          <div className="absolute left-[8%] top-[-25%] h-64 w-64 rounded-full bg-primary/12 blur-[140px]" />
          <div className="absolute right-[12%] bottom-[-18%] h-60 w-60 rounded-full bg-sky-400/12 blur-[140px]" />
        </div>
        <main className="mx-auto flex max-w-6xl flex-col gap-14 px-6 pb-24 pt-12">
          <section
            className="reveal overflow-hidden rounded-3xl border border-white/6 bg-card/75 px-6 py-8 backdrop-blur-sm"
            data-reveal
          >
            <div className="flex flex-wrap items-center justify-between gap-6">
              <div className="space-y-4">
                <div className="flex items-center gap-3 text-xs uppercase tracking-[0.32em] text-muted-foreground">
                  <span className="inline-flex h-10 w-10 items-center justify-center rounded-full border border-white/12 bg-primary/10 p-2">
                    <img
                      src={harborGlyph}
                      alt="Harbor icon"
                      className="h-full w-full object-contain"
                    />
                  </span>
                  Install Harbor
                </div>
                <h1 className="font-display text-3xl leading-tight sm:text-4xl">
                  Secrets, synced securely.
                </h1>
                <p className="max-w-xl text-sm text-muted-foreground sm:text-base">
                  Bring Harbor into your stack with the Nix flake first, then
                  choose Cargo or manual builds when you need flexible entry
                  points.
                </p>
              </div>
              <a
                href="/"
                className="inline-flex items-center gap-2 rounded-full border border-white/20 px-5 py-2.5 text-xs font-semibold uppercase tracking-[0.35em] text-foreground/80 transition hover:border-white/40 hover:text-foreground"
              >
                ← Back to home
              </a>
            </div>
          </section>

          <section className="grid gap-8 lg:grid-cols-[1fr_0.85fr]">
            <div
              id="flake"
              className="reveal rounded-3xl border border-white/8 bg-background/70 p-6 backdrop-blur-sm"
              data-reveal
            >
              <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                <span className="h-[0.35rem] w-10 rounded-full bg-primary/50" />
                Nix flake install
              </div>
              <h2 className="font-display mt-5 text-2xl">
                Declarative sync in one move.
              </h2>
              <p className="mt-3 text-sm text-muted-foreground">
                Bring Harbor in as an input to keep the CLI and server API
                aligned across every developer machine.
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
                <span>
                  Pin the revision in your flake.lock for reproducibility.
                </span>
                <a
                  href="/install"
                  className="text-foreground transition hover:text-primary"
                >
                  View docs →
                </a>
              </div>
            </div>

            <div
              className="reveal rounded-3xl border border-white/8 bg-card/75 p-6 backdrop-blur-sm"
              data-reveal
            >
              <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                <span className="h-[0.35rem] w-10 rounded-full bg-primary/50" />
                Readiness checklist
              </div>
              <p className="mt-4 text-sm text-muted-foreground">
                Double-check these prerequisites before rolling Harbor onto your
                machines or CI runners.
              </p>
              <ul className="mt-6 space-y-3 text-sm text-muted-foreground/80">
                {readinessChecklist.map((item) => (
                  <li key={item} className="flex items-start gap-2">
                    <span className="mt-1 h-1.5 w-1.5 rounded-full bg-primary/60" />
                    <span>{item}</span>
                  </li>
                ))}
              </ul>
            </div>
          </section>

          <section className="reveal space-y-6" data-reveal>
            <div className="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
              <div>
                <span className="text-xs uppercase tracking-[0.35em] text-muted-foreground">
                  Alternative pathways
                </span>
                <h2 className="font-display mt-2 text-2xl">
                  Bring in Harbor your way.
                </h2>
              </div>
              <p className="max-w-lg text-sm text-muted-foreground">
                Prefer Cargo or bespoke builds? Keep the same Harbor commands
                and sync protocol whichever option you choose.
              </p>
            </div>
            <div className="grid gap-6 sm:grid-cols-2">
              {secondaryInstalls.map((option, index) => (
                <article
                  key={option.title}
                  className="reveal rounded-3xl border border-white/8 bg-card/75 p-6 backdrop-blur-sm transition hover:border-white/20"
                  data-reveal
                  data-delay={String(index + 1)}
                >
                  <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                    <span className="h-[0.35rem] w-10 rounded-full bg-primary/50" />
                    {option.title}
                  </div>
                  <p className="mt-4 text-sm text-muted-foreground">
                    {option.description}
                  </p>
                  <pre className="mt-5 rounded-2xl border border-white/5 bg-black/50 p-4 text-xs text-sky-100/90">
                    <code>{option.code}</code>
                  </pre>
                  <span className="mt-4 block text-xs uppercase tracking-[0.3em] text-muted-foreground">
                    {option.footnote}
                  </span>
                </article>
              ))}
            </div>
          </section>

          <section
            className="reveal rounded-3xl border border-white/8 bg-card/75 p-6 backdrop-blur-sm"
            data-reveal
          >
            <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
              <span className="h-[0.35rem] w-10 rounded-full bg-primary/50" />
              After install
            </div>
            <h2 className="font-display mt-5 text-2xl">
              Stay in lockstep with these commands.
            </h2>
            <div className="mt-6 grid gap-4 md:grid-cols-3">
              {postInstallSteps.map((step, index) => (
                <div
                  key={step.title}
                  className="reveal rounded-2xl border border-white/5 bg-white/5 p-5"
                  data-reveal
                  data-delay={String(index + 1)}
                >
                  <div className="flex items-start gap-3">
                    <span className="flex h-8 w-8 items-center justify-center rounded-full bg-primary/10 text-sm font-semibold text-primary">
                      {index + 1}
                    </span>
                    <div>
                      <h3 className="text-lg font-semibold text-foreground">
                        {step.title}
                      </h3>
                      <p className="mt-2 text-sm text-muted-foreground">
                        {step.description}
                      </p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </section>
        </main>

        <footer className="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-3 px-6 pb-10 text-xs text-muted-foreground">
          <span>Harbor install</span>
          <span>Flakes first. Cargo and manual builds when you need them.</span>
        </footer>
      </div>
    </div>
  );
}

export default InstallApp;
