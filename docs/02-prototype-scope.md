# 02 — Prototype Scope

## Objetivo

Implementar o menor episódio jogável que conecte a lore ao controle de Rust:
prólogo de Ada/Assembly, salto temporal, manhã cotidiana, ataque de uma errática
e vitória com pesar. A [entrega 27](27-rust-story-adventure.md) detalha a sequência
e seus critérios. O escopo de luta abaixo permanece como histórico do produto
já entregue.

## Versão-alvo

**Experimento de aventura — prólogo e primeiro encontro**, na branch
`feature/rust-adventure-prologue`. O produto distribuído permanece
`v0.1.0-prototype.3` até novo corte explícito de release.

## Escopo autorizado da aventura

- Ada começa como humana comum trabalhando; a mensagem misteriosa e o despertar
  de Assembly são encenados sem explicar a origem da mensagem.
- Texto digitado em terminal conduz ao salto de muito tempo.
- Rust acorda numa manhã comum e o controle passa ao jogador.
- Uma errática tenta matá-lo; movimento e combate próprios permitem vencê-la.
- O encerramento mostra Rust balançando a cabeça com pesar e necessidade.
- Pausa, avanço/pulo de texto e nova tentativa permitem revisar a sequência
  sem obrigar o jogador a repetir o prólogo a cada derrota.
- Features e binários separam aventura e luta; apenas `math` e `runtime_paths`
  compõem a base compartilhada inicial. [ADR 0021](adr/0021-isolated-adventure-experiment.md).
- Animações, transições e contato precisam de revisão no renderer real;
  testes de estado não comprovam expressividade humana.

A [entrega33](33-after-the-silence.md), autorizada em10/09/2026, amplia o escopo
com um capítulo de Rust após o prólogo: rua evacuada, contatos presenciais,
telefone com Python, dois trechos conectados e proteção da passagem. Atores e
geometria são modulares; o menu oferece continuar/recomeçar/rever o prólogo.

Vínculo, capítulo em Sirius, plataforma instável, personagem de apoio, campanha
com seleção livre, mapa aberto e múltiplos inimigos simultâneos ficam estacionados. O [diário](worklogs/rust-adventure-prologue.md)
registra entregas e limites observados.

## Status do Prototype 0.1 de luta

O protótipo 0.1 já possui uma versão jogável em Rust + Raylib na `main`.

O corte inicial abaixo foi ampliado por rodadas autorizadas. A
[rodada de conclusão visual](26-playtest-visual-completion.md) acrescenta seleção
visual, pausa/revanche e energia, integradas e distribuídas no Prototype 3.

O slice atual inclui:

- janela, loop principal e carregamento de assets;
- menu principal mínimo com submenus de versus, treino, lore/roster e opções, incluindo seleção de arena e volume de música;
- dois jogadores locais, com Player 2 em CPU por padrão;
- opção de IA para ambos os jogadores;
- movimento horizontal, pulo diagonal, abaixar e defesa;
- soco fraco, soco forte, chute, varredura baixa, overhead, anti-air, agarrão curto, ataques aéreos e especial projectile;
- primeiro corte de identidade mecânica: Rust mais técnico/responsivo, Duke mais longo/pesado, Go como rushdown mantido para CLI/ferramentas, C como fundamentos de alcance/risco e Python como punisher ágil, incluindo `ProjectileSpec` por personagem;
- colisão corpo-corpo;
- hitbox/hurtbox configuráveis no código;
- dano, vida, vitória e reinício;
- flags runtime para HUD, debug, ajuda, gamepad, dano do Player 1 e dano do Player 2;
- seis arenas bitmap com nome/contexto e seleção manual; reinício e revanche preservam a arena base escolhida;
- entrada cinematográfica com contagem pré-luta `11` / `10` / `01` / `Fight!`;
- runtime inicial de sprites por atlas + manifesto JSON;
- animações placeholder de luta, entrada cinematográfica e vitória.
- motor inicial de áudio por eventos, incluindo música por tela/arena, impactos, UI, contagem pré-luta e vozes de golpes por personagem.
- livro de história e fichas de roster carregados de `assets/lore/story.json` em runtime.

O corte distribuído é `v0.1.0-prototype.3`. Playtest humano, timing e acabamento
pendentes estão no [backlog](03-backlog.md); a aventura é a frente ativa atual.

## Entregáveis do corte inicial de luta — histórico

### Obrigatórios

- [x] Janela do jogo.
- [x] Loop principal.
- [x] Dois personagens placeholder.
- [x] Chão e limites da arena.
- [x] Movimento horizontal.
- [x] Pulo simples e diagonal.
- [x] Ataques básicos.
- [x] Hitbox.
- [x] Hurtbox.
- [x] Dano.
- [x] Barra de vida.
- [x] Condição de vitória.
- [x] Reinício da partida.

### Desejáveis

- [x] Knockback simples.
- [x] Animação placeholder.
- [x] Tela simples de vitória.
- [x] Debug draw de hitbox/hurtbox.
- [x] Menu principal mínimo.
- [x] IA simples de playtest.
- [x] Pausa dedicada com continuar, reiniciar, trocar personagens e menu.

### Fora do protótipo 0.1

- Online.
- IA avançada ou competitiva.
- Sprites finais.
- Áudio final.
- Roster grande ou final de personagens.
- Menus completos de produto final, como story mode, perfil ou loja. A seleção visual de lutadores foi autorizada na [rodada 26](26-playtest-visual-completion.md).
- Sistema de combo.
- Story mode.
- ECS sofisticado.
- Editor de fases/personagens.

## Regra de ouro do corte de luta

Se uma feature não ajuda a provar que o combate básico funciona, ela não entra
no corte inicial de luta. A aventura tem autorização e escopo próprios acima;
não amplia automaticamente as regras ou os sistemas desse corte.

## Riscos

| Risco | Mitigação |
|---|---|
| Escopo crescer demais | Manter somente dois personagens placeholder |
| Arte travar desenvolvimento | Usar caixas, formas e placeholders |
| Engine virar o projeto principal | Escrever somente o necessário para o jogo |
| Rust desacelerar prototipação | Preferir código simples e refatorável |
| Sistema de luta ficar complexo cedo | Manter golpes tradicionais, mas sem combo tree antes do feeling básico |
| Assets guiarem hitbox/hurtbox de forma frágil | Manter pivot, hurtbox e hitbox ajustáveis por personagem/ação |
