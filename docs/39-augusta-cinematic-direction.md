# 39 — Augusta: encenação e câmera cinematográfica

A revisão transforma a apresentação do capítulo em tomadas dirigidas, com
perspectiva 3D real, mantendo o combate lateral e a identidade ilustrada.
O cenário é o mesmo mapa: letreiro Limiar, porta, fachadas vizinhas e posições
dos personagens não mudam de lugar quando a câmera corta.

## Sequência

1. **Bairro, 20 segundos:** grua sobre as fachadas, travelling pela vida noturna,
   apresentação do Limiar, Julia retida e revelação de C++.
2. **Aproximação jogável:** C++ caminha até o conflito. Ao chegar, a câmera
   mostra Julia reconhecendo a amiga e tentando se soltar, antes da conversa.
3. **Tentativa de Julia, 7 segundos:** close de reação, detalhe da pegada no
   braço e plano que relaciona C++ ao par. Julia fica à direita e atrás do
   cafetão, considerando C++ à esquerda. Ela tem 22 anos, conforme o catálogo.
4. **Confronto:** planos próximos acompanham quem fala, mantendo o eixo da rua.
5. **Seguranças, 9 segundos:** a porta abre e três homens saem por ela em
   intervalos. O primeiro contorna C++ pelo primeiro plano. Nenhum ataca antes
   da entrega ao jogador; os apoios finais coincidem com a simulação.
6. **Chegada das EPs, 8 segundos:** ameaça no alto, reação assustada do cafetão,
   fuga dele e queda das erráticas. O bairro evacua. O cafetão já saiu quando
   o combate começa; derrotar as EPs permite seguir diretamente ao resgate.
7. **Resgate e saída:** conversa com planos próximos e deslocamento acompanhado.

As conversas continuam sob controle do jogador. **Enter / A** avança falas;
**Backspace / View** conclui a atuação atual. Pular uma tomada leva ao estado
final daquela encenação e não elimina inimigos. **Esc / Start** pausa o relógio.
Derrota e retomada das EPs conservam a rua evacuada e o cafetão ausente.

## Continuidade e estilo

O palco usa fachadas pintadas sobre volumes rasos, chão texturizado, calçada
e meio-fio em profundidade. Os atores conservam as pinturas e os rigs usados
no gameplay. A câmera faz grua, travelling, aproximação e arcos oblíquos na
frente da rua. Personagens continuam ilustrados em planos; esta etapa não
cria modelos humanos 3D completos nem uma câmera livre de 360 graus.

O braço posterior da C++ tinha seu alvo atrás do tronco. A pose de repouso
e os retornos agora deixam as duas mãos visíveis. As imagens originais foram
preservadas. O contato de Julia tem uma pintura conjunta própria, compartilhada
pelos planos da cinemática e pelo gameplay. Uma malha acompanha a tentativa de
saída e mantém o punho preso; o shader retira o fundo verde da fonte em runtime.
As passadas dos NPCs articulam recortes das pinturas existentes, com pernas,
apoios e contrapasso, sem redesenhar o rosto e a roupa a cada quadro.

Há 22 adultos decorativos, oito figurinos pintados, grupos conversando,
pessoas nas mesas, mulheres na vida noturna, telefone, vasos e cinco veículos,
incluindo táxi e entrega de moto. Rostos, roupas e proporções dos figurantes
seguem o acabamento ilustrado do elenco principal. Máscaras da própria pintura
formam malhas contínuas nos braços e pernas; o tecido acompanha as coxas e as
solas mantêm o apoio. Caminhada e fuga usam pinturas próprias de perfil, com
peito, quadril e sapatos voltados para o deslocamento. Os oito figurinos têm
16 registros; conversas, mesas e telefone mantêm as vistas frontais.
O mesmo relógio os mantém em posição entre cortes.
Ao surgir a ameaça, fogem sem reaparecer durante o encontro. Móveis e veículos
completam o ambiente com desenhos locais do renderer.

