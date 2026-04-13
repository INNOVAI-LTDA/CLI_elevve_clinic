const ExplanationSection = () => (
  <section className="py-20 md:py-28 bg-muted">
    <div className="container mx-auto px-6 max-w-3xl">
      <p className="font-body text-muted-foreground leading-relaxed mb-6 text-center">
        A maioria das pessoas só associa problema intestinal quando existe constipação, diarreia ou dor abdominal.
      </p>
      <p className="font-body text-foreground font-medium text-lg mb-12 text-center">
        Mas na prática, isso é só a parte visível do problema.
      </p>

      <div className="bg-background rounded-2xl p-8 md:p-10 shadow-[var(--shadow-card)] mb-10">
        <p className="font-heading text-xl text-foreground mb-6 italic text-center">
          É muito comum encontrar pacientes que evacuam todos os dias… e ainda assim apresentam sinais claros de desregulação intestinal.
        </p>
        <p className="font-body text-muted-foreground mb-4">Isso acontece porque:</p>
        <ul className="space-y-3 font-body text-muted-foreground">
          <li className="flex gap-3"><span className="text-olive">•</span>Evacuar diariamente não garante absorção adequada de nutrientes</li>
          <li className="flex gap-3"><span className="text-olive">•</span>Não significa equilíbrio da microbiota</li>
          <li className="flex gap-3"><span className="text-olive">•</span>Não exclui inflamação intestinal</li>
          <li className="flex gap-3"><span className="text-olive">•</span>Não exclui aumento de permeabilidade intestinal</li>
        </ul>
      </div>

      {/* Highlight box with left accent bar */}
      <div className="relative bg-olive/10 border-l-4 border-olive rounded-r-xl p-6 md:p-8 mb-10">
        <p className="font-heading text-xl md:text-2xl text-foreground leading-snug">
          O intestino pode estar eliminando — mas não absorvendo, regulando e protegendo como deveria.
        </p>
      </div>

      <p className="font-body text-muted-foreground mb-2">E quando isso passa despercebido:</p>
      <ul className="space-y-2 font-body text-muted-foreground ml-1">
        <li className="flex gap-3"><span className="text-olive">•</span>O paciente continua tratando sintomas isolados</li>
        <li className="flex gap-3"><span className="text-olive">•</span>Os exames parecem "normais"</li>
        <li className="flex gap-3"><span className="text-olive">•</span>E o corpo simplesmente não responde</li>
      </ul>
    </div>
  </section>
);

export default ExplanationSection;
