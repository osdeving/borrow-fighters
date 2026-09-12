# 38 — C++, Augusta e produção fora da campanha

## O piloto

O capítulo **Rua Augusta** acompanha C++ em uma história paralela a Rust e
Python. Julia, sua amiga adulta de 22 anos, quer ir embora e é impedida pelo
cafetão. C++ enfrenta três seguranças; depois, duas erráticas descem dos lados
opostos. O resgate termina em conversa e saída acompanhada. As vitórias vêm
do combate; avançar falas não elimina inimigos.

O gênero desta parte é **aventura de ação 2D cinematográfica com rolagem
lateral**, exploração e combate corpo a corpo. O estilo visual combina
personagens ilustrados de influência anime/cartoon, pintura de cenários e
efeitos gráficos. Versus continua sendo o jogo de luta separado.

Esta etapa mantém Rust/Raylib e inaugura uma pipeline reutilizável dentro da
aventura. A [ADR 0032](adr/0032-character-production-and-independent-chapters.md)
explica o recorte. O rig e o capítulo antigos de Rust não foram migrados.

## Usar sem abrir a campanha

```sh
# Inspecionar C++ diretamente, com timeline e controle de poses.
cargo run --bin borrow-actor-lab

# Validar um pacote sem janela ou texturas na GPU.
cargo run --bin borrow-actor-lab -- --actor assets/adventure/actors/cpp/character.json --validate

# Testar os mesmos golpes contra uma errática, fora do mapa/campanha.
cargo run --bin borrow-actor-lab -- --enemy assets/adventure/actors/erratic/character.json

# Exportar para um diretório novo: poses, vídeo, telemetria e fonte portátil.
cargo run --bin borrow-actor-lab -- --enemy assets/adventure/actors/security/character.json --review /tmp/cpp-review

# Entrar diretamente no capítulo integrado.
cargo run -- --start augusta
```

Para compilar somente a aventura, acrescentar `--no-default-features --features
adventure` antes de `--bin`. O laboratório não carrega cenário, abertura, Ada,
Rust ou catálogo da luta. `--actor outro/character.json` é o ponto de entrada
para outra personagem. Ainda é necessário produzir a arte e os movimentos
dela: o formato não transforma um desenho frontal em vistas inexistentes.

No laboratório: **Tab** escolhe clip, **Espaço** pausa, **←/→** avança quadros,
**Enter** alterna prévia/arena, **F3** mostra geometria, **F5** recarrega o pacote,
**R** reinicia. A recarga conserva o cursor da prévia; na arena, reinicia o
combate com o novo pacote. Só um candidato completamente válido substitui o
anterior. [Protocolo e comandos completos](evidence/production-pipeline/lab/README.md).

## Artefatos equivalentes a um projeto de editor

| Arquivo | Responsabilidade | Alteração isolada |
| --- | --- | --- |
| `actors/<id>/character.json` | Identidade, corpo, movimento, defesa e referências | Velocidade, HP, elenco de golpes |
| `combat.json` | Startup, contato, recuperação, combo e projétil | Alcance/timing/dano sem alterar desenho |
| `rig.json` | Peças, pivôs, dimensões, malha, vistas e efeitos | Corrigir tênis, ombro, bolsa ou aparência do projétil |
| `clips.json` | Poses, alvos, curvas, duração e transição | Corrigir um apoio ou giro sem gerar novas imagens |
| `source/` + `import.json` | Pinturas selecionadas e receita explícita | Substituir uma peça preservando o restante |
| `import-manifest.json` | Recortes, landmarks, alpha e hashes | Auditar/reproduzir o export |
| `chapter.json` | Identidade do capítulo e referências | Trocar seu pacote de mapa/textos/arte |
| `world.json` | Limites, posições, módulos e gatilhos | Ampliar a rua e reposicionar acontecimentos |
| `world-art.json` | Catálogo visual, elenco, placas e parallax | Trocar fachadas sem mudar a física |
| `texts.json` | Falas, nomes, objetivos e comandos | Revisar narrativa sem recompilar |

Os formatos de ator têm `schema_version: 1`; combate e clips usam **60 ticks/s**.
O chão é a origem local do personagem, X positivo aponta para a frente e Y
negativo aponta para cima. Facing espelha essa coordenada. Pivôs são pixels
do recorte; `size` é dimensão no mundo. Janelas de contato usam `[início,fim)`.

Não há um conversor diferente para cada personagem. O [importador](../tools/art/README-import-adventure.md)
usa recortes explícitos, preserva as fontes e rejeita saídas vazias, coordenadas
inválidas e caminhos que saem do pacote. `--check` repete o export em uma pasta
temporária e compara hashes/bytes. Os JSONs de rig e clips são os arquivos de
autoria, não descrições para pedir novamente a mesma imagem à IA.

## Movimento e combate

C++ usa peças rígidas para a parte superior do tronco, braços, botas e bolsa,
e uma malha contínua para cada perna. A faixa `body_leg_blend` de cada vista
liga o tecido do quadril às coxas; o excedente pintado do rodapé se mistura
às silhuetas articuladas. `leg_alpha_half_width` registra a meia largura visível
da pintura da coxa em coordenada U, descontando a margem transparente da imagem.
Assim, levantar a perna não deixa uma ponta fixa
de calça sob o tronco. A malha mantém a largura na dobra; calça e canela não são dois
recortes sobrepostos. Pontos de apoio e articulações são avaliados fora do
renderer. As imagens usam mipmaps/filtragem trilinear para reduzir granulação
ao desenhar uma fonte grande em tamanho de jogo.

