# C++ / Augusta — diário da pipeline de produção

Branch: `main`. Base limpa: `c6a5c9d` (toda a etapa anterior já commitada).
Estado: implementação e validação concluídas; pipeline externa + capítulo C++/Julia.

## Autorização e recorte

Usuário pediu implementação completa, laboratório fora da campanha, formatos
reutilizáveis, carregamento contextual e capítulo integrado. C++ e Julia (22)
são adultas; arco paralelo a Rust/Python. Não há migração de engine nem do rig
de Rust. Decisão: [ADR 0032](../adr/0032-character-production-and-independent-chapters.md).

## Plano e responsáveis

1. Contratos/validação e simulação de personagens/golpes: `chapter_combat`.
2. Mundo, narrativa, checkpoints e entrada por protagonista: `ep_arrival`.
3. Escopo dos recursos do capítulo Rust, evidência antes/depois: `rust_gait`.
4. Referências Augusta, arte, rig/clips, renderer e laboratório compartilhados:
   agente principal. Integração, revisão visual e verificação final também raiz.

## Concluído

- Lidos README, arquitetura, ADR21, skills atlas/Rust/gameplay/arte/imagegen.
- Confirmada base limpa; goal criado sem orçamento de tokens.
- Identificado carregamento indevido de imagens de Ada/manhã/biografias no
  capítulo Rust e Sprite Studio da luta incompatível com o rig da aventura.
- Pesquisado roteiro oficial SPTuris do Baixo Augusta e referências fotográficas.
- Contratos iniciais e ownership distribuídos; ADR registrada antes de código.
- Pipeline implementada: `production/{spec,sim,animation}` e adaptadores comuns
  `engine/production`; binário `borrow-actor-lab` abre um personagem sem campanha.
- Vinte clips C++, rig com vistas pintadas, pernas de malha contínua, IK e curvas;
  importador de recortes/chroma com landmarks/hashes e pacotes NPC/EP independentes.
- Augusta tem mapa de 3600px, fachadas em módulos, roteiro externo e saves próprios;
  seletor Rust/C++ e CLI `--start augusta`. Combate com combo3, chute, giro, parry
  e projétil Linker. Atores com IDs estáveis; três seguranças e duas erráticas.
- Captura inicial real em `/tmp/cpp-chapter-draft1`: 14 fases até Complete,
  3+2 KOs por inputs normais, HP final 96; vídeo96,6s com mix de cues auditável.
  Carga contextual C++: 27 texturas, 32,39MiB RGBA base, só cinco atores do pacote.
- Carga Rust: 72→58 aberturas PNG; 4 contextos de capítulo e 5 poses de pesar
  idênticos antes/depois. Fixture sem 43 arquivos exclusivos do prólogo funciona.
- Laboratório inicial `/tmp/cpp-lab-draft`: 40 folhas, 1080 quadros únicos/18s60fps,
  captura portátil recarregável; detectados braço acima da cabeça e giro reverso.
  Corrigidos polo do cotovelo, yaw sem wrap indevido, registro da bolsa, mipmaps
  e alcance dos apoios. Na segunda revisão, uma troca de ramo do IK ainda fazia
  o cotovelo saltar; os alvos foram reautorados por arcos de articulações.
- Terceira revisão da corrida/giro aprovada visualmente e por amostragem densa:
  4096 posições, comprimentos, apoios, largura da malha e continuidade dos braços.
  A revisão posterior de todos os golpes identificou uma recuperação que passava
  de 330° a 0° pelo arco errado. O contrato agora escolhe `shortest` por padrão
  e `unwrapped` explicitamente para o giro; regressões cobrem as duas estratégias.
- Recarga F5 transacional no lab e no capítulo; fase da prévia preservada,
  cenário retomado pelo checkpoint. PNGs decodificados antes da GPU, referências
  e dimensões validadas. Snapshot exportado inclui pacote portátil executável.
- Fontes, rigs, clips, combate, mapa, textos e efeitos são arquivos próprios.
  Importações reproduzíveis sem regenerar pintura; runtime recebe apenas a
  cadeia de dependências dos capítulos. Perfil opcional do laboratório no pacote.
- Rust/aventura/luta passam pela verificação de isolamento; o novo binário tem
  classificação explícita de aventura e fixture que rejeita dependência da luta.
