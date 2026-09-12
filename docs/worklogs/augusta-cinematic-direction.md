# Augusta — direção cinematográfica e encenação

## Objetivo autorizado

Em 12/09/2026: corrigir braço ausente em idle da C++, Julia atrás do cafetão
segurada pelo braço, fuga dele na chegada das EPs e seguranças saindo do bar.
Ampliar a apresentação do bairro e a narrativa com atuação, closes, movimentos
de câmera e uso de 3D quando viável, preservando identidade e geografia.
Commits de etapas foram explicitamente autorizados.

## Estado inicial e retomada

- Branch: `feat/augusta-cinematic-direction`.
- Base: `ad2f034`, `main` limpa; nada pendente para o commit inicial.
- Leia este diário, `git status --short` e `git log -5 --oneline` antes de retomar.
- Preserve mudanças em andamento dos agentes; commits são coordenados pelo root.
- Skills: repo-atlas, rust-gamedev, gameplay-design e art-direction.
- Contexto: README, docs 07/08/38 e ADRs 0001/0003/0021/0032.

## Decisão em avaliação

Usar um palco 3D limitado à frente das fachadas pintadas: perspectiva, travelling,
grua e arcos oblíquos sobre posições fixas, com atores ilustrados e o mesmo mapa.
As vistas inexistentes dos personagens não serão inventadas pelo renderer.
Validar a aparência nativa antes de fechar a implementação e registrar ADR.

## Trabalho

- Em andamento: braços e recuperação (agente cpp_arm).
- Em andamento: fases, contato, porta, fuga e regressões (augusta_staging).
- Em andamento: vida noturna e evacuação determinística (augusta_ambient).
- Em andamento: câmera, palco, integração, evidência e revisão (root).

## Checkpoint — correções e primeiro palco

- `fec2a87`: braços corrigidos por alvos de pose; evidência antes/depois nos dois
  sentidos e vídeo de 450 ticks. Sete testes de animação aprovados pelo agente.
- Encenação corrigida: JuliaAttempt antes da conversa, contato curto, porta única
  [1536,500], guarda contorna C++ pela frente; medo/fuga ocorre na chegada EP.
- Relógio de ameaça atravessa cortes e mantém evacuação após retry. Dezesseis
  testes Augusta integrados passaram, incluindo duas vitórias reais.
- Palco 3D integrado em `engine/production/cinema.rs`, com câmera pura em
  `augusta/cinema.rs`; ADR 0033 registra limites do elenco ilustrado em planos.
- 22 adultos e cinco veículos do bairro, com animação e evacuação determinística.
- Draft nativo `/tmp/augusta-cinema-draft1` revelou z-fighting das primitivas
  transparentes: corrigida descarga dos batches antes de alternar depth test.
- Draft `/tmp/augusta-cinema-draft2` em revisão; contato pareado ainda em polimento.
- `cargo check --all-targets` passou. Fmt/Clippy/testes globais ainda pendentes
  até estabilizar integração. Não usar essas capturas preliminares como entrega.

## Verificação pendente

`cargo fmt`, Clippy, testes Rust, isolamento de domínios e links locais;
captura nativa completa, inspeção de stills e vídeo; skip/retry/pausa/handoff;
commit das etapas coerentes e relatório final com limites reais.

## Checkpoint — sequência completa e acabamento

- Draft3 `/tmp/augusta-cinema-draft3`: 8.238 frames, 18 tomadas nomeadas,
  `Complete`, HP96, sem forçar resultados. Continua preliminar para arte.
- Primeira matriz global: `cargo clippy --all-targets -- -D warnings` passou;
  `cargo test --all-targets`: 597 passaram, 0 falharam, 2 ignorados preexistentes.
- Auditoria independente corrigiu enquadramento da ameaça no alto, estado único
  da porta, dica de skip durante diálogo, cópia obrigatória e fachada à esquerda.
- Reprovação visual do contato por overlay: arte conjunta nova com identidade
  preservada foi produzida; integração e prova de chroma ainda em andamento.
- Em acabamento: passada dos NPCs e escala adulta da população de fundo.
- Áudio: porta, passos, ruptura e pânico; tráfego some em360 ticks, ar permanece.
  Gerador completo `tools/audio/generate_augusta_cinema_audio.py`; empacotador
  passou a seguir também o loop `air` do catálogo.
- Python padrão do host é antigo: usar `python3.13` para isolamento/pacote.
- Próximo: congelar arte/código, repetir checks afetados, captura final completa,
  rodar `tools/review/check_augusta_cinema_x11.py`, mixar cues e publicar evidência.

## Checkpoint — encenação validada, figurantes em revisão

- Palco, porta persistente, par pintado com punho preso e passadas dos NPCs
  concluídos. Revisão independente de sete planos aprovou anatomia, contornos,
  apoios, saída dos guardas e fuga durante a chegada das EPs.
- Matriz global: fmt e Clippy estrito passaram; 601 testes passaram, 0 falharam,
  2 ignorados preexistentes. Isolamento de domínios aprovado, 33 testes de
  empacotamento e 2 testes do mixer aprovados.