Os carros têm aproximadamente 440px de comprimento, proporcionais aos adultos
de 167–194px, e a moto com piloto mede 170px de altura. As duas faixas mantêm
o mesmo tamanho físico por veículo; a perspectiva 3D fornece a diferença de
profundidade. Rodas usam distância/raio para acompanhar o deslocamento, e as
passagens na câmera jogável ficam abaixo da área principal do combate.

Porta, passos, ruptura e pânico têm Foley original sintetizado, sem vozes.
O tráfego se apaga nos seis primeiros segundos da ameaça; o ar continua.
A pausa suspende os sons e a retomada não repete cues abandonados.

## Arquivos de autoria

| Arquivo | Responsabilidade |
| --- | --- |
| [chapter.json](../assets/adventure/chapters/cpp-augusta/chapter.json) | Durações das atuações |
| [world.json](../assets/adventure/chapters/cpp-augusta/world.json) | Porta, posições e fachadas |
| [texts.json](../assets/adventure/chapters/cpp-augusta/texts.json) | Falas e legendas |
| [augusta/chapter.rs](../src/adventure/augusta/chapter.rs) | Contato, trajetórias, fuga e progressão |
| [augusta/cinema.rs](../src/adventure/augusta/cinema.rs) | Tomadas, lentes, trilhos e cortes |
| [augusta/ambient.rs](../src/adventure/augusta/ambient.rs) | Vida noturna e evacuação determinística |
| [production/cinema.rs](../src/adventure/engine/production/cinema.rs) | Palco e projeção 3D |
| [production/restraint.rs](../src/adventure/engine/production/restraint.rs) | Atuação pareada com contato |
| [production/nightlife.rs](../src/adventure/engine/production/nightlife.rs) | Figurantes, móveis e veículos |
| [production/painted_crowd.rs](../src/adventure/engine/production/painted_crowd.rs) | Recortes pintados e articulação dos figurantes |
| [nightlife-cast.json](../assets/adventure/chapters/cpp-augusta/nightlife-cast.json) | Registros de corpo e membros dos oito figurinos |
| [Proveniência dos figurantes](../assets/adventure/chapters/cpp-augusta/source/nightlife-cast.provenance.json) | Prompts, referências e hashes das pinturas imagegen |
| [Proveniência dos perfis](../assets/adventure/chapters/cpp-augusta/source/nightlife-profile.provenance.json) | Vistas laterais para caminhada e fuga, preservando os oito figurinos |
| [production/frame_motion.rs](../src/adventure/engine/production/frame_motion.rs) | Passadas dos NPCs com a pintura existente |
| [clips.json](../assets/adventure/actors/cpp/clips.json) | Duas mãos legíveis em repouso e recuperações |

A câmera é autoria Rust localizada; alterar trilhos requer recompilar. Os
assets/tempos/textos continuam externos. A [ADR 0033](adr/0033-augusta-cinematic-stage.md)
registra a decisão, e o [diário](worklogs/augusta-cinematic-direction.md) mantém
comandos, revisões e instruções de recuperação.

```sh
cargo run --release -- --start augusta
# Captura reproduzível, com perfil próprio no diretório da revisão:
cargo run --release -- --start augusta --hidden --review /tmp/augusta-film --frames 24000
python3 tools/review/mix_augusta_review_audio.py /tmp/augusta-film
# 900 frames das cinco atuações, com oito figurinos nos dois sentidos:
python3 tools/review/capture_augusta_crowd.py /tmp/augusta-crowd-frames
```

O modo de revisão atravessa o capítulo com controles normais, sem forçar dano
ou vitória. Telemetria registra tomadas, poses de chegada, NPCs e relógio de
ameaça. O áudio da revisão é reconstruído dos cues e amostras do runtime.
As evidências e verificações finais são registradas no diário de trabalho.
