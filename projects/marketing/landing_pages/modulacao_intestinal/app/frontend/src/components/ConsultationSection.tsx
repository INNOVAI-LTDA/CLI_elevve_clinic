import { CTA_URL } from "@/lib/cta";
const ConsultationSection = () => (
  <section className="py-20 md:py-28 bg-olive-light">
    <div className="container mx-auto px-6 max-w-3xl text-center">
      <h2 className="font-heading text-3xl md:text-4xl text-foreground mb-6 text-balance">
        O primeiro passo é entender se, no seu caso, o intestino realmente está envolvido.
      </h2>
      <p className="font-body text-muted-foreground leading-relaxed mb-10 max-w-2xl mx-auto">
        Porque nem sempre ele é o único fator — mas, quando é, muda completamente a estratégia de tratamento. Por isso, tudo começa com uma avaliação aprofundada.
      </p>

      <a
        href={CTA_URL}
        target="_blank"
        rel="noopener noreferrer"
        className="inline-block bg-olive hover:bg-accent transition-colors px-8 py-4 rounded-lg font-body font-medium text-primary-foreground tracking-wide mb-16"
      >
        Quero entender meu caso de forma completa →
      </a>

      <div className="bg-background rounded-2xl p-8 md:p-10 shadow-[var(--shadow-card)] text-left">
        <h3 className="font-heading text-2xl text-foreground mb-6">Na consulta com a Dra. Cristal:</h3>
        <ul className="space-y-4 font-body text-muted-foreground">
          <li className="flex gap-3"><span className="text-olive font-medium">✦</span>Seu caso é analisado de forma detalhada</li>
          <li className="flex gap-3"><span className="text-olive font-medium">✦</span>Os sinais são cruzados de forma estratégica</li>
          <li className="flex gap-3"><span className="text-olive font-medium">✦</span>É definido se o intestino faz parte da causa</li>
        </ul>
        <p className="font-body text-foreground font-medium mt-6">
          O objetivo não é apenas entender — é sair com um direcionamento claro e individualizado.
        </p>
      </div>
    </div>
  </section>
);

export default ConsultationSection;
