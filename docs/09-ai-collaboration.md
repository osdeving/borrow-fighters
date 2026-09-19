# 09 — Workflow de desenvolvimento

O fluxo normal usa contexto e verificações proporcionais à mudança.
[AGENTS.md](../AGENTS.md) é a regra comum para Codex e Claude. Skills são rotas
curtas, carregadas por necessidade. CI e release conservam a validação ampla.

## Uma entrada operacional

Execute `./dev --help`. Argumentos seguem para a ferramenta correspondente;
nenhum comando instala dependências automaticamente.

| Comando | Uso |
| --- | --- |
| `./dev story --start augusta` | Capítulo 2D da Augusta |
| `./dev game` | Versus |
| `./dev lab` | Laboratório de atores da aventura |
| `./dev studio` | Sprite Studio, editor de manifestos de luta |
| `./dev check` | Compilação rápida, sem executar testes |
| `./dev check fast adventure` | Compilação rápida do domínio |
| `./dev test adventure filtro` | Teste diretamente afetado |
| `./dev check adventure` | Fmt, Clippy e testes desse domínio |
| `./dev check fighting` | Fmt, Clippy e testes desse domínio |
| `./dev check docs arquivo.md` | Links/YAML dos arquivos indicados |
| `./dev check full` | Matriz Rust, contratos, docs e testes de ferramentas |
| `./dev art --help` | Builder: receitas, validação e publicação |
| `./dev schema check` | Conferir derivados do contrato Rust |
| `./dev schema build` | Atualizar schema/tipos derivados |

Duração depende de cache/máquina. `full` não publica nem cria pacotes;
[release](06-release-process.md) acrescenta stage/verify, smoke e distribuição.

## Trabalho necessário

1. Localize problema, arquivos e invariantes; leia somente a seção útil.
2. Altere o menor conjunto e rode a checagem afetada durante a iteração.
3. Integre e confira o domínio uma vez. Arte pede revisão curta em movimento;
   não filme o capítulo inteiro para cada ajuste de sprite.
4. Entregue comportamento, resultados dos checks e limitações concretas.

Não criar ADR, relatório, suíte ou ferramenta para toda tarefa. ADRs ficam para
decisões duradouras difíceis de desfazer. Docs mudam quando alguém precisa
conhecer novo comportamento/comando/contrato. Goals interrompíveis mantêm um
único diário curto, sem transcrever logs/conversas.

## Arte: fonte e alterações localizadas

- Autoria: referência canônica, imagens e receita. Novos conjuntos usam
  `assets-src/`; fontes legadas permanecem até uma migração ser necessária.
- Build: `./dev art` produz intermediários em `assets-generated/`.
- Runtime: somente a seleção aceita é publicada em `assets/`. Rebuild não
  substitui silenciosamente arte aceita pelo jogo.
- Sprite Studio calibra manifestos de luta; Actor Lab revisa atores da aventura.
  O antigo Sprite Viewer Raylib foi retirado.
- Preserve timing físico, hitboxes e regras ao trocar aparência. Corrija a
  fonte/receita afetada, evitando scripts por personagem e remendos anatômicos
  no jogo. Confira poucas poses no tamanho real antes de ampliar o lote.

[Builder](../tools/art/README.md) · [Sprite Studio e contrato](18-sprite-studio.md).
