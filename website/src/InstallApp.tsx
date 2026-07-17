import { useEffect } from "react";
import { ArrowLeft, ArrowRight, Layers, Package, Terminal, Wrench } from "lucide-react";
import "./index.css";

export function InstallApp() {
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
          <a href="/" className="reveal flex items-center gap-3" data-delay="1">
            <div className="grid h-11 w-11 place-items-center rounded-2xl bg-primary text-primary-foreground shadow-[0_12px_30px_-16px_rgba(16,24,40,0.6)]">
              <Layers className="h-5 w-5" aria-hidden="true" />
            </div>
            <div>
              <p className="font-display text-lg tracking-tight">Harbor</p>
              <p className="text-[0.65rem] uppercase tracking-[0.32em] text-muted-foreground">Install</p>
            </div>
          </a>
          <a
            href="/"
            className="hidden items-center gap-2 rounded-full border border-foreground/15 bg-card/80 px-4 py-2 text-xs font-semibold uppercase tracking-[0.3em] text-foreground/80 shadow-sm md:inline-flex reveal"
            data-delay="2"
          >
            <ArrowLeft className="h-4 w-4" aria-hidden="true" />
            Back to home
          </a>
        </header>

        <main className="mx-auto flex max-w-6xl flex-col gap-16 px-6 pb-24 pt-12">
          <section className="grid items-center gap-10 lg:grid-cols-[1.1fr_0.9fr]" data-reveal>
            <div className="reveal space-y-6">
              <div className="float-slow inline-flex items-center gap-2 rounded-full border border-foreground/10 bg-card/80 px-4 py-2 text-xs font-semibold uppercase tracking-[0.3em] text-foreground/70">
                All install methods
              </div>
              <h1 className="font-display text-4xl leading-tight tracking-tight sm:text-5xl lg:text-6xl">
                Install Harbor locally, the way your team prefers.
              </h1>
              <p className="text-base text-muted-foreground sm:text-lg">
                Pick Cargo for a quick install or build manually with the Rust toolchain. The Nix flake input lives on
                the main landing page.
              </p>
              <div className="flex flex-wrap gap-3">
                <a
                  href="/"
                  className="inline-flex items-center gap-2 rounded-full border border-foreground/15 bg-background px-4 py-2 text-xs font-semibold uppercase tracking-[0.3em] text-foreground/80"
                >
                  View flake input
                  <ArrowRight className="h-4 w-4" aria-hidden="true" />
                </a>
              </div>
            </div>

            <div className="reveal rounded-3xl border border-foreground/10 bg-card/80 p-6 shadow-sm" data-delay="2">
              <p className="text-xs uppercase tracking-[0.3em] text-muted-foreground">Install checklist</p>
              <ul className="mt-5 space-y-4 text-sm">
                <li className="flex items-start gap-3">
                  <Terminal className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                  <div>
                    <p className="font-semibold">Rust toolchain</p>
                    <p className="text-muted-foreground">Ensure Rust is installed before running commands.</p>
                  </div>
                </li>
                <li className="flex items-start gap-3">
                  <Package className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                  <div>
                    <p className="font-semibold">Cargo install</p>
                    <p className="text-muted-foreground">Grab the CLI directly from crates.io.</p>
                  </div>
                </li>
                <li className="flex items-start gap-3">
                  <Wrench className="mt-1 h-4 w-4 text-primary" aria-hidden="true" />
                  <div>
                    <p className="font-semibold">Manual build</p>
                    <p className="text-muted-foreground">Build from source with one command.</p>
                  </div>
                </li>
              </ul>
            </div>
          </section>

          <section className="rounded-2xl border border-foreground/10 bg-card/80 p-6 shadow-sm reveal" data-reveal>
            <div className="flex items-center gap-3">
              <Layers className="h-5 w-5 text-primary" aria-hidden="true" />
              <h2 className="font-display text-2xl">Nix flake input</h2>
            </div>
            <p className="mt-3 text-sm text-muted-foreground">Add Harbor as a flake input in your repo.</p>
            <pre className="mt-5 rounded-xl bg-foreground/5 p-4 text-xs text-foreground/80">
              <code>{`inputs = {
  harbor = {
    url = "git+https://gitea.maariz.org/pure_sagacity/harbor.git";
    inputs.nixpkgs.follows = "nixpkgs";
  };
};`}</code>
            </pre>
          </section>

          <section className="grid gap-6 md:grid-cols-2" data-reveal>
            <div className="reveal rounded-2xl border border-foreground/10 bg-card/80 p-6 shadow-sm" data-delay="1">
              <div className="flex items-center gap-3">
                <Package className="h-5 w-5 text-primary" aria-hidden="true" />
                <h2 className="font-display text-2xl">Cargo install</h2>
              </div>
              <p className="mt-3 text-sm text-muted-foreground">Install from crates.io.</p>
              <pre className="mt-5 rounded-xl bg-foreground/5 p-4 text-xs text-foreground/80">
                <code>cargo install harbor</code>
              </pre>
            </div>

            <div className="reveal rounded-2xl border border-foreground/10 bg-card/80 p-6 shadow-sm" data-delay="2">
              <div className="flex items-center gap-3">
                <Wrench className="h-5 w-5 text-primary" aria-hidden="true" />
                <h2 className="font-display text-2xl">Manual build</h2>
              </div>
              <p className="mt-3 text-sm text-muted-foreground">Build the CLI directly from the repo.</p>
              <pre className="mt-5 rounded-xl bg-foreground/5 p-4 text-xs text-foreground/80">
                <code>cargo build -p cli --release</code>
              </pre>
            </div>
          </section>
        </main>

        <footer className="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-3 px-6 pb-10 text-xs text-muted-foreground">
          <span>Harbor install</span>
          <span>Choose Cargo or manual builds.</span>
        </footer>
      </div>
    </div>
  );
}

export default InstallApp;
