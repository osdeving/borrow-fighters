# Adereços e fuga do ciclista

Assets isolados da aventura para a rua modular e a evacuação da EP, gerados
em 10 de setembro de 2026 com o **image_gen embutido**. São candidatos para
o protótipo, com identidade e proporções conferidas contra o ciclista de
[`../street-life.png`](../street-life.png) e o bairro já existente.
Não foram usados samples, fotografias externas ou marcas de terceiros.
Aplicam-se as licenças do repositório.

## Arquivos selecionados

| Atlas | Conteúdo | Metadados | Prompt final |
|---|---|---|---|
| [props.png](props.png) | Fachada, abrigo de ônibus, bicicleta caída e canto de engradados/cadeira/lixeira | [props.json](props.json) | [props.txt](prompts/props.txt) |
| [cyclist-escape.png](cyclist-escape.png) | Frenagem, desmontagem, soltura da bicicleta, bicicleta vazia e quatro poses correndo | [cyclist-escape.json](cyclist-escape.json) | [cyclist-escape-clean-cells.txt](prompts/cyclist-escape-clean-cells.txt) |

Cada atlas mede 1536 × 1024 pixels e possui alpha verdadeiro. O atlas de
adereços contém **858.222 pixels com alpha zero**; o de ciclista contém
**1.168.122**. Os demais pixels usam alpha intermediário, com máximo 254,
incluindo bordas suavizadas. Os PNGs foram copiados sem alterar seus pixels;
SHA-256, contagens de alpha e janelas de medição constam nos JSONs.

## Adereços

Os IDs, na ordem de `rects`, são `storefront-casa-nossa`, `bus-stop-shelter`,
`abandoned-bicycle-lying` e `grocery-corner-props`. São peças sobrepostas,
sem pessoas, cenário incorporado ou palavras pintadas. Os recortes seguem
as silhuetas reais, com duas linhas e larguras irregulares, em vez de assumir
quadrantes iguais. `pivots` usa pixels relativos ao canto superior esquerdo
de cada recorte e indica o apoio inferior central.

A fachada tem painel vazio para **Bar e Mercearia Casa Nossa**; o abrigo tem
plaquinha vazia. `text_areas` fornece os retângulos internos em coordenadas
do PNG e do recorte. Em uma fachada com largura 300, o painel interno mede
aproximadamente 197 × 20 pixels. No abrigo com largura 220, sua plaquinha
mede aproximadamente 10 × 21 pixels: comporta um símbolo; uma legenda maior
pode ser desenhada separadamente pelo runtime. O texto continua editável.

## Ciclista

O adulto mantém capacete cinza, pele morena, barba curta, camiseta
vermelho-tijolo, bermuda marinho e tênis azul/branco do ciclista anterior.
Os primeiros quatro recortes têm bicicleta apontando para a **direita**:

1. `cyclist-brake-right`: corpo recua ao frear, com um pé saindo do pedal.
2. `cyclist-dismount-right`: ambos os pés descem enquanto segura o guidão.
3. `cyclist-release-right`: mãos livres e primeiro passo para a esquerda.
4. `bicycle-upright-right`: bicicleta vazia para iniciar a queda.

Os recortes `cyclist-run-left-0` a `cyclist-run-left-3` representam corrida
para a **esquerda**, alternando passadas extensas, passagem das pernas e
braços. Não possuem bicicleta. A bicicleta caída está no atlas de adereços.

Para uma altura próxima dos 100 pixels do ciclista anterior, usar escala
uniforme **100 / 370**, inclusive na bicicleta vazia; as poses de corrida
ficam com recorte de aproximadamente 98 pixels. Escalar cada frame para a
mesma altura total faria a bicicleta vazia crescer e mudaria as proporções.

Os `pivots` dos frames 0–3 ancoram o ponto entre as rodas no mesmo chão.
`visual_feet_pivots` também registra o apoio dos pés, que podem ficar cerca
de três pixels de tela abaixo das rodas durante a desmontagem. Nos frames
4–7, os pivôs seguem o quadril horizontalmente e compartilham o mesmo chão
medido, sem recentralizar cada passada por sua largura. A margem inferior
dos recortes de corrida inclui esse apoio. O renderer deve considerar
`authored_facing` ao espelhar os recortes.

## Seleção e verificação

O primeiro [prompt do ciclista](prompts/cyclist-escape.txt) produziu alpha
válido, mas duas poses ficaram próximas demais para recortes limpos. Uma
[tentativa de ajuste com referência](prompts/cyclist-escape-spacing.txt)
produziu fundo xadrez RGB e foi rejeitada. A geração textual final separou
as oito poses; nenhuma das bordas verticais de suas células contém alpha
acima do limiar 3. As tentativas descartadas permanecem somente no diretório
local de imagens geradas, sem entrar no runtime.

Foram conferidos RGBA, dimensões, alpha vazio, recortes dentro do PNG,
pivôs dentro dos recortes, placas sem texto, número de poses e direção.
Os arquivos não receberam retoque, remoção de fundo ou corte de pixels por
script: os scripts de inspeção apenas mediram a imagem e escreveram JSON.
A escala final, a continuidade da desmontagem e as sobreposições devem ser
avaliadas também na execução real da cena.
