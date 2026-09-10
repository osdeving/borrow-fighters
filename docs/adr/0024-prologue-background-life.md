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

## Extensão — automóveis e acidente

O pedido adicional de trânsito e acidente usa a mesma fronteira: trajetória,
frenagem e impacto são estado decorativo puro em `adventure/ambient`, com
renderização e áudio locais à aventura. O mesmo sinal da EP inicia a fuga
do garoto e o acidente. Cues de buzina, pneus e metal derivam da passagem
pelos marcos da sequência, sem física veicular nem contatos de combate.
O carro amassado permanece até reinício; a via de automóveis ocupa um plano
distinto da ciclovia e da calçada. Pausa e descarte por skip incluem o som.
