import draCristal from "@/assets/dra-cristal-2.jpeg";
import { buildTrackedCtaUrl } from "@/lib/cta";

const HeroSection = () => (
  <section className="relative min-h-screen flex items-center">
    <div className="absolute inset-0">
      <img
        src={draCristal}
        alt="Dra. Cristal"
        className="w-full h-full object-cover object-top -scale-x-100"
      />
      <div className="absolute inset-0 bg-gradient-to-r from-foreground/80 via-foreground/60 to-transparent" />
    </div>

    <div className="relative z-10 container mx-auto px-6 py-24 max-w-6xl">
      <div className="max-w-xl">
        <p className="font-body text-sm tracking-[0.2em] uppercase text-warm mb-6 opacity-80">
          Elevve Clinic · Dra. Cristal
        </p>
        <h1 className="font-heading text-4xl md:text-5xl lg:text-6xl leading-tight text-primary-foreground mb-6 text-balance">
          Ir ao banheiro todos os dias não significa que seu intestino está saudável.
        </h1>
        <p className="font-body text-lg text-warm/90 leading-relaxed mb-4">
          Se você convive com cansaço constante, dificuldade para emagrecer, alterações hormonais, baixa imunidade ou exames "normais" sem explicação…
        </p>
        <p className="font-body text-lg text-warm leading-relaxed mb-10 font-medium">
          é possível que o seu intestino esteja desregulado — mesmo funcionando todos os dias.
        </p>
        <a
          href={buildTrackedCtaUrl("HeroSection")}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-block bg-olive hover:bg-accent transition-colors px-8 py-4 rounded-lg font-body font-medium text-primary-foreground tracking-wide"
        >
          Quero investigar o que está por trás do meu problema →
        </a>
      </div>
    </div>
  </section>
);

export default HeroSection;
