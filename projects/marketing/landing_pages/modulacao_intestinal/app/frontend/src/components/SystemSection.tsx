const SystemSection = () => (
  <section className="py-20 md:py-28 bg-muted">
    <div className="container mx-auto px-6 max-w-3xl">
      <h2 className="font-heading text-3xl md:text-4xl text-foreground mb-6 text-balance">
        Isso acontece porque o intestino não funciona isoladamente
      </h2>
      <p className="font-body text-muted-foreground mb-8 leading-relaxed">
        Ele influencia diretamente:
      </p>

      <div className="grid grid-cols-2 md:grid-cols-3 gap-4 mb-10">
        {["Metabolismo", "Regulação hormonal", "Inflamação sistêmica", "Absorção de vitaminas", "Nutrientes", "Imunidade"].map((item) => (
          <div key={item} className="bg-background rounded-xl p-5 text-center shadow-[var(--shadow-soft)]">
            <span className="font-body text-sm font-medium text-foreground">{item}</span>
          </div>
        ))}
      </div>

      <p className="font-body text-foreground font-medium text-lg">
        Quando esses fatores não são avaliados juntos, o tratamento fica superficial e o padrão se repete.
      </p>
    </div>
  </section>
);

export default SystemSection;
