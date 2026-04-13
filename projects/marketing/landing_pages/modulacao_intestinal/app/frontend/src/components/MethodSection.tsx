import heroMicrobiome from "@/assets/hero-microbiome.webp";

const MethodSection = () => (
  <section className="py-20 md:py-28 bg-background">
    <div className="container mx-auto px-6 max-w-5xl">
      <div className="grid md:grid-cols-2 gap-12 items-center">
        <div>
          <p className="font-body text-sm tracking-[0.15em] uppercase text-olive mb-4">Elevve Clinic</p>
          <h2 className="font-heading text-3xl md:text-4xl text-foreground mb-6 text-balance">
            Utilizamos um método estruturado que avalia os principais eixos que regulam o funcionamento do organismo.
          </h2>
          <p className="font-body text-muted-foreground leading-relaxed mb-8">
            O intestino é um deles — e, em muitos casos, é a base do problema. Esse método cruza:
          </p>
          <ul className="space-y-3 font-body text-foreground">
            <li className="flex gap-3"><span className="text-olive font-medium">→</span>Sintomas</li>
            <li className="flex gap-3"><span className="text-olive font-medium">→</span>Histórico clínico</li>
            <li className="flex gap-3"><span className="text-olive font-medium">→</span>Exames laboratoriais</li>
            <li className="flex gap-3"><span className="text-olive font-medium">→</span>Sinais inflamatórios</li>
            <li className="flex gap-3"><span className="text-olive font-medium">→</span>Funcionamento metabólico e hormonal</li>
          </ul>
          <p className="font-body text-foreground font-medium mt-6">
            Para entender com precisão o que está impedindo seu corpo de responder.
          </p>
        </div>
        <div className="rounded-2xl overflow-hidden shadow-[var(--shadow-card)]">
          <img
            src={heroMicrobiome}
            alt="Ilustração do sistema digestivo - Elevve Clinic"
            className="w-full h-auto"
          />
        </div>
      </div>
    </div>
  </section>
);

export default MethodSection;
