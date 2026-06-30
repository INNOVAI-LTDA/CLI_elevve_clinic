# Prompt — Implementar endpoint/contrato do Radar

Crie ou ajuste endpoint para retornar Radar por paciente.

Endpoint sugerido:

```text
GET /api/bi/radar/patients/{patient_id}
```

Retorno mínimo:

```json
{
  "patient": {},
  "radar": {
    "priority_axis": "",
    "pillars": []
  },
  "meta": {
    "calculated_at": "",
    "pending_configuration": []
  }
}
```

Regras:

- Seguir padrão de schemas/DTOs existente.
- Não quebrar contratos v1 atuais.
- Se já existir contrato de Radar, adaptar sem duplicar desnecessariamente.
- Preservar error envelope.
