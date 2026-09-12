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
