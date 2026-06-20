# 03 — Backlog Inicial

## Legenda de t-shirt sizing

- **XS**: muito pequeno.
- **S**: pequeno.
- **M**: médio.
- **L**: grande.
- **XL**: muito grande; evitar no protótipo.
- **?**: precisa de investigação.

## Prototype 0.1 — Two Boxes Fighting

Status do primeiro greybox:

- Criado em `feature/greybox-vertical-slice`.
- Cobre o núcleo mínimo jogável em debug visual.
- Ainda não fecha Prototype 0.1 como release; serve para playtest e discussão.

| Item | Tamanho | Prioridade | Observação |
|---|---:|---:|---|
| Criar projeto Rust | S | Alta | Cargo project básico |
| Configurar Raylib/Raylib-rs | M | Alta | Validar build no Windows |
| Criar janela e loop principal | S | Alta | Primeiro teste visual |
| Desenhar arena simples | S | Alta | Chão e limites |
| Criar entidade Player | M | Alta | Posição, velocidade, vida |
| Implementar input local | M | Alta | Teclado inicialmente |
| Movimento horizontal | M | Alta | Esquerda/direita |
| Gravidade e pulo | M | Alta | Física simples |
| Direção/facing do personagem | S | Média | Olhar para adversário |
| Ataque básico | M | Alta | Estado de ataque |
| Hurtbox | M | Alta | Área vulnerável |
| Hitbox | L | Alta | Área ofensiva temporária |
| Detecção de colisão hitbox/hurtbox | L | Alta | Coração do combate |
| Aplicação de dano | M | Alta | Dano fixo |
| Barra de vida | S | Alta | UI simples |
| Condição de vitória | S | Alta | Vida <= 0 |
| Reinício da partida | S | Média | Tecla R |
| Debug draw | M | Média | Mostrar caixas |
| Knockback simples | M | Baixa | Pode ficar para 0.2 |
| Animação placeholder | M | Baixa | Pode ser só troca de cor/pose |

## Fora do backlog inicial

| Item | Motivo |
|---|---|
| Online multiplayer | Complexidade XL |
| Sistema de combo | Depende do combate básico |
| Vários personagens | Depende da abstração mínima |
| Arte final | Depende do vertical slice |
| Trilha sonora | Não prova gameplay |
| Menu completo | Não prova gameplay |
| IA avançada | Pode distrair do combate local |
