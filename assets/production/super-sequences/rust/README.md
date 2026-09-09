# Ownership Eclipse — matéria do Sirius

`mutation-props.png`: atlas RGBA 1448×1086, gerado e corrigido com a ferramenta
imagegen integrada em 2026-09-09. Quatro objetos: anel científico, torre,
plataforma rachada e cabos. 50,27% dos pixels têm alpha zero. O renderer usa
cortes explícitos em `src/engine/render/authored_supers.rs`; não é uma grade regular.

Fonte final preservada:
`/home/willams/.codex/generated_images/01a08339-7fe6-7293-8023-8cb6ff3a3f76/exec-cc7078a7-5c33-4353-a94a-ce15806b9e06.png`.

Referências do próprio projeto: `assets/production/stage-life/arena-sirius-clean.png`
e `assets/production/rust/signature_special/source-spectacle-v1.png`. A referência
de pose definiu a paleta âmbar/metal, e a arena definiu o anel do Sirius.
[Prompts e tentativas](prompts.md). Gerações RGB com quadriculado pintado foram
rejeitadas; o asset final tem transparência verdadeira. A única operação após
a geração foi cópia do PNG. Análise de alpha foi somente leitura.

O anel emerge durante a construção, torres crescem, cabos acendem e placas
se rearranjam/derretem. Na restauração, as estruturas se dissolvem. Isso não
altera a colisão do cenário: o contato do pulso é comandado por `World`.
