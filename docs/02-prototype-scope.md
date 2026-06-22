# 02 — Prototype Scope

## Objetivo

Criar o menor protótipo jogável possível para provar a mecânica central de luta e orientar a produção visual inicial.

## Versão-alvo

**Prototype 0.1 — Greybox Fighting Slice**

## Status atual

O protótipo 0.1 já possui uma versão jogável em Rust + Raylib na `main`.

O slice atual inclui:

- janela, loop principal e carregamento de assets;
- tela inicial de preferências com seleção mínima de personagens;
- dois jogadores locais, com Player 2 em CPU por padrão;
- opção de IA para ambos os jogadores;
- movimento horizontal, pulo diagonal, abaixar e defesa;
- soco fraco, soco forte, chute, varredura baixa, overhead, anti-air, agarrão curto, ataques aéreos e especial projectile;
- primeiro corte de identidade mecânica: Rust mais técnico/responsivo, Duke mais longo/pesado e Go como rushdown greybox no Combat Lab, no menu e em match via CLI;
- colisão corpo-corpo;
- hitbox/hurtbox configuráveis no código;
- dano, vida, vitória e reinício;
- flags runtime para HUD, debug, ajuda, gamepad, dano do Player 1 e dano do Player 2;
- arenas bitmap Sirius, Fortaleza Tech Coast e Java Street com rotação no início da próxima luta após uma vitória;
- entrada cinematográfica com contagem pré-luta `11` / `10` / `01` / `Fight!`;
- runtime inicial de sprites por atlas + manifesto JSON;
- animações placeholder de luta, entrada cinematográfica e vitória.
- motor inicial de áudio por eventos, incluindo música, impactos, UI e contagem pré-luta.

Ainda não é uma release fechada. O objetivo agora é melhorar leitura visual, timing, feeling e documentação de contribuição para artistas.

## Entregáveis

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
- [x] Tela de preferências.
- [x] IA simples de playtest.
- [ ] Pausa dedicada.

### Fora do protótipo 0.1

- Online.
- IA avançada ou competitiva.
- Sprites finais.
- Áudio final.
- Vários personagens.
- Menus completos além da tela de preferências.
- Sistema de combo.
- Story mode.
- ECS sofisticado.
- Editor de fases/personagens.

## Regra de ouro

Se uma feature não ajuda a provar que o combate básico funciona, ela não entra no protótipo 0.1.

## Riscos

| Risco | Mitigação |
|---|---|
| Escopo crescer demais | Manter somente dois personagens placeholder |
| Arte travar desenvolvimento | Usar caixas, formas e placeholders |
| Engine virar o projeto principal | Escrever somente o necessário para o jogo |
| Rust desacelerar prototipação | Preferir código simples e refatorável |
| Sistema de luta ficar complexo cedo | Manter golpes tradicionais, mas sem combo tree antes do feeling básico |
| Assets guiarem hitbox/hurtbox de forma frágil | Manter pivot, hurtbox e hitbox ajustáveis por personagem/ação |
