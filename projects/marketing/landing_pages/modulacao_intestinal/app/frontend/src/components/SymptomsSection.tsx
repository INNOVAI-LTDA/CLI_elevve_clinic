const SymptomsSection = () => (
  <section className="py-20 md:py-28 bg-background">
    <div className="container mx-auto px-6 max-w-4xl">
      <h2 className="font-heading text-3xl md:text-4xl text-foreground text-center mb-16 text-balance">
        Você pode se identificar com isso de duas formas:
      </h2>

      <div className="grid md:grid-cols-2 gap-8 md:gap-12">
        <div className="bg-cream rounded-2xl p-8 shadow-[var(--shadow-soft)]">
          <h3 className="font-heading text-xl text-foreground mb-6">Sinais digestivos</h3>
          <ul className="space-y-3 font-body text-muted-foreground">
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Intestino preso ou irregular</li>
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Distensão abdominal</li>
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Gases frequentes ou fétidos</li>
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Desconforto abdominal</li>
          </ul>
        </div>

        <div className="bg-cream rounded-2xl p-8 shadow-[var(--shadow-soft)]">
          <h3 className="font-heading text-xl text-foreground mb-6">Sinais sistêmicos</h3>
          <ul className="space-y-3 font-body text-muted-foreground">
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Cansaço constante que não melhora</li>
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Dificuldade para emagrecer mesmo fazendo dieta</li>
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Névoa mental, lapsos de memória ou falta de foco</li>
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Alterações hormonais (estrogênio alto, endometriose, lipedema)</li>
            <li className="flex gap-3"><span className="text-olive mt-0.5">•</span>Exames "normais", mas você não se sente bem</li>
          </ul>
        </div>
      </div>

      <p className="font-heading text-xl text-center text-foreground mt-12 italic">
        E muitas vezes, esses dois perfis têm a mesma origem.
      </p>
    </div>
  </section>
);

export default SymptomsSection;
