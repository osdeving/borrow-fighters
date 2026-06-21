# 01 — Mini-GDD

## 1. Resumo

**Borrow Fighters** é um jogo 2D de luta com personagens inspirados em linguagens de programação, mascotes e conceitos técnicos.

O primeiro objetivo é criar um protótipo jogável com dois personagens, movimentação, ataque, hitbox/hurtbox, dano, barra de vida e condição de vitória.

A direção narrativa coloca essas entidades em arenas brasileiras de ciência, tecnologia, arquitetura e inovação. O pano de fundo é cósmico e levemente misterioso: linguagens, runtimes e bugs antigos ganharam forma depois de um fenômeno conhecido como **O Linker**.

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

Papel narrativo: protagonista clássico. Rust deve ser fácil de gostar: disciplinado, corajoso, cuidadoso com quem está ao redor e convencido de que poder sem controle não é força. Ele luta para entender o Linker e impedir que entidades instáveis sejam consumidas por camadas antigas demais da máquina.

Possíveis golpes:

- Borrow Check.
- Lifetime Lock.
- Ownership Transfer.
- Panic!
- Zero-Cost Counter.

### Duke / Java

Arquétipo: lutador verboso, resistente e cheio de pressão.

Papel narrativo: veterano carismático. Duke é teatral, antigo, cheio de cerimônia e mais sábio do que parece. Ele provoca Rust, mas também protege sistemas legados que ainda sustentam muita coisa viva. Deve ser engraçado sem virar descartável.

Possíveis golpes:

- System.out.println Barrage.
- Garbage Collector Sweep.
- AbstractFactory Uppercut.
- NullPointer Trap.
- Enterprise Combo.

### Assembly

Arquétipo: boss final não-jogável, antigo, poderoso e quase místico.

Assembly representa a camada anterior às abstrações confortáveis. Ele não é vilão simples: é detentor de uma sabedoria pesada, precisa e difícil de encarar. Sua presença deve carregar horror cósmico de baixo nível, como se toda linguagem moderna fosse apenas uma sombra projetada sobre o metal.

Mais detalhes de história, personagem e arenas estão em [`docs/12-worldbuilding.md`](12-worldbuilding.md).

## 8. Arenas

As arenas principais devem ser inspiradas em locais brasileiros de tecnologia, ciência, arquitetura e inovação.

Direções iniciais:

- MASP, em São Paulo, como portal urbano e arena suspensa;
- Sirius/LNLS, em Campinas, como altar científico de luz e matéria;
- Brasília/BioTIC, como cidade planejada e biotecnológica;
- Fortaleza, como costa tecnológica de energia, formação e sistemas;
- Curitiba/Vale do Pinhão, como smart city cartunesca;
- Recife/Porto Digital, como passado e futuro no mesmo commit.

A lista completa de possibilidades e ganchos narrativos vive em [`docs/12-worldbuilding.md`](12-worldbuilding.md).

## 9. Vertical slice desejado

O vertical slice deve demonstrar uma luta curta entre dois personagens com:

- identidade visual mínima;
- pelo menos dois ataques por personagem;
- feedback visual de impacto;
- barra de vida funcional;
- uma arena simples;
- tela de vitória;
- controles responsivos;
- código modular o suficiente para adicionar novos personagens.

## 10. Fora de escopo inicial

- Online multiplayer.
- Menu completo.
- Modo história completo.
- Vários personagens.
- Vários cenários.
- Sistema complexo de combo.
- Inteligência artificial avançada.
- Arte final.
- Animações finais.
- Trilha sonora final.
- Balanceamento refinado.

## 11. Critério de sucesso do primeiro protótipo

O primeiro protótipo será considerado bem-sucedido quando:

- dois personagens aparecerem na tela;
- ambos puderem se mover;
- um personagem puder atacar o outro;
- o dano for aplicado corretamente;
- a barra de vida diminuir;
- a partida terminar quando a vida chegar a zero;
- o jogo puder ser reiniciado.
