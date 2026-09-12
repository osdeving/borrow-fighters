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

## Verificação pendente

`cargo fmt`, Clippy, testes Rust, isolamento de domínios e links locais;
captura nativa completa, inspeção de stills e vídeo; skip/retry/pausa/handoff;
commit das etapas coerentes e relatório final com limites reais.
