# ADR 0010 — Produção de sprites por ação com exportação explícita

## Status

Aceito para o fluxo de produção. As imagens exportadas continuam candidatas até a revisão visual e funcional.

## Contexto

Os scripts anteriores de Python e C++ distribuem poses de uma folha genérica em vários clips. Esse fluxo foi suficiente para placeholders, mas pode repetir a mesma pose durante preparação, contato e recuperação e compartilhar desenhos entre golpes diferentes. A quantidade de frames no JSON não comprova uma animação própria.

O runtime já aceita PNG + `borrow-fighters.sprite.v1`, com sequência explícita de nomes, duração, pivô e escala. Portanto, produzir folhas menores por ação não exige carregar uma textura por ação nem substituir o contrato existente.

## Decisão

Produzir e revisar cada personagem por ação, começando pelo Rust. Usar uma referência principal estável extraída dos assets existentes, e acrescentar referências de continuidade quando necessário. Manter os assets existentes disponíveis como referência e fallback; os recortes dessas referências não contam como nova arte final.

Separar os arquivos em:

```text
assets/production/<personagem>/
  reference/
  review-plan.json
  production.json
  <acao>/
    source-v1.png
    source.png
    action.json
    prepared.png
    frames/<nome-do-frame>.png
assets/candidates/<personagem>/
  <personagem>-fighter-atlas.png
  <personagem>-fighter.sprite.json
  <personagem>-fighter.provenance.json
```

`source-v1.png` é um exemplo de original preservado; versões e arquivos adicionais podem ser registrados na proveniência. `source.png` é um exemplo de folha com transparência real revisada para extração; o nome efetivo vem da metadata. Cada ação tem sua própria folha de trabalho. O utilitário de exportação recebe os recortes e pivôs explícitos, sem gerar arte, inferir poses ou corrigir pixels.

Fontes em alta resolução podem passar antes por `prepare_reviewed_actions.py`. O `review-plan.json` declara quais ações estão aprovadas para preparação e um fator positivo `scale_to_runtime` por ação. O `action.json` fornece explicitamente folha, recortes, pivôs, ordem, durações e fases. A preparação aplica o mesmo fator a todos os recortes e pivôs daquela ação, com reamostragem Lanczos e arredondamento para pixels inteiros, e escreve `prepared.png` e `production.json` separados das fontes. A escala é uma decisão de revisão, apoiada pela referência de altura corporal registrada na fonte; não é calculada ajustando cada pose ao seu alpha, o que distorceria agachamentos, saltos e membros estendidos. O fator, o caminho da fonte e as notas de revisão ficam na proveniência. Essa etapa recusa metadata `combat`, pois escalá-la implicitamente alteraria o contrato físico.

O contrato de trabalho `borrow-fighters.production.v1` fica fora do schema do jogo:

```json
{
  "schema": "borrow-fighters.production.v1",
  "character": "rust",
  "scale": 0.6667,
  "provenance": {
    "reference": "reference/master-existing.png",
    "notes": "Registrar geração, edição, reaproveitamento e revisão reais."
  },
  "actions": {
    "punch_light": {
      "sheet": "punch_light/source.png",
      "reviewed": true,
      "loop": false,
      "frames": [
        {
          "name": "punch_light_startup_00",
          "source_rect": { "x": 0, "y": 0, "w": 384, "h": 384 },
          "pivot": { "x": 192, "y": 360 },
          "duration_ms": 67,
          "phase": "startup"
        }
      ]
    }
  }
}
```

Os números acima ilustram o formato; não constituem escala, pivô ou timing aprovados para Rust. O caminho `sheet` é relativo ao `production.json`; o pivô é local ao recorte. A ordem em `frames` define a sequência, independentemente da posição dos desenhos na folha. `reviewed: true` é uma declaração de revisão visual dos recortes de cada ação, não uma aprovação automática de arte final ou gameplay.

Comandos de preparação, quando necessária, e exportação:

```bash
python3 tools/art/prepare_reviewed_actions.py assets/production/rust/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/rust/production.json
```

O segundo comando também aceita `--columns` e `--output-dir`. A saída padrão é `assets/candidates/<personagem>/`; ela deve ficar fora do diretório de produção. Depois da preparação opcional, o exportador apenas empacota os pixels recebidos:

- exige PNG com transparência real e recortes com conteúdo visível e pixels transparentes;
- valida recortes dentro da folha, nomes únicos, pivôs inteiros dentro do recorte, duração positiva, escala positiva e `loop` explícito;
- preserva os pixels dos recortes, inclusive bordas semitransparentes, sem redimensionar ou centralizar os personagens;
- dimensiona as células pelo maior recorte e calcula as linhas necessárias, sem limitar a quantidade de frames por uma grade fixa;
- exporta os quadros individuais na pasta da ação e um atlas com seu manifesto para o personagem;
- preserva retângulos de origem, fases opcionais, snapshot do contrato e hashes SHA-256 no arquivo de proveniência;
- não substitui fontes originais pelos arquivos exportados.