O ciclo de corrida avança pela **distância percorrida**. O trecho plantado do pé
tem deslocamento linear compatível com essa distância; os alvos do braço
seguem arcos contínuos. Partida, parada, inversão, salto e aterrissagem possuem
clips próprios e transições. O laboratório e o capítulo chamam o mesmo
avaliador e o mesmo renderer. Atualizações das transições acompanham o tick
fixo, e não a quantidade de chamadas de desenho.

O giro percorre vistas pintadas de perfil, três quartos e costas, com ângulo
contínuo no arquivo. O campo `yaw_interpolation` usa `shortest` por padrão:
330°→0° cruza zero pelo arco de 30°. O clip de giro declara `unwrapped` para
preservar o percurso completo de 330°→720°. É uma rotação encenada de um
personagem 2D; não há câmera 3D livre ou síntese automática de qualquer vista. Animação por quadros também
é aceita no mesmo contrato, usada nos NPCs e na EP neste piloto. Esses clips
por quadros usam relógio e não o modo de stride da malha.

Já há combo leve de três golpes, chute, giro, guarda, parry e projétil Linker.
Startup, janelas de contato, hitstop, knockback, buffer e recuperação pertencem
à simulação. Cada execução atinge cada alvo no máximo uma vez. Projéteis usam
varredura do deslocamento para não atravessar alvos entre ticks. Elenco e
projéteis têm IDs estáveis; aliados não são atingidos. Aparência do projétil
é resolvida pelo `visual_id` no catálogo de efeitos do rig, também no lab.

Isso permite explorar ritmo de ação, defesa precisa e finalizações em 2D.
Uma nova regra — por exemplo agarrão pareado, teleporte ou escalada — ainda
exige implementação e testes. O contrato atual não promete todos os sistemas
de jogos 3D de grande orçamento. O próximo personagem pode demonstrar onde
generalizar; não é necessário antecipar uma engine universal.

## Augusta: referência e composição

O recorte é inspirado no **Baixo Augusta**: mistura de arquitetura antiga e
moderna, vida noturna e cultura alternativa. A localização de referência é o
trecho entre Paulista e Roosevelt descrito pelo [roteiro oficial da SPTuris](https://cidadedesaopaulo.com/wp-content/uploads/2025/01/ROTEIRO-4-BAIXO-AUGUSTA.pdf).
As [fotografias da rua publicadas pela Veja São Paulo](https://vejasp.abril.com.br/estabelecimento/rua-augusta/)
orientaram toldos verdes, fachadas, relação com a calçada e luz urbana.

São pinturas novas, com fachadas de boteco, bar de música e comércio com mural,
apoios no mesmo plano, piso repetível, juntas de alvenaria e skyline separado.
`mirror_ground_tiles` alterna o espelhamento dos módulos do piso para que as
bordas compartilhem a mesma pintura e não haja um salto vertical de iluminação.
O mapa tem 3600 pixels e módulos posicionados por instância. O estabelecimento
**Limiar**, o cafetão e os seguranças são fictícios. O cenário interpreta os
elementos da região; não é levantamento métrico nem cópia de um endereço real.
Fotos de referência não integram o pacote distribuído.

## Capítulos, carregamento e revisão

`Modo História` oferece Rust e C++ com progresso independente.
`adventure/campaign-v1.json` continua sendo o perfil Rust;
`adventure/cpp-augusta-v1.json` guarda os marcos do novo capítulo. Dados
corrompidos ou de versão futura não são sobrescritos silenciosamente.
Novas rotas só entram no seletor quando implementadas; Python não é anunciada
como capítulo jogável. O registro aceita listas não vazias sem fixar a contagem.

A sessão C++ carrega cinco pacotes de ator e o cenário atual. A medição inicial
foi de **27 texturas / 32,39 MiB RGBA base**, sem contar mipmaps e custos do
driver. O relatório da captura lista todos os caminhos. O menu da luta é
carregado sob demanda e liberado ao entrar na campanha. No capítulo Rust,
separar `SharedAssets` retirou **14 aberturas PNG**, preservando quatro contextos
e cinco poses de pesar com capturas idênticas. [Evidência de carga](evidence/production-pipeline/loading/README.md).

O empacotador segue referências runtime e deixa fontes, receitas e revisões no
checkout de produção. O laboratório pode ser distribuído em perfil opcional
`--lab-binary`. [Empacotamento](../packaging/README.md).

O protocolo inclui revisão visual das poses nos dois sentidos, vídeo em
movimento, teste de ida/volta do pacote portátil, validação de arquivos e
gameplay até o resgate. A [revisão final do laboratório](evidence/production-pipeline/lab/README.md)
inclui 18 segundos a 60 fps; o [capítulo completo](evidence/production-pipeline/augusta/README.md)
inclui combate, resgate e testes dos saves pelo menu.
Os primeiros drafts detectaram cotovelo acima da cabeça,
inversão prematura de vista e granulação; as correções mudaram curvas, registro
e filtragem, preservando a pintura. Há regressões densas de alcance, apoio e
continuidade das articulações. [Diário e resultados](worklogs/cpp-augusta-production.md).

## Referências técnicas

Projetos editáveis de poses, malhas e pesos são uma técnica estabelecida de
animação 2D; os guias oficiais de [malhas](https://esotericsoftware.com/spine-meshes)
e [pesos](https://esotericsoftware.com/spine-weights) do Spine documentam essa
abordagem. O projeto usa seu próprio contrato pequeno; não embute runtime Spine.
Outra alternativa válida seria autorar um modelo 3D e exportar quadros 2D,
como relata o artista de Dead Cells no [relato original da produção](https://www.gamedeveloper.com/production/art-design-deep-dive-using-a-3d-pipeline-for-2d-animation-in-i-dead-cells-i-).
Esse caminho pode ser avaliado se o custo de novas vistas justificar modelagem.
