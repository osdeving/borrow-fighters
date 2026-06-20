# 02 — Prototype Scope

## Objetivo

Criar o menor protótipo jogável possível para provar a mecânica central de luta.

## Versão-alvo

**Prototype 0.1 — Two Boxes Fighting**

## Entregáveis

### Obrigatórios

- Janela do jogo.
- Loop principal.
- Dois personagens placeholder.
- Chão e limites da arena.
- Movimento horizontal.
- Pulo simples.
- Ataque básico.
- Hitbox.
- Hurtbox.
- Dano.
- Barra de vida.
- Condição de vitória.
- Reinício da partida.

### Desejáveis

- Knockback simples.
- Animação placeholder.
- Tela simples de vitória.
- Pausa.
- Debug draw de hitbox/hurtbox.

### Fora do protótipo 0.1

- Online.
- IA avançada.
- Sprites finais.
- Áudio final.
- Vários personagens.
- Menus completos.
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
| Sistema de luta ficar complexo cedo | Começar com dano fixo e ataque único |
