# 01 — Mini-GDD

## 1. Resumo

**Borrow Fighters** é um jogo 2D de luta com personagens inspirados em linguagens de programação, mascotes e conceitos técnicos.

O primeiro objetivo é criar um protótipo jogável com dois personagens, movimentação, ataque, hitbox/hurtbox, dano, barra de vida e condição de vitória.

## 2. Gênero

- Jogo de luta 2D.
- Side-view.
- Partidas curtas.
- Combate local.
- Protótipo single-player/local antes de qualquer modo online.

## 3. Plataforma inicial

- PC desktop.
- Desenvolvimento local.
- Sem compromisso inicial com Steam, consoles ou mobile.

## 4. Stack inicial

- Linguagem: Rust.
- Biblioteca/engine inicial: Raylib/Raylib-rs.
- Arquitetura: micro-engine própria evolutiva.
- Arte inicial: placeholder.
- Som inicial: placeholder ou ausente.

## 5. Loop principal

1. Jogador se move.
2. Jogador tenta acertar o adversário.
3. Ataques geram hitboxes.
4. Hitboxes colidem com hurtboxes.
5. Dano é aplicado.
6. Barra de vida diminui.
7. Partida termina quando a vida chega a zero.
8. Jogo pode ser reiniciado.

## 6. Mecânicas centrais

### Movimento

- Andar para esquerda/direita.
- Pular.
- Cair.
- Virar automaticamente para o adversário.

### Ataque

- Ataque básico inicial.
- Ataque pesado futuramente.
- Ataque especial futuramente.

### Colisão

Separar:

- colisão física com chão/limites;
- hurtbox: área vulnerável do personagem;
- hitbox: área ofensiva do golpe.

### Dano

- Dano fixo inicialmente.
- Knockback simples pode ser adicionado depois.
- Sem balanceamento complexo no início.

## 7. Personagens iniciais

### Rustacean / Rust

Arquétipo: lutador técnico, seguro e preciso.

Possíveis golpes:

- Borrow Check.
- Lifetime Lock.
- Ownership Transfer.
- Panic!
- Zero-Cost Counter.

### Duke / Java

Arquétipo: lutador verboso, resistente e cheio de pressão.

Possíveis golpes:

- System.out.println Barrage.
- Garbage Collector Sweep.
- AbstractFactory Uppercut.
- NullPointer Trap.
- Enterprise Combo.

## 8. Vertical slice desejado

O vertical slice deve demonstrar uma luta curta entre dois personagens com:

- identidade visual mínima;
- pelo menos dois ataques por personagem;
- feedback visual de impacto;
- barra de vida funcional;
- uma arena simples;
- tela de vitória;
- controles responsivos;
- código modular o suficiente para adicionar novos personagens.

## 9. Fora de escopo inicial

- Online multiplayer.
- Menu completo.
- História.
- Vários personagens.
- Vários cenários.
- Sistema complexo de combo.
- Inteligência artificial avançada.
- Arte final.
- Animações finais.
- Trilha sonora final.
- Balanceamento refinado.

## 10. Critério de sucesso do primeiro protótipo

O primeiro protótipo será considerado bem-sucedido quando:

- dois personagens aparecerem na tela;
- ambos puderem se mover;
- um personagem puder atacar o outro;
- o dano for aplicado corretamente;
- a barra de vida diminuir;
- a partida terminar quando a vida chegar a zero;
- o jogo puder ser reiniciado.
