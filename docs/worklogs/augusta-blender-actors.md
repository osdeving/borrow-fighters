# Augusta — humanos e veículos 3D no Blender

## Pedido e limites

Em 12/09/2026 o usuário autorizou baixar e instalar Blender e substituir os
humanos e carros da Augusta por modelos 3D. A cena, cenário, enquadramentos,
sequência, movimentos e identidade visual devem permanecer os mesmos. O objetivo
é corrigir volume, articulação e orientação dos corpos, não redesenhar o capítulo.

## Retomada

- Branch: `feat/augusta-blender-actors`; base limpa `27ca4bb`.
- Leia este diário e inspecione status/log antes de repetir trabalho.
- A versão pintada e sua evidência permanecem como referência comparativa.
- Root coordena commits e integração; não descartar mudanças dos agentes.
- Skills repo-atlas, art-direction e rust-gamedev; README, docs 07/08/38/39,
  ADRs 0001/0003/0021/0032/0033 consultados.

## Trabalho em andamento

- Root: instalação Blender, continuidade da cena, docs, pacote e revisão visual.
- `cpp_arm`: fontes Blender, geometria/rig/identidade dos humanos; piloto C++.
- `augusta_staging`: fontes Blender e modelos dos veículos existentes.
- `augusta_ambient`: carregamento/renderização de GLB nas câmeras existentes.

Modelos usam metros; Blender Z para cima e frente -Y, exportados em glTF com
Y para cima e frente +Z. Pivô no chão. GLB deve conter materiais e animações;
arquivos `.blend` e scripts são fontes de autoria e não dependências do jogo.

## Instalação

Blender 4.5.4 LTS Linux x64 baixado do domínio oficial `download.blender.org`.
Arquivo e checksums em `/tmp/borrow-blender-install`. Instalação local em
`/home/willams/.local/opt/blender-4.5.4-linux-x64`, comando
`/home/willams/.local/bin/blender`; validação SHA-256 antes da extração.

- SHA-256 confirmado: `2e6ef8e99fc36327270429ddc8e7bad2859dd878a5a137d2e0bf0f02f6792505`.
- `blender --version`: 4.5.4 LTS, build `b3efe983cc58`.
- Render headless Cycles CPU concluído com sucesso em
  `/tmp/borrow-blender-install/smoke.png`; instalação funcional.
- Launcher desktop: `/home/willams/.local/share/applications/blender-borrow.desktop`.
- Baseline de continuidade: `docs/evidence/augusta-3d-pilot/baseline.json`
  registra SHA-256 de onze arquivos de cenário, história e câmera.

## Próximos marcos

1. Confirmar execução Blender e render do piloto com identidade revisável.
2. Reproduzir C++ 3D num enquadramento atual, sem alterar o cenário.
3. Completar humanos/carros, poses/contatos e reprodução no gameplay.
4. Rever quadros consecutivos e tomadas equivalentes; validar fontes e pacote.
5. Executar fmt, Clippy e testes pertinentes à integração, registrar resultados
   e commitar etapas coerentes. Não confundir uma renderização bonita isolada
   com o resultado dentro do jogo.
