# ADR 0024 — Vida de fundo no prólogo

## Status

Aceito para o experimento solicitado em 10 de setembro de 2026.

## Contexto

A manhã precisa de uma casa com identidade de Rust e uma rua com ciclistas
e um garoto que solta a pipa e foge quando a errática se manifesta. O usuário
pediu explicitamente uma faixa de fundo inacessível a Rust para dispensar
colisão com esses atores.

## Decisão

Manter estado e desenho da encenação em `adventure`, com relógio de updates
fixos e reação ao estado já existente `combat.enemy_awake`. A encenação
consome esse sinal, sem decidir dano, movimento ou resultado do encontro.
Pausa preserva o relógio, e retry/restart restauram o estado apropriado.

Ciclovia e faixa do garoto ficam atrás da área jogável, com separação visível
de profundidade. Seus atores nunca entram nas coleções de corpos, hitboxes
ou contatos do combate. Texturas, poses e créditos pertencem a
`assets/adventure/`; não se importam os atores de cenário do jogo de luta.

O quarto mantém colchão, borda e chão usados pelos apoios de Rust. Telas e
pôsteres acrescentam identidade sem exigir novas poses ou um editor de cenas.

## Consequências

Uma pequena estrutura explícita permite testar fuga, continuidade, pausa e
retry sem janela. A renderização usa esse estado para escolher poses e
posições, sem ECS, eventos genéricos ou física de NPCs. A camada de fundo
deve ser conferida com a câmera nas duas extremidades e durante o combate.

Implementação e evidência: [entrega 30](../30-prologue-scene-improvements.md).
