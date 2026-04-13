import draCristal from "@/assets/dra-cristal-1.jpeg";

const CTA_URL = "https://wa.me/5500000000000";

const DoctorSection = () => (
  <section className="py-20 md:py-28 bg-muted">
    <div className="container mx-auto px-6 max-w-4xl">
      <div className="grid md:grid-cols-[280px_1fr] gap-10 items-center">
        <div className="rounded-2xl overflow-hidden shadow-[var(--shadow-card)] mx-auto md:mx-0 max-w-[280px]">
          <img src={draCristal} alt="Dra. Cristal" className="w-full h-auto" />
        </div>
        <div>
          <h2 className="font-heading text-3xl text-foreground mb-2">Dra. Cristal</h2>
          <p className="font-body text-olive font-medium mb-4">Endocrinologista · Fundadora da Elevve Clinic</p>
          <p className="font-body text-muted-foreground leading-relaxed mb-8">
            Atuação focada na correção da base metabólica com acompanhamento contínuo.
          </p>
          <a
            href={CTA_URL}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-block bg-olive hover:bg-accent transition-colors px-8 py-4 rounded-lg font-body font-medium text-primary-foreground tracking-wide"
          >
            Quero entender o que realmente está causando meu problema →
          </a>
        </div>
      </div>
    </div>
  </section>
);

export default DoctorSection;
