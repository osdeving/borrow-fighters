# Importação reproduzível de peças da aventura

`import_adventure_actor.py` executa recortes explícitos e extração de alpha
com ImageMagick. Apesar do nome, a receita serve também para objetos, inimigos
e cenários. Não desenha, redimensiona, recorta automaticamente pela silhueta
nem infere animação. Requer Python 3.8+ e ImageMagick 6 (`convert`) ou 7
(`magick`), sem Pillow/NumPy.

```sh
python3 tools/art/import_adventure_actor.py --recipe assets/adventure/actors/cpp/import.json
python3 tools/art/import_adventure_actor.py --recipe assets/adventure/actors/cpp/import.json --check
python3 -m unittest discover -s tools/art -p test_import_adventure_actor.py -v
```

`--check` exporta novamente em armazenamento temporário e compara os bytes
dos PNGs e do manifesto; não altera fontes, exports, manifesto ou seus mtimes.
Uma diferença exige revisar e executar uma importação intencional. A receita
e o manifesto da C++ estão em [actors/cpp](../../assets/adventure/actors/cpp/import.json).

## Receita mínima

```json
{
  "version": 1,
  "metadata": "import-manifest.json",
  "keying": "magenta",
  "sources": {
    "parts": { "path": "source/parts.png" }
  },
  "exports": [
    {
      "id": "arm",
      "source": "parts",
      "crop": [20, 40, 120, 240],
      "output": "sprites/arm.png",
      "landmarks": { "shoulder": [80, 60], "elbow": [82, 250] }
    }
  ]
}
```

Todos os caminhos ficam dentro da pasta da receita, inclusive após resolver
symlinks. Cada fonte pode declarar `sha256` como trava opcional: uma nova
pintura exige atualizar conscientemente esse valor. Sem trava, uma importação
aceita a nova fonte e sela o novo hash; `--check` continua detectando mudanças.

`crop` é `[x, y, largura, altura]` em pixels inteiros da fonte, sem resize ou
trim. `landmarks` contém pontos em coordenadas da **fonte**, dentro desse crop.
O manifesto preserva os pontos e registra `landmarks_local = fonte − origem
do crop`, dimensões, limites da região visível, contagens de alpha, SHA256 de
cada fonte/export, hash da receita e versão do ImageMagick. Essa versão faz
parte do contrato de reprodução: uma mudança de ferramenta deve ser revista,
mesmo se a aparência não mudar.

## Alpha e integridade

`keying: "none"` conserva alpha e cores existentes, inclusive roxo legítimo.
`"magenta"` destina-se a arte opaca sem pigmento roxo sobre fundo FF00FF.
O importador estima a contribuição do fundo pelo excesso de vermelho/azul
sobre verde, remove o campo saturado e descontamina as bordas misturadas.
Preserva alpha parcial do contorno e não aplica erosão morfológica. Esse
método não separa pigmento roxo verdadeiro de chroma: nesses casos, usar uma
fonte com alpha e `"none"`.

`"magenta-flat"` restringe essa operação ao fundo saturado e a uma vizinhança
máxima de dois pixels. O ImageMagick expande somente essa máscara de seleção;
não aplica erosão/expansão ao alpha exportado. Isso limpa a mistura da borda
sem propagar por falhas no contorno até a roupa roxa inteira. O interior RGB e
alpha permanece intacto. Esse é o modo usado na Julia e nas fachadas após
inspeção da fonte. Pigmento igual ao chroma ainda é ambíguo; conferir a folha
sobre outra cor antes de aceitar uma fonte. A validação de resíduo desse modo
verifica o campo magenta saturado, permitindo roxos legítimos.

A validação rejeita crop fora da imagem, landmark inválido, caminho que sai
da pasta, saída repetida/reservada, trava SHA divergente, sprite vazio e
magenta visível residual. Todas as peças são geradas e validadas antes da
primeira promoção; uma fonte ou receita alterada durante o export aborta a
operação. Cada arquivo é promovido atomicamente, com o manifesto por último.
Uma falha de disco durante a promoção exige repetir a importação; não existe
transação de diretório ou histórico automático de revisões.

## Recortes da C++

Os quatro torsos usam intervalos explícitos: dividir a folha em quatro células
de 384 pixels cortaria cabelo que ultrapassa duas células. As pernas chegam
até y=561; a segunda linha começa em y=576. Nenhum export é aparado novamente
depois desses crops. Os landmarks iniciais são pontos de registro autorados
para montar o rig e podem ser corrigidos isoladamente na receita; eles não
atestam postura ou movimento final.