`provenance` pode registrar livremente prompts, referência principal, originais, tratamento técnico e revisão. O snapshot preserva também notas de ações e frames. A validação de alpha não reconhece automaticamente textos, xadrez desenhado, resíduos ou anatomia; esses critérios precisam ser inspecionados nas imagens e animações.

`phase` documenta preparação, contato ou recuperação e fica na proveniência; não cria um campo novo no runtime. `frames[].combat` só é incluído quando explicitamente fornecido no contrato de trabalho e válido para o retângulo local. Não são inferidas caixas de colisão ou origens de projéteis. Alterações dessas informações exigem análise do efeito no combate, pois podem substituir os dados de fallback.

O timing de combate a 60 Hz continua sendo a referência para a animação. Ajustar a arte e suas durações não autoriza mudar startup, frames ativos, recovery, dano, alcance ou corpo físico. A associação de clips aos estados e golpes deve ser conferida no código, incluindo entrada, vitória e derrota.

[Os testes de candidatos](../../tests/sprite_candidates.rs) carregam o piloto Rust obrigatório e os demais personagens quando seus candidatos existem. Conferem os 20 clips, a sequência e as durações da fonte e a ausência de metadata de combate na preparação visual. Para os nove golpes próximos, amostram `frame_for_fighter_clip_at` em cada tick convertido por `FrameCount::as_seconds`, ligando o nome retornado à fase registrada no `production.json`. Preparação, cada tick ativo inclusive as duas extremidades e recuperação precisam corresponder ao frame data existente; a duração total admite no máximo 1 ms de diferença. Essa tolerância total não substitui a checagem de fronteiras: 67 ms de preparação atrasariam o contato do Borrow Jab, cujo primeiro tick ativo é 4, aproximadamente 66,667 ms. Os testes verificam a fase declarada; reconhecer o contato no desenho continua exigindo revisão visual.

Exportar um candidato não o ativa automaticamente no jogo. Antes de ampliar a produção para outros personagens, conferir o piloto em movimento, quadro a quadro, nas duas orientações, na escala real e contra as arenas. Usar Sprite Studio e Combat Lab conforme disponíveis, registrando qualquer verificação que o ambiente não permita. Atualizar o contrato de trabalho com ajustes de pivôs e durações feitos no Studio antes de regenerar; o manifesto exportado é um artefato derivado.

### Integração de revisão e preservação de combate

O opt-in de desenvolvimento `BORROW_FIGHTERS_SPRITE_CANDIDATES=1` permite que `GameAssets` carregue `assets/candidates/<key>/<key>-fighter.sprite.json`, com as chaves `rust`, `duke`, `go`, `c`, `python` e `cpp`. Ele exige os 20 clips de `FighterSpriteClip::REQUIRED`: entrada, idle, caminhada, agachamento, salto, defesa em pé e agachada, reação, nove ataques próximos, especial, vitória e derrota. Nomes ausentes são reportados no terminal e mantêm o placeholder completo do personagem; arquivos/texturas inválidos também preservam o fallback. Conjuntos parciais permanecem disponíveis diretamente no Sprite Viewer/Studio. Essa checagem estrutural não avalia identidade, anatomia ou qualidade das animações.

`SpriteAtlasAsset.manifest` e suas texturas descrevem o candidato visual. `SpriteAtlasAsset.combat_manifest` conserva o manifesto baseline de `assets/placeholder/`; é esse dado que `App` entrega ao `World` e que o overlay da luta projeta. Metadata, escala e pivôs do candidato não substituem colisão ou origem do projectile durante a revisão. Uma futura promoção de metadata será uma mudança de gameplay explícita, independente de aceitar a aparência do personagem. O schema PNG + JSON permanece igual.

O mesmo opt-in aceita uma textura de efeito opcional em `assets/candidates/<key>/<key>-projectile.png`, separada do atlas do lutador. Isso permite corrigir recortes inadequados encontrados na revisão visual sem sobrescrever o original. Ausência ou falha de carregamento usam a textura baseline, independentemente da completude do atlas candidato. Specs, origem e colisão continuam originais. O renderer conserva o tamanho do PNG multiplicado por `0.45 * RESOLUTION_SCALE`; mudar as dimensões do canvas muda o envelope desenhado e exige revisão visual, sem ajuste automático à hitbox física.

