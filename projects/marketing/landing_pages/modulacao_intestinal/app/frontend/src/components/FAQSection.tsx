import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { CTA_URL, handleCTAClick } from "@/lib/cta";

const FAQSection = () => {
  const faqs = [
    {
      question: "Quanto tempo dura o protocolo de restauração intestinal?",
      answer: "O protocolo varia conforme cada caso, mas geralmente dura entre 3 a 6 meses. A duração depende da gravidade da disbiose, do comprometimento do paciente e dos resultados das reavaliações laboratoriais."
    },
    {
      question: "Preciso fazer exames antes de começar?",
      answer: "Sim. Realizamos uma bateria completa de exames incluindo teste de microbiota intestinal (PCR), testes de permeabilidade intestinal, análise de ácidos orgânicos urinários e avaliação metabólica completa. Estes exames são fundamentais para personalizar seu protocolo."
    },
    {
      question: "O protocolo inclui restrições alimentares?",
      answer: "Sim, inicialmente podem ser necessárias algumas restrições alimentares temporárias para reduzir inflamação e permitir a recuperação intestinal. Porém, nosso objetivo é expandir sua dieta gradualmente, não restringir permanentemente."
    },
    {
      question: "Vocês atendem online ou apenas presencial?",
      answer: "Atendemos tanto presencialmente em nossa clínica quanto online para pacientes de outras cidades ou países. O acompanhamento remoto tem os mesmos protocolos e qualidade do atendimento presencial."
    },
    {
      question: "O protocolo funciona para quem já tentou outros tratamentos?",
      answer: "Sim! Nosso diferencial está justamente na abordagem investigativa profunda. Muitos pacientes chegam até nós após anos tentando diferentes abordagens sem sucesso. Nossa metodologia identifica as causas raiz que podem ter sido negligenciadas."
    },
    {
      question: "Qual o investimento do protocolo?",
      answer: "O valor varia conforme a complexidade do caso e os exames necessários. Agende uma avaliação inicial para receber um orçamento personalizado. Trabalhamos com diferentes formas de pagamento para facilitar seu acesso ao tratamento."
    },
    {
      question: "Há garantia de resultados?",
      answer: "Não prometemos resultados mágicos ou imediatos. O que oferecemos é uma metodologia comprovada, acompanhamento rigoroso e dedicação total para alcançar seus objetivos de saúde. Resultados dependem do comprometimento do paciente com o protocolo."
    }
  ];

  return (
    <section className="py-20 bg-gray-50" id="faq">
      <div className="container mx-auto px-4 max-w-4xl">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-serif font-bold text-gray-900 mb-4">
            Perguntas Frequentes
          </h2>
          <p className="text-gray-600 text-lg">
            Tire suas dúvidas sobre o Protocolo de Restauração Intestinal
          </p>
        </div>

        <Accordion type="single" collapsible className="w-full">
          {faqs.map((faq, index) => (
            <AccordionItem key={index} value={`item-${index}`}>
              <AccordionTrigger className="text-left">
                <span className="font-medium text-gray-900">{faq.question}</span>
              </AccordionTrigger>
              <AccordionContent className="text-gray-700 leading-relaxed">
                {faq.answer}
              </AccordionContent>
            </AccordionItem>
          ))}
        </Accordion>

        <div className="mt-12 text-center">
          <p className="text-gray-600 mb-6">
            Ainda tem dúvidas? Entre em contato conosco.
          </p>
          <a
            href={CTA_URL}
            onClick={(event) => handleCTAClick(event, "faq")}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 bg-green-600 hover:bg-green-700 text-white font-semibold py-3 px-8 rounded-full transition-all duration-300 transform hover:scale-105 shadow-lg"
          >
            Falar com especialista
          </a>
        </div>
      </div>
    </section>
  );
};

export default FAQSection;
