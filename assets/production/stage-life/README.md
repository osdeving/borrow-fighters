# Vida nos cenários

Assets candidatos produzidos em 8 de setembro de 2026 para os figurantes de
fundo. O pedido do usuário forneceu duas referências: um cachorro caramelo
brasileiro de pelo curto e o gesto do meme “Já acabou, Jéssica?”. As imagens
originais orientaram aparência, roupa e gesto; os PNGs abaixo são novas
ilustrações produzidas com a ferramenta integrada `imagegen`.

O [conjunto completo de prompts](prompts.md) registra geração, extração de fundo e
limpeza da arena. O jogo usa a versão final com alpha da figurante.

| Arquivo | Dimensões | Uso |
| --- | --- | --- |
| [caramelo-run.png](caramelo-run.png) | 2172 × 724, RGBA | Quatro poses de corrida, vistas de perfil para a direita. |
| [jessica-gesture.png](jessica-gesture.png) | 1536 × 1024, RGBA | Três poses da figurante: repouso, abertura e gesto. |
| [arena-sirius-clean.png](arena-sirius-clean.png) | 1672 × 941, RGB | Sirius com a calçada recomposta e o cachorro estático removido. |

A limpeza de Sirius usou como referência a
[arena anterior](../../placeholder/arena-sirius.png), preservada no repositório.
O cachorro móvel substitui o cachorro felpudo que estava embutido no fundo.
A figurante usa camiseta vermelha com faixa diagonal branca e estrela azul,
calça preta e tênis brancos; aparece apenas na arena de São Paulo.

Os arquivos foram copiados diretamente das saídas da geração, sem remoção de
fundo, recorte, recompressão ou repintura posterior. Os dois atlas já contêm
alpha: aproximadamente 83,6% dos pixels do cachorro e 67,9% da figurante são
totalmente transparentes. Um visualizador que ignore alpha pode exibir cores
de fundo presentes nos canais RGB. O runtime usa o alpha original e filtra
as texturas com mipmaps e interpolação trilinear.

## Recortes e apoio

As poses **não formam células iguais**. O renderer usa retângulos explícitos
medidos nos componentes conexos de alpha, com margem para filtragem. Nenhum
arquivo de imagem precisa ser alterado para separar os frames.

`source` é `[x, y, largura, altura]`; `pivot` é local ao recorte. Todas as poses
do mesmo ator usam a mesma escala. O cachorro conserva a elevação da primeira
pose, enquanto os pés da figurante permanecem no mesmo apoio.

| Ator / frame | source | pivot |
| --- | --- | --- |
| Caramelo 0 | `[12, 190, 607, 312]` | `[320, 350]` |
| Caramelo 1 | `[622, 197, 502, 345]` | `[246, 343]` |
| Caramelo 2 | `[1180, 206, 460, 330]` | `[212, 334]` |
| Caramelo 3 | `[1659, 204, 490, 337]` | `[225, 336]` |
| Jéssica 0 | `[58, 62, 396, 899]` | `[198, 896]` |
| Jéssica 1 | `[540, 65, 455, 891]` | `[229, 893]` |
| Jéssica 2 | `[1010, 67, 526, 890]` | `[310, 891]` |

O [renderer](../../../src/engine/render/stage_life.rs) desenha o cachorro a
12 frames por segundo, atravessando por 6,8 segundos a cada 26 segundos e
alternando direção. Cada arena tem atraso e linha de chão próprios. A figurante
faz um gesto de 1,68 segundo a cada 12 segundos.

Todos os atores ficam atrás dos lutadores e somem durante cinematográficas.
Eles não têm hitbox, hurtbox, colisão ou efeito sobre o resultado da luta.
`Options → Vida nos cenarios` controla atores e detalhes decorativos:

| Arena | Detalhe |
| --- | --- |
| Sirius / Campinas | Monitor `NAZARE.exe`, com cursor de cálculo. |
| Fortaleza | Faixa `BORA, BILL!` como sinal de chamada. |
| São Paulo | Figurante e pequeno adesivo `JA ACABOU, JESSICA?`. |
| BioTIC / Brasília | Aviso de papel `E VERDADE ESSE BILETE`. |
| Porto Digital / Recife | Pequena placa `AMOSTRADINHO`. |
| Vale do Pinhão / Curitiba | Aviso no terminal `HOJE NAO, FARO`. |

As inscrições são desenhadas com a fonte do jogo e ficam presas a monitores,
placas ou avisos existentes. As referências funcionam como detalhes de cultura
cotidiana, com escala e contraste inferiores aos lutadores e ao HUD.

## Verificação

Os testes de `engine::render::stage_life` verificam intervalo entre travessias,
troca de direção, faixa de profundidade, índices de animação e os recortes
contra as dimensões dos PNGs distribuídos. Na revisão visual, conferir São
Paulo próximo de 9,5 segundos de relógio visual: cachorro e gesto aparecem
juntos, sem disputar a área central da luta.

As [cinco arenas adicionais](../../../docs/evidence/presentation-polish/stages/)
foram capturadas pelo renderer real com o cachorro próximo ao meio da pista.
Sirius confirma a retirada do cachorro estático; os avisos das outras arenas
foram conferidos sobre os monitores, grades e pedestais do fundo.

```bash
cargo test --lib engine::render::stage_life
cargo run --example capture_stage_review -- docs/evidence/presentation-polish/stages
cargo run --example capture_presentation_review -- docs/evidence/presentation-polish
```