Também separam-se os relógios de apresentação do sampling de metadata. Reações avançam desde cada impacto; defesa, agachamento e salto avançam desde a entrada no estado; trocar a postura da guarda reinicia seu clip. Ataques e especiais mantêm os relógios existentes, enquanto idle/walk usam tempo global. A metadata conserva os tempos antigos de stun, crouch e jump, e a nova defesa agachada consulta as boxes baseline de `block`. Assim, reproduzir os desenhos previstos não recalibra caixas silenciosamente. O salto precisa ser revisado contra a trajetória física real, pois não usa mais apenas três amostras visuais por velocidade.

O instrumento de revisão também recebe esse baseline: Combat Lab e Move Showcase usam a mesma projeção de origem do especial em tempo zero e as caixas de combate da luta, com fallback quando a metadata está ausente. Isso corrige a divergência anterior do Lab, que sempre mostrava as caixas de MoveSpec e criava projéteis pela origem padrão. Capturas anteriores dessa origem ou das caixas autoradas de Rust exigem recaptura; estimativas de vantagem e dummy continuam baseadas em MoveSpec. O exemplo nativo admite filtro opcional por clips para revisar os contextos afetados sem repetir toda a captura.

Entrada pode viver no atlas principal e tem prioridade sobre a entrada separada. Vitória/derrota começam quando o resultado aparece e continuam enquanto a simulação de combate está congelada; empate usa derrota nos dois lutadores. Esses sprites ancoram no chão mesmo se o KO congelar um corpo no ar, conservando a posição horizontal. Posição, velocidade e boxes físicas não mudam. Efeitos congelados de ataque, dano ou guarda deixam de encobrir essas animações, sem limpar os estados usados pelo debug. O Combat Lab oferece também `--pose spawn`, `--pose defeat` e `--pose crouch_block`, com pause, avanço de um frame e reinício.

Manifestos antigos conservam fallback visual explícito para clips ausentes, incluindo `victory → taunt → idle` e `defeat → hit → idle`. Esses aliases não contam como cobertura para aceitar um candidato completo e não emprestam metadata de golpes diferentes. A regra preexistente que pode limpar `crouching` durante blockstun permanece: corrigi-la mudaria postura física/hurtbox e requer revisão própria no [guia de combate](../12-technical-combat-guide.md).

## Alternativas consideradas

- **Uma folha gigante com todas as ações:** dificulta continuidade, revisão e identificação dos desenhos, e incentiva reutilização inadequada de poses.
- **Um PNG por ação no runtime:** já seria possível pelo suporte multi-atlas, mas não é necessário para manter fontes independentes.
- **Packing e edição visual novos dentro do Studio:** acrescentariam uma ferramenta de produção antes de validar o processo com os utilitários existentes.
- **Reaproveitar automaticamente o contrato de clips de outro personagem:** preserva nomes e quantidade, mas não assegura identidade, timing ou gesto correto.
- **Ativar candidatos parciais e preencher golpes faltantes com idle:** esconderia lacunas de produção na luta; a revisão parcial usa o Viewer/Studio.
- **Usar a metadata do candidato automaticamente:** poderia remover boxes Rust ou deslocar o nascimento de projéteis apenas por trocar arte/escala.

## Consequências

O trabalho artístico fica independente do layout do atlas, com revisão rastreável por ação. O runtime mantém seu schema atual, e todos os frames exportados têm associação explícita. O custo é manter recortes e pivôs revisados e sincronizar ajustes feitos nas ferramentas de inspeção com o contrato de trabalho.

A revisão em luta mantém dois manifestos em memória por atlas: apresentação e baseline de combate. Isso custa pouca metadata duplicada e torna explícita a diferença entre melhorar desenho e alterar balanceamento. Os overlays baseline podem expor desalinhamentos reais com a arte candidata; eles devem ser registrados, não corrigidos automaticamente pelo exportador.

Os utilitários anteriores continuam disponíveis para reproduzir os placeholders. O novo exportador não os executa nem apresenta suas imagens como arte nova.

## Critério de revisão

Revisar esta decisão se o projeto adotar Aseprite como fonte principal, se limites reais de textura exigirem mais páginas ou se a produção demonstrar necessidade de packing visual. Não mudar o formato para múltiplas páginas apenas por organizar a arte em ações.

## Referências locais

- [Pipeline de sprites](../11-sprite-pipeline.md).
- [Escala visual e métricas](../17-visual-scale-and-stage-metrics.md).
- [Sprite Studio](../18-sprite-studio.md).
- [Suporte existente a múltiplos atlas](0009-multi-image-sprite-manifests.md).
- [Exportador de frames revisados](../../tools/art/build_reviewed_sprite_atlas.py).
- [Preparação de fontes em escala de runtime](../../tools/art/prepare_reviewed_actions.py).
