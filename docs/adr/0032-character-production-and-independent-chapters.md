# ADR 0032 — Autoria de personagens e capítulos independentes

Status: aceita e implementada no piloto C++/Augusta.

## Contexto

O usuário autorizou um laboratório fora da campanha, usando C++ como piloto,
e um capítulo independente na Augusta. As correções recorrentes da corrida de
Rust mostraram um custo real de autoria: rever o mesmo movimento exigia navegar
pela história e os artefatos não ofereciam um contrato comum de personagem.
O capítulo Rust também carregava imagens exclusivas do prólogo.

## Decisão

- Preservar Rust/Raylib e o isolamento aventura/luta da ADR 0021. O piloto não
  modifica o rig de Rust nem migra as regras antigas para uma arquitetura nova.
- Separar personagem, rig, clips, golpes, mapa e texto em artefatos JSON
  versionados. Fontes de arte, receita de importação e proveniência ficam
  preservadas; arquivos exportados são reproduzíveis a partir dessas fontes.
- Compartilhar avaliador de animação, simulação e renderer entre o laboratório
  independente e o capítulo. A prévia não simula uma versão diferente do jogo.
- Interpolar poses e transições. Giros usam vistas desenhadas e troca de
  attachments; não prometem câmera 3D ou qualquer ângulo de um único desenho.
  Golpes possuem janelas de contato explícitas, independentes da arte.
- Usar coleção de atores com identidades estáveis, estados e eventos explícitos.
  A necessidade atual não exige ECS, plugins ou uma linguagem de scripting.
- Manter capítulos e saves por protagonista. O perfil Rust existente permanece
  compatível; só capítulos implementados aparecem como jogáveis.
- Carregar recursos por sessão/pacote, validar referências antes de trocar um
  pacote e medir contagem e dimensões das texturas. Não antecipar streaming.
- Representar o Baixo Augusta com módulos compatíveis em apoio/perspectiva e
  referências documentadas. O local e os envolvidos na coerção são fictícios.

## Consequências e critérios de revisão

Adicionar uma aparência ou ajustar timing não deve exigir alterações no
renderer. Adicionar uma nova mecânica ainda pode exigir código e testes.
O contrato é deliberadamente menor que um editor geral, com validação, revisão
de poses nos dois sentidos, teste interativo e exportação de evidências.

O piloto deve comprovar movimento frequente, partida/parada/reversão, giro,
contatos, parry, projétil, derrota/retry, eventos e resgate no capítulo integrado.
Aprovar JSON não aprova automaticamente a qualidade visual: capturas nativas
e reprodução em movimento fazem parte da aceitação.

Reavaliar outra ferramenta de autoria ou engine com medições de retrabalho,
qualidade e custo de integração; o piloto não torna inevitável uma migração.

Referências: [arquitetura](../08-code-architecture.md),
[isolamento](0021-isolated-adventure-experiment.md),
[revisão da corrida](../37-run-cycle-rebuild.md).