- `/tmp/augusta-cinema-native-final/native-checks.json`: interação X11 real
  aprovada para pausa, avanço, skip, handoff de três guardas e F5.
- Pacote isolado `/tmp/augusta-cinema-portable` verificado e carregado por
  90 frames sem variáveis de assets apontando para o checkout.
- A captura completa `/tmp/augusta-cinematic-final` fica **preliminar**: o usuário
  apontou a diferença de acabamento dos figurantes geométricos. Essa observação
  amplia o acabamento visual pendente; não entregar essas imagens como finais.
- Nova etapa: `cpp_arm` produz oito figurinos adultos pintados com imagegen;
  `augusta_ambient` integra recortes articulados preservando poses e trajetórias;
  `augusta_staging` audita a sequência e as verificações existentes; root integra,
  revisa closes/gameplay e repete checks afetados antes de nova captura.
- Skills desta etapa: imagegen, além de art-direction e rust-gamedev. Prompts,
  referências e arquivos selecionados devem ficar no repositório.

## Checkpoint — figurantes pintados integrados

- `c32c56b` preserva a encenação validada antes da troca dos figurantes.
- Duas folhas imagegen `sprites/nightlife-cast-{a,b}.png` aprovadas como fontes:
  oito adultos distintos, cada pintura com aproximadamente 600px de altura.
  `nightlife-cast.json` registra recortes; world-art aponta para esse catálogo.
- `painted_crowd.rs` substitui os humanos geométricos nas duas câmeras, mantendo
  poses e evacuação do modelo. O carregador valida o elenco antes de alocar GPU.
- Empacotamento inclui o novo catálogo e somente as folhas referenciadas.
  Todos os 35 testes Python do pacote passaram; 266 links locais válidos.
- Build da integração passou. `/tmp/augusta-painted-candidate` contém uma
  abertura de 1.250 frames com estilo aprovado nos planos gerais.
- **Ainda não concluir:** painel ampliado `/tmp/painted-crowd-review/frame000.png`
  revelou braços ocultos, ombros abertos e pequenos recortes na cintura. Agentes
  `augusta_ambient` e `cpp_arm` ajustam os recortes/ordem antes da captura final.
- Root aguarda rig revisado para matriz Rust, captura completa e controles;
  `augusta_staging` repetirá stage/verify/smoke portátil com o binário estável.
- Usuário reforçou que braços não podem entrar no corpo e pediu inspeção dos
  frames. `tools/review/capture_augusta_crowd.py` compila uma galeria nativa
  (`review_augusta_crowd.rs`) com 900 frames: cinco atuações contínuas, oito
  pinturas e ambos os sentidos; vídeo de 30 segundos e hashes dos recursos.
  Sem fallback que congele o ciclo de figurantes saindo da rua. Usar esta galeria
  além da captura do capítulo e conservar quadros sequenciais da revisão aceita.

## Checkpoint — ciclos íntegros, orientação do tronco em correção

- As bandas de membros foram substituídas por uma malha contínua por máscara,
  com ombros conectados, cotovelos baixos e tecido acompanhando as coxas.
  Galeria `/tmp/augusta-crowd-frames-final`: 900 PNGs e MP4, cinco atuações,
  oito figurinos, ambos os sentidos; revisão de sequências aprovou anatomia.
- Matriz dessa revisão: 606 testes passaram, 0 falharam, 2 ignorados; fmt,
  Clippy estrito e fronteiras aprovados. Pacote com 392 assets/30 texturas
  passou stage, verify e smoke em `/tmp/augusta-painted-portable`.
- **Nova correção obrigatória do usuário:** andar lateralmente conservava
  o peito voltado à câmera. Toda evidência acima passa a ser preliminar para
  orientação. Frontais ficam parados; caminhada/fuga passam a oito pinturas
  de perfil real, produzidas por `cpp_arm`, preservando identidade e roupa.
- `augusta_ambient` acrescentou `profile_entries` obrigatório, seleção por
  atividade e slots explícitos de membros. Código compila; load/testes de
  catálogo aguardam os perfis e seus registros. Não tentar entregar esse
  estado intermediário sem completar o JSON.
- Root acrescentou cache de eixos das juntas, altura de barra, posição de
  uniform e projeção de vértices, eliminando operações repetidas sem mudar
  a autoria da atuação. Checklib passa; aguardar dados para matriz completa.
- Native input da revisão pintada falhou ao perder um toque curto de pausa
  durante capturas concorrentes. Medição posterior sem captura revelou também
  ~20–22 FPS no debug antigo deste host WSL. Comparar o cache/perfis em execução
  isolada; não reportar como simples interferência entre ferramentas.
- `/tmp/augusta-painted-film-final` foi interrompida por root após a correção
  do usuário; é draft. Regravar o capítulo, galeria e controles na versão final.
- Responsabilidades: cpp_arm PNGs/máscaras/perfis/proveniência; ambient seleção,
  slots e revisão de perfil; root cache, integração, verificação e commits;
  staging mediu performance e repetirá pacote quando o perfil estiver fechado.
