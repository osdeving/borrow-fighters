# Expansão cinematográfica e passagem do capítulo

Goal solicitado em 12/09/2026. Branch inicial: `main`; árvore limpa ao iniciar.
Commit da entrega solicitado antes de iniciar a rodada seguinte de correções. Publicação não solicitada. Preservar alterações durante retomada.

## Plano e critérios

1. EP: preservar tomada da pipa; ao despertar a ameaça, acompanhar descida do
   alto, devolver enquadramento jogável antes do contato, aterrissar com peso,
   poeira e reação da vizinhança no impacto. Pausa/skip/retry consistentes.
2. Rust: revisar apoios ao acordar e caminhada lateral. Compartilhar a animação
   de locomoção entre prólogo, gameplay e atuação do capítulo; preservar roupa,
   proporções e estilo. Evitar troca de recortes que muda o apoio dos pés.
3. Duke: chegada de limousine à torre espelhada na Paulista e reunião na
   cabeceira; identidade original de Duke e pintura coerente com Python/C++.
4. Old C: ambiente atual com setup rústico, livros K&R/CC, bagunça de trabalho,
   postura intelectual e abordagem raiz. Personagem e objetos independentes.
5. Capítulo: obstáculo com múltiplas peças destrutíveis e dano progressivo,
   socos e chute com contato real; abertura libera duas EPs mais resistentes.
6. Conteúdo: manifestos validados para imagens, instâncias, trajetórias e
   tempos; textos externos. Preferir ampliar peças/estados existentes sem ECS.

## Divisão de trabalho

- `ep_arrival`: domínio/câmera/impacto da primeira EP.
- `rust_gait`: locomoção compartilhada e apoios da manhã.
- `chapter_combat`: chute, destroços, encontro duplo e checkpoints.
- Principal: arte e cenas de Duke/Old C, contrato de conteúdo, integração,
  documentação e verificação nativa/fmt/Clippy/testes/fronteiras.

## Checkpoint inicial

- Lidos README, arquitetura, ADRs de isolamento/peças/capítulo e skills de
  atlas, Rust, gameplay, arte e imagegen.
- Identificado: apresentação atual usa painéis fixos C++/Python; Duke/Old C
  têm somente retratos. Rua/capítulo já têm catálogo e composição separados.
- Arte existente inspecionada: Python/C++ com pintura cinematográfica;
  Duke mascote preto/branco com nariz vermelho, Old C grisalho de jeans azul.

## Checkpoint de integração

- EP implementada com trajetória contínua, câmera retornando antes do chão,
  pose de apoio, compressão/poeira/tremor e dois WAVs originais. Pânico no
  impacto; captura X11 passou 11/11, incluindo pausa, input, derrota/retry/skip.
- Rust: oito PNGs de caminhada + quatro de chute, apoios estáveis e cursor por
  distância física; mesma animação em gameplay/caminhos/quarto. F5 transacional.
- Duke/Old C: duas tomadas cada, catálogo de peças, keyframes de câmera/atores,
  rótulos presos a objetos e textos externos. Arquitetura na ADR 0028.
- Seleção de arte corrigida pelo imagegen para alpha real e retirada de
  lettering incorporado. Apenas dados de composição foram ajustados para
  corrigir oclusão por mesas, mantendo personagens/objetos reutilizáveis.
- Trilha 64s, logo 57s. Limite de captura automática agora deriva das durações
  narrativas, evitando truncamento depois da expansão.
- Capítulo: nove caixas com HP/gravidade/dano local, chute V/RT, duas EPs com
  120/112 PV e tuning individual. Revisão completa terminou em 6575 ticks com
 14 contatos na carga. Correção posterior impede bypass por pulos repetidos
  e faz os corpos derrotados assentarem no chão.
- Packaging: 96 arquivos da aventura entre 285 dependências totais; os quatro
  novos JSONs, dois atlas de biografias, 12 poses e dois WAVs estão incluídos;
  fontes de produção ficam fora do pacote.

## Comandos e resultados

- `cargo fmt --all`: passou.
- `cargo clippy --all-targets --all-features -- -D warnings`: passou.
- `cargo test --all-targets --all-features`: 515 aprovados, 2 ignorados preexistentes.
- Aventura isolada 119; luta isolada 396 (+2 ignorados); core 3 aprovados.
- `python3.13 tools/check_domain_boundaries.py`: passou; 56 fixtures aprovadas.
- `python3.13 -m unittest discover -s tools/release -p test_package.py`: 20 passaram.
- Mixer de revisão: 48 testes passaram.
- Links Markdown: 412 referências locais sem erro no checkpoint.
- `git diff --check`: passou. Nenhum YAML alterado.
- Ambiente tem Python 3 padrão anterior ao 3.11; verificações de fronteira
  executadas com `python3.13` disponível.
- Falhas resolvidas: teste antigo limitado a 90 s após ampliar a abertura;
  referências de texto dinâmico validadas contra o catálogo efetivo `--texts`;
  glyph de bullet na instrução da EP substituído por barra vertical.

## Fechamento

- Após as correções finais de stride e colisão, aventura isolada: 119 testes
  passaram; Clippy estrito passou novamente. Suíte completa final: 515 testes
  aprovados, 2 ignorados preexistentes.
- Composição conferida no renderer nativo: Duke apoiado na cabeceira, Old C
  visível atrás da escrivaninha, computador e livros no tampo, K&R legível,
  limousine no nível da rua. Dados corrigidos sem regenerar atores ou cenas.
- Evidências e limites em `docs/evidence/cinematic-expansion/README.md`;
  video/capturas de EP, locomoção, biografias e capítulo.
- Arte final e prompts permanecem em `assets/adventure/locomotion/` e
  `assets/adventure/opening/scenes/`, com procedência própria.
- Alterações verificadas e reunidas em commit por solicitação do usuário, antes
  da rodada de corrida, mundo modular e revisão das biografias. PR, release e
  deploy não fazem parte deste checkpoint.

## Retomada

Inspecionar `git status --short`, `git diff --stat`, este diário e agentes.
Não regenerar assets existentes sem revisar o que já foi produzido.