- Verificação final de código após correção de ângulo: 583 testes Rust passaram,
  0 falhas, 2 testes preexistentes ignorados; 107 testes Python passaram
  (15 importação, 2 mix de áudio, 33 pacote, 57 isolamento). A primeira chamada
  Python por nomes qualificados não encontrou três imports locais; foi repetida
  com `unittest discover -s` no diretório de cada suíte, sem alteração de código.
- `git diff --check` passou; 403 links Markdown locais verificados sem ausências
  no checkpoint anterior às evidências finais.

## Acabamento após a revisão integrada

- A captura completa também expôs a junta de iluminação do piso. A composição
  agora usa `mirror_ground_tiles` com ajuste de pivô; prova nativa curta em
  `/tmp/cpp-ground-seam-check` confirmou continuidade das bordas.
- No giro, a pintura do tronco incluía bases de coxa fixas. O crop retangular
  foi testado e rejeitado por criar uma barra horizontal. A faixa opcional
  `body_leg_blend` liga somente o tecido do quadril às coxas; cobertura gradual
  elimina o excedente pintado fora das silhuetas. A pintura continua intacta.
- Acrescentado teste de vértices finitos, opacidade válida e rigidez do tronco
  acima da faixa. Suíte atual: 584 testes Rust passaram; os seis testes de
  animação passaram novamente após o ajuste final. Fmt e Clippy com warnings
  negados passaram. Drafts anteriores permanecem em `/tmp`.
- `leg_alpha_half_width: 0.35` registra a largura real da tinta da coxa, que
  é menor que a largura do PNG por causa da margem transparente. A prova
  `/tmp/cpp-lab-body-alpha-fit` foi aceita nas duas direções; código/assets congelados.
- Staging final repetido após o acabamento em `/tmp/borrow-production-stage-final-c76eq3ls`:
  `stage`/`verify`, Augusta nativa, lab nativo e `--validate` passaram fora do
  checkout, sem overrides de assets e com dados temporários. 383 arquivos
  runtime, 60 da produção; nenhum fallback ou original/review na distribuição.
  [Evidência de empacotamento](../evidence/production-pipeline/packaging/README.md).

## Entrega validada

- [Laboratório](../evidence/production-pipeline/lab/README.md): 40 folhas de clips,
  seis ampliações, 1080 ticks/frames consecutivos, 720p60 por 18s; todos os seis
  golpes tiveram contato e a vitória veio por comandos reais. Pacote exportado
  validado e reaberto nativamente de outro diretório. 37 hashes sem drift.
- [Capítulo](../evidence/production-pipeline/augusta/README.md): 100,334s, 3010
  frames de vídeo a 30fps e 6020 amostras a 60Hz, 14 fases, Complete com HP96.
  Revisados impacto/poeira, piso contínuo, giro e recuperações. Três MP4
  decodificados integralmente; dez PNGs conferidos. 40 hashes sem drift.
- Menus criaram e retomaram os perfis Rust/StreetStart e C++/Arrival em dados
  temporários, preservando o perfil oposto. Nenhum perfil real foi usado.
- Áudio do vídeo reconstruído dos cues e WAVs do runtime; dispositivo físico
  de áudio e gamepad não foram testados. Laboratório gravado sem áudio.
- Validação de código: 584 testes Rust passaram, 2 preexistentes ignorados;
  seis testes de animação repetidos após o último ajuste e 107 testes Python
  passaram. Fmt, Clippy `-D warnings`, isolamento de domínios, links locais e
  `git diff --check` passaram. Staging final também passou (ver acima).
- Nenhuma etapa funcional pendente. Entrega registrada em commit com título
  `feat(adventure): add actor lab and C++ Augusta chapter`; consultar `git log`
  para o identificador, sem embutir hash circular neste diário.

## Limites do piloto

Giros usam vistas pintadas e malhas 2D; não há câmera 3D livre. Python ainda
precisa de seu pacote e capítulo. Novos golpes podem reutilizar os formatos;
novas mecânicas continuam exigindo código/testes. O rig Rust foi preservado.
O cenário interpreta o Baixo Augusta, com estabelecimento/personagens fictícios.
Staging usa binários debug e bibliotecas deste host; não é release publicada.

## Retomada

Inspecionar `git status`, este diário e agentes antes de editar arquivos sob
ownership concorrente. Não apagar alterações ou evidências anteriores.
Os personagens e capítulo novos ficam em `assets/adventure/actors/` e
`assets/adventure/chapters/cpp-augusta/`; runtime novo em `adventure/production`
e `engine/production`, separado do capítulo Rust legado.
