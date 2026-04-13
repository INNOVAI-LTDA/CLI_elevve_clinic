const pillars = [
  { icon: "🔍", title: "Investigação aprofundada", desc: "Avaliação detalhada para identificar as causas reais" },
  { icon: "⚡", title: "Suporte metabólico direcionado", desc: "Estratégias personalizadas para o seu metabolismo" },
  { icon: "🥗", title: "Estratégia alimentar específica", desc: "Plano alimentar individualizado e funcional" },
];

const TreatmentSection = () => (
  <section className="py-20 md:py-28 bg-background">
    <div className="container mx-auto px-6 max-w-4xl">
      <h2 className="font-heading text-3xl md:text-4xl text-foreground mb-4 text-center text-balance">
        Quando o intestino é identificado como fator principal, o tratamento inclui:
      </h2>
      <p className="font-body text-muted-foreground text-center mb-12 leading-relaxed">
        Tudo conduzido com acompanhamento ao longo do processo.
      </p>

      <div className="grid md:grid-cols-3 gap-6">
        {pillars.map((p) => (
          <div
            key={p.title}
            className="bg-card rounded-2xl p-8 shadow-[var(--shadow-card)] border border-border text-center flex flex-col items-center"
          >
            <span className="text-4xl mb-4">{p.icon}</span>
            <h3 className="font-heading text-lg text-foreground mb-2">{p.title}</h3>
            <p className="font-body text-sm text-muted-foreground leading-relaxed">{p.desc}</p>
          </div>
        ))}
      </div>
    </div>
  </section>
);

export default TreatmentSection;
