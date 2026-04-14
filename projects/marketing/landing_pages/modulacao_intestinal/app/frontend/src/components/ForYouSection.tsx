import { CTA_URL, handleCTAClick } from "@/lib/cta";
const forYou = [
  "Já tentou dieta, suplementação ou tratamentos sem resultado duradouro",
  "Sente que seu corpo não responde como deveria",
  "Tem sintomas que não se explicam totalmente pelos exames",
  "Já fez \"tudo certo\" e mesmo assim não evoluiu",
  "Tem dificuldade para emagrecer ou manter resultado",
  "Sente cansaço frequente ou falta de energia",
  "Percebe alterações hormonais ou inflamatórias",
  "Quer entender a causa do problema, e não apenas aliviar sintomas",
  "Está disposto(a) a seguir um processo estruturado",
];

const notForYou = [
  "Busca solução rápida ou imediata",
  "Quer apenas um protocolo pronto",
  "Não quer se aprofundar no próprio caso",
  "Não está disposto(a) a acompanhar o processo",
];

const ForYouSection = () => (
  <section className="py-20 md:py-28 bg-muted">
    <div className="container mx-auto px-6 max-w-4xl">
      <h2 className="font-heading text-3xl md:text-4xl text-foreground text-center mb-14 text-balance">
        Essa abordagem faz sentido para você?
      </h2>

      <div className="grid md:grid-cols-2 gap-8 mb-16">
        <div className="bg-background rounded-2xl p-8 shadow-[var(--shadow-soft)]">
          <h3 className="font-heading text-xl text-foreground mb-6">Faz sentido se você:</h3>
          <ul className="space-y-3 font-body text-muted-foreground text-sm">
            {forYou.map((item, i) => (
              <li key={i} className="flex gap-3">
                <span className="text-olive font-medium shrink-0">✔</span>
                {item}
              </li>
            ))}
          </ul>
        </div>

        <div className="bg-background rounded-2xl p-8 shadow-[var(--shadow-soft)]">
          <h3 className="font-heading text-xl text-foreground mb-6">Não é para quem:</h3>
          <ul className="space-y-3 font-body text-muted-foreground text-sm">
            {notForYou.map((item, i) => (
              <li key={i} className="flex gap-3">
                <span className="text-destructive shrink-0">✗</span>
                {item}
              </li>
            ))}
          </ul>
        </div>
      </div>

      <div className="text-center">
        <a
          href={CTA_URL}
          onClick={(event) => handleCTAClick(event, "for_you")}
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

export default ForYouSection;
