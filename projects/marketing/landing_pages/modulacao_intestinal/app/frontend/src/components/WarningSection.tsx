import { AlertTriangle } from "lucide-react";

const WarningSection = () => {
  return (
    <section className="py-16 bg-amber-50/50">
      <div className="container mx-auto px-4 max-w-4xl">
        <div className="bg-white rounded-2xl shadow-lg p-8 border-l-4 border-amber-500">
          <div className="flex items-start gap-4">
            <AlertTriangle className="w-8 h-8 text-amber-500 flex-shrink-0 mt-1" />
            <div>
              <h2 className="text-2xl font-serif font-bold text-gray-900 mb-4">
                Atenção: Este protocolo não é para todos
              </h2>
              <div className="bg-amber-100 rounded-lg p-4">
                <p className="text-amber-900 font-medium">
                  Este protocolo requer acompanhamento médico especializado e exames laboratoriais completos. Não tente implementar estas estratégias por conta própria.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default WarningSection;
