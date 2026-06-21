# 03 — Backlog Inicial

## Legenda de t-shirt sizing

- **XS**: muito pequeno.
- **S**: pequeno.
- **M**: médio.
- **L**: grande.
- **XL**: muito grande; evitar no protótipo.
- **?**: precisa de investigação.

## Prototype 0.1 — Greybox Fighting Slice

Status do primeiro greybox:

- Mergeado na `main`.
- Cobre o núcleo mínimo jogável com sprites placeholder, cenário bitmap e tela de preferências.
- Ainda não fecha Prototype 0.1 como release; serve para playtest, arte inicial e discussão de feeling.

| Item | Tamanho | Prioridade | Status | Observação |
|---|---:|---:|---|---|
| Criar projeto Rust | S | Alta | Feito | Cargo project básico |
| Configurar Raylib/Raylib-rs | M | Alta | Feito | Validado com checks locais |
| Criar janela e loop principal | S | Alta | Feito | Primeiro teste visual |
| Desenhar arena simples | S | Alta | Feito | Arenas Sirius, Fortaleza Tech Coast e Java Street em rotação |
| Criar entidade Player | M | Alta | Feito | Posição, velocidade, vida e estado |
| Implementar input local | M | Alta | Feito | Teclado e gamepad quando disponível |
| Movimento horizontal | M | Alta | Feito | Esquerda/direita com suavização inicial |
| Gravidade e pulo | M | Alta | Feito | Pulo vertical e diagonal |
| Direção/facing do personagem | S | Média | Feito | Olhar para adversário |
| Soco fraco e forte | M | Alta | Feito | Estados separados |
| Chute | M | Alta | Feito | Usado por jogador e CPU |
| Especial projectile | M | Alta | Feito | Rust gear e Duke bean placeholder |
| Defesa e abaixar | M | Alta | Feito | Inclui leitura visual em sprite |
| Hurtbox | M | Alta | Feito | Ajustável por estado/personagem no código |
| Hitbox | L | Alta | Feito | Área ofensiva temporária |
| Detecção hitbox/hurtbox | L | Alta | Feito | Coração do combate |
| Aplicação de dano | M | Alta | Feito | Flags permitem invencibilidade de P1/P2 |
| Barra de vida | S | Alta | Feito | HUD opcional |
| Condição de vitória | S | Alta | Feito | Vida <= 0 |
| Reinício da partida | S | Média | Feito | Tecla R |
| Debug draw | M | Média | Feito | Toggle de hitbox/hurtbox |
| Runtime de sprites | M | Média | Feito | Atlas + manifesto JSON v1 |
| IA de playtest | M | Média | Feito | P1/P2, perfis diferentes, ataques variados |
| Polimento de timing | M | Alta | Em andamento | Ataques, projectile, spawn e IA ainda precisam tuning |
| Arte placeholder melhorada | L | Alta | Em andamento | Rust, Duke, entrada e cenário ainda não são finais |

## Fora do backlog inicial

| Item | Motivo |
|---|---|
| Online multiplayer | Complexidade XL |
| Sistema de combo | Depende do feeling básico |
| Vários personagens | Depende da abstração mínima |
| Arte final | Depende do vertical slice |
| Trilha sonora | Não prova gameplay |
| Menu completo | Não prova gameplay |
| IA avançada | A IA atual é apenas playtest, não desafio competitivo |
