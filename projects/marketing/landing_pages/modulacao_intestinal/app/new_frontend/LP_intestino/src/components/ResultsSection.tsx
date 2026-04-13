const results = [
  { emoji: "⚡", label: "Energia" },
  { emoji: "🏋️", label: "Composição corporal" },
  { emoji: "🧠", label: "Clareza mental" },
  { emoji: "💊", label: "Sintomas hormonais" },
  { emoji: "🛡️", label: "Imunidade" },
];

const ResultsSection = () => (
  <section className="py-20 md:py-28 bg-olive-light">
    <div className="container mx-auto px-6 max-w-4xl text-center">
      <h2 className="font-heading text-3xl md:text-4xl text-foreground mb-12 text-balance">
        Pacientes com esse padrão frequentemente apresentam melhora em:
      </h2>

      <div className="grid grid-cols-2 md:grid-cols-5 gap-4 mb-12">
        {results.map((r) => (
          <div
            key={r.label}
            className="bg-background rounded-2xl p-6 shadow-[var(--shadow-soft)] flex flex-col items-center gap-3"
          >
            <span className="text-3xl">{r.emoji}</span>
            <span className="font-body text-sm font-medium text-foreground">{r.label}</span>
          </div>
        ))}
      </div>

      <p className="font-heading text-2xl text-foreground italic">
        Quando a base é tratada.
      </p>
    </div>
  </section>
);

export default ResultsSection;
