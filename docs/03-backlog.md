# 03 — Backlog Inicial

## Fonte de verdade

Este documento e a fonte de verdade para **o que vem agora**.

Roadmaps especializados continuam existindo, mas devem apontar para este backlog quando uma frente virar trabalho ativo:

- combate e balanceamento: [`docs/13-combat-design-roadmap.md`](13-combat-design-roadmap.md);
- sprites, atlas e ferramenta de inspecao: [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md);
- arte e mood: [`docs/07-art-direction.md`](07-art-direction.md);
- audio: [`docs/14-audio-pipeline.md`](14-audio-pipeline.md);
- processo, PRs e GitHub: [`docs/05-governance.md`](05-governance.md).

Regra operacional:

1. toda frente ativa deve aparecer na tabela **Agora / Proximo / Depois** abaixo;
2. toda tarefa aceita deve ter issue ou PR relacionado quando sair de ideia para execucao;
3. toda mudanca que altera comandos, processo, formato de dados, roadmap ou contribuicao deve atualizar este backlog ou o roadmap especializado correspondente;
4. se uma decisao criar padrao duradouro, registrar ADR.

## Agora / Proximo / Depois

| Janela | Frente | Status | Registro | Proxima acao |
|---|---|---|---|---|
| Feito local | Runtime de `frames[].combat` | Feito | [`docs/11-sprite-pipeline.md`](11-sprite-pipeline.md), [`docs/12-technical-combat-guide.md`](12-technical-combat-guide.md), [`docs/adr/0007-sprite-frame-combat-runtime.md`](adr/0007-sprite-frame-combat-runtime.md) | Runtime consome hitboxes, hurtboxes e origem de projectile do manifesto com fallback para o greybox. |
| Agora | Calibracao de sprite/hitbox | Em andamento local | [`docs/11-sprite-pipeline.md`](11-sprite-pipeline.md), [`docs/12-technical-combat-guide.md`](12-technical-combat-guide.md), [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md) | Usar `H`/`J`/`Delete` no viewer para ajustar hitboxes dos golpes ativos por frame, começando por Rust jab e special. |
| Proximo | Feeling e balanceamento | Planejado | [`docs/13-combat-design-roadmap.md`](13-combat-design-roadmap.md), [`docs/15-character-combat-matrix.md`](15-character-combat-matrix.md) | Playtestar Rust x Duke x Go com Combat Lab e ajustar frame data por dados, nao por achismo. |
| Proximo | Arte candidata | Planejado | [`docs/07-art-direction.md`](07-art-direction.md), [`docs/11-sprite-pipeline.md`](11-sprite-pipeline.md) | Definir criterio de aceitacao para o primeiro atlas candidato de Rust antes de pedir polimento final. |
| Depois | Ferramenta visual clicavel | Aberto | [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md) | Avaliar `raygui` somente se atalhos e texto ficarem insuficientes. |
| Depois | Release Prototype 0.1 | Aberto | [`docs/06-release-process.md`](06-release-process.md) | Criar milestone/release checklist quando o slice tiver playtest minimo e assets candidatos. |

## Ritual de manutencao

Antes de abrir branch:

- verificar esta tabela;
- confirmar se existe issue ou PR para a frente ativa;
- se nao existir, criar issue pequena ou atualizar este backlog.

Antes de mergear PR:

- atualizar este backlog se a mudanca concluiu, criou ou reordenou trabalho;
- atualizar o roadmap especializado quando houver detalhe tecnico de uma frente;
- registrar no [`CHANGELOG.md`](../CHANGELOG.md) mudancas relevantes.

Depois de mergear:

- fechar ou comentar a issue relacionada;
- decidir explicitamente qual linha da tabela vira o novo **Agora**.

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
| Especial projectile | M | Alta | Feito | `ProjectileSpec` por personagem para Rust, Duke e Go |
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
