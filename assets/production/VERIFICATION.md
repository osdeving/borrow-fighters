# Verificação dos candidatos de sprites

O piloto Rust foi validado tecnicamente antes da produção dos outros cinco personagens. A entrega contém **120 clips de personagem e 373 quadros selecionados**, preservando os contratos de combate existentes. Todos os conjuntos continuam candidatos à aprovação artística final.

| Personagem | Clips | Quadros | Laudo atual |
| --- | ---: | ---: | --- |
| Rust | 20 | 71 | [Revisão](rust/review.md) |
| Duke / Java | 20 | 61 | [Revisão](duke/review.md) |
| Go | 20 | 59 | [Verificação](go/VERIFICATION.md) |
| C | 20 | 61 | [Revisão](c/review.md) |
| Python | 20 | 60 | [Revisão](python/review.md) |
| C++ | 20 | 61 | [Revisão](cpp/review.md) |

Os laudos registram fontes geradas, identidade, alpha, recortes, pivôs, escalas, relógios dos golpes, exportação e prévias nos dois sentidos. O Combat Lab cobre os 19 contextos que oferece; caminhada, entrada/contagem e arco físico do salto foram examinados em cenários determinísticos do World nas duas orientações. Capturas antigas e recapturas com o manifesto de combate baseline têm alcances distintos, explicitados em cada laudo. Três partidas entre CPUs chegaram ao resultado no aplicativo nativo: Rust × Rust (9 × 0), Go × C (0 × 76) e Python × C++ (17 × 0), com [capturas preservadas](../candidates/runtime-matches/README.md). Isso não equivale a playtest manual de teclado nem à cobertura de todos os confrontos.

Notas como `pending`, `prepared` ou “aguardando validação” em prompts e metadados de geração registram o momento da preparação. Onde os laudos finais documentam a conclusão, essa evidência posterior define o estado técnico atual. Os arquivos de proveniência congelados foram preservados; essas notas históricas e a aprovação para exportar um candidato não são selos de aprovação artística. Limitações de movimento, contato e identidade continuam descritas nos laudos.

A [matriz de cobertura](../../docs/19-sprite-production-coverage.md) relaciona as ações ao código, distingue os projéteis separados e reúne a verificação global. Ela e os laudos acima são os pontos de consulta para o estado atual.
