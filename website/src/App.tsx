import { useEffect } from "react";
import {
  ArrowDown,
  ArrowRight,
  Cloud,
  Key,
  Layers,
  Lock,
  Package,
  Server,
  Settings,
  Shield,
  Terminal,
} from "lucide-react";
import "./index.css";

export function App() {
  useEffect(() => {
    const elements = Array.from(document.querySelectorAll<HTMLElement>(".reveal"));
    const observer = new IntersectionObserver(
      entries => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            entry.target.classList.add("is-visible");
            observer.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.2 }
    );

    elements.forEach(element => observer.observe(element));

    return () => observer.disconnect();
  }, []);

  return (
    <div className="min-h-screen text-foreground">
      <div className="relative">
        <header className="mx-auto flex max-w-6xl items-center justify-between px-6 pt-8" data-reveal>
          <div className="reveal flex items-center gap-3" data-delay="1">
            <div className="grid h-11 w-11 place-items-center rounded-2xl bg-primary text-primary-foreground shadow-[0_12px_30px_-16px_rgba(16,24,40,0.6)]">
              <Layers className="h-5 w-5" aria-hidden="true" />
            </div>
            <div>
              <p className="font-display text-lg tracking-tight">Harbor</p>
              <p className="text-[0.65rem] uppercase tracking-[0.32em] text-muted-foreground">Local-first secrets</p>
            </div>
          </div>
          <nav className="hidden items-center gap-6 text-sm font-medium text-muted-foreground md:flex reveal" data-delay="2">
            <a href="#local" className="transition hover:text-foreground">
              Local
            </a>
            <a href="#server" className="transition hover:text-foreground">
              Server
            </a>
            <a href="#install" className="transition hover:text-foreground">
              Nix Flake
            </a>
            <a href="#capabilities" className="transition hover:text-foreground">
              Capabilities
            </a>
          </nav>
          <a
            href="/install"
            className="hidden rounded-full border border-foreground/15 bg-card/80 px-4 py-2 text-xs font-semibold uppercase tracking-[0.3em] text-foreground/80 shadow-sm md:inline-flex reveal"
            data-delay="3"
          >
            Install
          </a>
        </header>

        <main className="mx-auto flex max-w-6xl flex-col gap-20 px-6 pb-24 pt-12">
          <section className="grid items-center gap-10 lg:grid-cols-[1.1fr_0.9fr]">
            <div className="space-y-6" data-reveal>
              <div className="reveal float-slow inline-flex items-center gap-2 rounded-full border border-foreground/10 bg-card/80 px-4 py-2 text-xs font-semibold uppercase tracking-[0.3em] text-foreground/70">
                Local + server in sync
              </div>
              <h1 className="font-display reveal text-4xl leading-tight tracking-tight sm:text-5xl lg:text-6xl" data-delay="1">
                Harbor keeps local config and team secrets in lockstep.
              </h1>
              <p className="reveal text-base text-muted-foreground sm:text-lg" data-delay="2">
                Run the CLI locally to read configuration straight from your code. Sync only the projects you want, and
                let the server manage secrets, configs, and shared updates for the whole team.
              </p>
              <div className="reveal flex flex-wrap gap-3" data-delay="3">
                <a
                  href="/install"
                  className="rounded-full bg-primary px-5 py-2.5 text-sm font-semibold text-primary-foreground shadow-[0_12px_30px_-16px_rgba(16,24,40,0.6)] transition hover:translate-y-[-1px]"
                >
                  Install
                </a>
                <a
                  href="#capabilities"
                  className="rounded-full border border-foreground/15 bg-card/80 px-5 py-2.5 text-sm font-semibold text-foreground/80 shadow-sm transition hover:text-foreground"
                >
                  View capabilities
                </a>
              </div>
            </div>

            <div className="reveal rounded-3xl border border-foreground/10 bg-card/80 p-6 shadow-[0_40px_120px_-80px_rgba(16,24,40,0.6)]" data-reveal>
              <div className="flex items-center justify-between">
                <p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">Sync map</p>
                <span className="rounded-full bg-foreground/5 px-3 py-1 text-xs text-foreground/70">Team-wide</span>
              </div>
              <ol className="mt-6 space-y-5 text-sm">
                <li className="flex gap-3">
                  <Terminal className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                  <div>
                    <p className="font-semibold text-foreground">Local reads the code</p>
                    <p className="text-muted-foreground">The CLI inspects configuration directly in your repo.</p>
                  </div>
                </li>
                <li className="flex gap-3">
                  <Package className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                  <div>
                    <p className="font-semibold text-foreground">Selective project sync</p>
                    <p className="text-muted-foreground">Only the projects you choose are sent to the server.</p>
                  </div>
                </li>
                <li className="flex gap-3">
                  <Server className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                  <div>
                    <p className="font-semibold text-foreground">Server control plane</p>
                    <p className="text-muted-foreground">Manage secrets, configs, and shared updates.</p>
                  </div>
                </li>
                <li className="flex gap-3">
                  <Shield className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                  <div>
                    <p className="font-semibold text-foreground">Everyone stays aligned</p>
                    <p className="text-muted-foreground">Updates remain in sync across your entire team.</p>
                  </div>
                </li>
              </ol>
            </div>
          </section>

          <section id="local" className="scroll-mt-24" data-reveal>
            <div className="reveal grid gap-8 rounded-3xl border border-foreground/10 bg-card/80 p-8 shadow-sm lg:grid-cols-[1.1fr_0.9fr]">
              <div>
                <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                  <Terminal className="h-4 w-4 text-primary" aria-hidden="true" />
                  Local side
                </div>
                <h2 className="font-display mt-3 text-3xl">Local reads the code first.</h2>
                <p className="mt-3 text-sm text-muted-foreground">
                  Harbor runs next to your repo. Locally, you can just read the code, and the CLI does not require a
                  server to operate.
                </p>
                <div className="mt-6 space-y-4 text-sm">
                  <div className="flex items-start gap-3">
                    <Shield className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                    <div>
                      <p className="font-semibold">No server required</p>
                      <p className="text-muted-foreground">Everything runs locally until you choose to sync.</p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <Settings className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                    <div>
                      <p className="font-semibold">Understands your config</p>
                      <p className="text-muted-foreground">Reads configuration straight from your codebase.</p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <Package className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                    <div>
                      <p className="font-semibold">Selective project sync</p>
                      <p className="text-muted-foreground">Choose exactly which projects get shared.</p>
                    </div>
                  </div>
                </div>
                <a
                  href="#server"
                  className="mt-6 inline-flex items-center gap-2 rounded-full border border-foreground/15 bg-background px-4 py-2 text-xs font-semibold uppercase tracking-[0.3em] text-foreground/80"
                >
                  Switch to server
                  <ArrowDown className="h-4 w-4" aria-hidden="true" />
                </a>
              </div>

              <div className="flex flex-col justify-between gap-6 rounded-2xl border border-foreground/10 bg-background/60 p-5">
                <div>
                  <p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">Local workflow</p>
                  <h3 className="font-display mt-3 text-xl">CLI stays close to the repo.</h3>
                  <p className="mt-3 text-sm text-muted-foreground">
                    Read configuration where it lives, keep secrets local, and sync only when needed.
                  </p>
                </div>
                <div className="rounded-xl bg-foreground/5 p-4 text-xs text-foreground/70">
                  <p className="font-semibold text-foreground">Local-first cadence</p>
                  <p className="mt-2">Inspect config -> Select projects -> Sync to server</p>
                </div>
              </div>
            </div>
          </section>

          <section id="server" className="scroll-mt-24" data-reveal>
            <div className="reveal grid gap-8 rounded-3xl border border-foreground/10 bg-card/80 p-8 shadow-sm lg:grid-cols-[0.9fr_1.1fr]">
              <div className="order-2 lg:order-1 flex flex-col justify-between gap-6 rounded-2xl border border-foreground/10 bg-background/60 p-5">
                <div>
                  <p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">Server overview</p>
                  <h3 className="font-display mt-3 text-xl">Teamwide management hub.</h3>
                  <p className="mt-3 text-sm text-muted-foreground">
                    Use the local CLI to sync specific projects, then manage them on the server with full control.
                  </p>
                </div>
                <div className="rounded-xl bg-foreground/5 p-4 text-xs text-foreground/70">
                  <p className="font-semibold text-foreground">Server control</p>
                  <p className="mt-2">Edit secrets -> Create configs -> Sync to everyone</p>
                </div>
              </div>

              <div className="order-1 lg:order-2">
                <div className="flex items-center gap-3 text-xs uppercase tracking-[0.3em] text-muted-foreground">
                  <Server className="h-4 w-4 text-primary" aria-hidden="true" />
                  Server side
                </div>
                <h2 className="font-display mt-3 text-3xl">Manage secrets for the whole team.</h2>
                <p className="mt-3 text-sm text-muted-foreground">
                  Use the local CLI to sync selected projects. The server then manages those projects, including secrets
                  and configuration.
                </p>
                <div className="mt-6 space-y-4 text-sm">
                  <div className="flex items-start gap-3">
                    <Key className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                    <div>
                      <p className="font-semibold">Edit secrets and configs</p>
                      <p className="text-muted-foreground">Update values and create new configuration sets.</p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <Lock className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                    <div>
                      <p className="font-semibold">Generate new secrets</p>
                      <p className="text-muted-foreground">Create fresh values whenever you need them.</p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <Cloud className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                    <div>
                      <p className="font-semibold">Stay in sync</p>
                      <p className="text-muted-foreground">Every update stays aligned across your entire team.</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </section>

          <section id="install" className="scroll-mt-24" data-reveal>
            <div className="reveal flex flex-wrap items-end justify-between gap-4">
              <div>
                <p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">Install</p>
                <h2 className="font-display mt-3 text-3xl">Add Harbor as a flake input</h2>
              </div>
              <p className="max-w-md text-sm text-muted-foreground">
                Keep the flake input on your repo, then use the install page for Cargo and manual builds.
              </p>
            </div>

            <div className="reveal mt-8 rounded-2xl border border-foreground/10 bg-card/80 p-6 shadow-sm" data-delay="1">
              <div className="flex items-center gap-3">
                <Package className="h-5 w-5 text-primary" aria-hidden="true" />
                <h3 className="font-display text-xl">Nix flake input</h3>
              </div>
              <pre className="mt-4 rounded-xl bg-foreground/5 p-4 text-xs text-foreground/80">
                <code>{`inputs = {
  harbor = {
    url = "git+https://gitea.maariz.org/pure_sagacity/harbor.git";
    inputs.nixpkgs.follows = "nixpkgs";
  };
};`}</code>
              </pre>
              <div className="mt-5 flex flex-wrap items-center justify-between gap-4 text-sm">
                <span className="text-muted-foreground">Need Cargo or manual?</span>
                <a href="/install" className="inline-flex items-center gap-2 font-semibold text-primary">
                  View other install methods
                  <ArrowRight className="h-4 w-4" aria-hidden="true" />
                </a>
              </div>
            </div>
          </section>

          <section id="capabilities" className="scroll-mt-24" data-reveal>
            <div className="reveal flex flex-wrap items-end justify-between gap-4">
              <div>
                <p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">Capabilities</p>
                <h2 className="font-display mt-3 text-3xl">What Harbor can do</h2>
              </div>
              <p className="max-w-md text-sm text-muted-foreground">
                Local-first by design, with server-side controls that keep your team aligned.
              </p>
            </div>

            <div className="mt-8 grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
              {[
                {
                  title: "Read config from code",
                  description: "The CLI parses configuration directly from your repo.",
                  icon: Terminal,
                },
                {
                  title: "Project-level control",
                  description: "Sync only the projects you choose to share.",
                  icon: Package,
                },
                {
                  title: "Manage projects",
                  description: "Organize synced projects and environments.",
                  icon: Layers,
                },
                {
                  title: "Edit and update",
                  description: "Change secrets from the server and share updates.",
                  icon: Key,
                },
                {
                  title: "Create new configs",
                  description: "Build new configuration sets server-side.",
                  icon: Settings,
                },
                {
                  title: "Always aligned",
                  description: "Generate fresh secrets and keep every teammate in sync.",
                  icon: Shield,
                },
              ].map((item, index) => {
                const Icon = item.icon;
                return (
                  <div
                    key={item.title}
                    className="reveal rounded-2xl border border-foreground/10 bg-card/80 p-5 shadow-sm"
                    data-reveal
                    data-delay={index % 3 === 0 ? "1" : index % 3 === 1 ? "2" : "3"}
                  >
                    <div className="flex items-center gap-3">
                      <Icon className="h-5 w-5 text-primary" aria-hidden="true" />
                      <p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">Capability</p>
                    </div>
                    <h3 className="font-display mt-3 text-lg">{item.title}</h3>
                    <p className="mt-2 text-sm text-muted-foreground">{item.description}</p>
                  </div>
                );
              })}
            </div>
          </section>

          <section className="reveal rounded-3xl border border-foreground/10 bg-card/80 p-8 shadow-sm" data-reveal>
            <div className="max-w-2xl">
              <p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">Teamwide consistency</p>
              <h2 className="font-display mt-3 text-3xl">Every update stays in sync across your entire team.</h2>
              <p className="mt-3 text-sm text-muted-foreground">
                Sync a project once, then manage secrets and configs on the server. Harbor keeps local and server states
                aligned for everyone using the CLI.
              </p>
              <div className="mt-5 flex flex-wrap gap-3 text-xs font-semibold uppercase tracking-[0.24em] text-foreground/70">
                <span className="rounded-full border border-foreground/15 bg-background px-3 py-1">Local-first</span>
                <span className="rounded-full border border-foreground/15 bg-background px-3 py-1">Selective sync</span>
                <span className="rounded-full border border-foreground/15 bg-background px-3 py-1">Shared secrets</span>
              </div>
            </div>
          </section>
        </main>

        <footer className="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-3 px-6 pb-10 text-xs text-muted-foreground">
          <span>Harbor CLI + Server</span>
          <span>Local-first secrets and configuration sync.</span>
        </footer>
      </div>
    </div>
  );
}

export default App;
