import provaSocial from "@/assets/prova-social.jpeg";

const SocialProofSection = () => (
  <section className="py-20 md:py-28 bg-background">
    <div className="container mx-auto px-6 max-w-4xl">
      <h2 className="font-heading text-3xl md:text-4xl text-foreground text-center mb-6 text-balance">
        Quando tratamos a base, o resultado vai além do sintoma.
      </h2>
      <p className="font-body text-muted-foreground text-center max-w-2xl mx-auto mb-12 leading-relaxed">
        Pacientes que chegam há anos com intestino desregulado frequentemente descobrem que o problema não era apenas intestinal — e começam a responder de forma muito mais ampla.
      </p>

      <div className="flex justify-center mb-8">
        <div className="rounded-2xl overflow-hidden shadow-[var(--shadow-card)] max-w-sm border border-border">
          <img
            src={provaSocial}
            alt="Depoimento de paciente: Estou indo ao banheiro TODOS os dias!"
            className="w-full h-auto"
          />
        </div>
      </div>

      <p className="font-heading text-2xl text-center text-foreground italic">
        "Estou indo ao banheiro TODOS os dias!"
      </p>
    </div>
  </section>
);

export default SocialProofSection;
