# Considere as Consequências! (1930)

> **Adaptação interativa em Rust do primeiro livro-jogo de escolhas da história.**

![Rust](https://img.shields.io/badge/Rust-2024%20Edition-orange?logo=rust)
![Platforms](https://img.shields.io/badge/Plataformas-Windows%20%7C%20macOS%20%7C%20Linux-blue)
![License](https://img.shields.io/badge/Licen%C3%A7a-MIT-green)
![Hobby Project](https://img.shields.io/badge/Projeto-Hobby%20Sem%20Fins%20Lucrativos-purple)
![AI Assisted](https://img.shields.io/badge/Produ%C3%A7%C3%A3o-Assistida%20por%20IA-teal)

---

## Sobre o Projeto

**Considere as Consequências!** é uma adaptação interativa nativa em **Rust** (`eframe` / `egui`) da obra clássica *Consider the Consequences!* (1930), escrita por **Doris Webster** e **Mary Alden Hopkins** — amplamente reconhecida como o primeiro livro de ficção interativa ramificada (*escolha sua própria história*) publicado no mundo.

> **Aviso de Transparência:**
> - Este é um **projeto pessoal de hobby, totalmente gratuito e sem fins lucrativos**, criado para fins de estudo, experimentação em Rust e preservação interativa de uma obra histórica de 1930.
> - **Uso de Inteligência Artificial:** Este projeto foi desenvolvido com **auxílio de Inteligência Artificial (IA)** na estruturação do código em Rust, interface gráfica, testes automatizados, documentação e geração das ilustrações em estilo bico de pena dos anos 1930.

---

## Principais Recursos

- **História Completa em Dois Idiomas (Português & Inglês Original de 1930):**
  - Troca instantânea de idioma (`PT-BR` / `EN 1930`) na barra superior ou pela tecla `L` — incluindo texto original de 1930 e toda a interface.
  - **86 cenas e 43 finais:**
    - **Livro I — Helen Rogers** (36 cenas, 17 desfechos)
    - **Livro II — Jed Harringdale** (19 cenas, 10 desfechos)
    - **Livro III — Saunders** (31 cenas, 16 desfechos)
  - Inclui o **Prefácio original de 1930** em português e inglês acessível diretamente no menu inicial.
- **Visual Moderno com Estética Art Déco (1930):**
  - **3 temas com transição suave de cores:** *Noir Art Déco*, *Pergaminho* e *Sépia 1930*.
  - **6 ilustrações a bico de pena** embutidas diretamente no executável (retratos dos protagonistas e vinhetas de cena).
  - **Ícones vetoriais Art Déco** desenhados diretamente no canvas (sem dependência de fontes de emoji do sistema).
  - **Efeitos e animações fluidas:** partículas douradas ambiente, explosão de faíscas e ondas de choque ao clicar nas opções, entrada escalonada de parágrafos e celebrações personalizadas por tom de final (*Triunfo*, *Sereno*, *Agridoce*, *Irônico*, *Tragédia*).
- **Decisões Definitivas:**
  - Assim como no espírito do título, cada escolha é definitiva até a conclusão de um desfecho. Após terminar uma jornada, é possível iniciar novamente e explorar novos ramos.
- **Bússola de Caráter & Trilha de Escolhas:**
  - Painel lateral que acompanha em tempo real os 6 traços moldados pelas suas escolhas (*Coração*, *Razão*, *Ousadia*, *Dever*, *Ambição* e *Integridade*) e registra a crônica das decisões tomadas.
- **Árvore Progressiva de Caminhos e Finais:**
  - Diagrama em árvore com curvas Bézier que cresce organicamente conforme você percorre a narrativa — **sem spoilers**, revelando apenas os nós e finais já desbloqueados.
- **Salvamento Automático e Reset:**
  - Progresso salvo automaticamente em `savegame.json` ao lado do executável, com botão **Resetar Jogo** (com confirmação) no menu inicial.
- **Binário 100% Autocontido:**
  - Fontes (*Noto Serif* e *Noto Sans*), texto da história e ilustrações são embutidos no executável final.

---

## Download e Execução (Windows, macOS e Linux)

> **Não precisa instalar Rust, Cargo ou qualquer programação para jogar!**
> Basta baixar o executável pronto para o seu sistema operacional na aba **[Releases](https://github.com/marcoantoniopereirajr-star/considere-as-consequencias/releases)**, extrair e abrir com duplo clique.

| Sistema Operacional | Arquivo na Release | Como Executar |
| :--- | :--- | :--- |
| **Windows (64-bit)** | `considere-as-consequencias-windows-x86_64.zip` | Extraia o `.zip` e clique duas vezes em `considere-as-consequencias.exe`. |
| **macOS (Apple Silicon M1/M2/M3/M4)** | `considere-as-consequencias-macos-aarch64.tar.gz` | Extraia o `.tar.gz` e execute o binário `considere-as-consequencias`. |
| **macOS (Intel)** | `considere-as-consequencias-macos-x86_64.tar.gz` | Extraia o `.tar.gz` e execute o binário `considere-as-consequencias`. |
| **Linux (64-bit)** | `considere-as-consequencias-linux-x86_64.tar.gz` | Extraia o `.tar.gz` e execute `./considere-as-consequencias`. |

> **Nota para macOS / Windows:** Como este é um projeto de hobby sem certificado comercial pago de assinatura de código, o Windows SmartScreen ou o Gatekeeper do macOS podem exibir um aviso na primeira execução. No Windows, clique em *"Mais informações" → "Executar assim mesmo"*; no macOS, clique com o botão direito → *"Abrir"*.

---

## Controles e Atalhos de Teclado

| Tecla / Ação | Função |
| :--- | :--- |
| `1` – `9` | Seleciona diretamente a opção correspondente na cena |
| `T` | Alterna suavemente entre os 3 temas visuais |
| `L` | Alterna instantaneamente entre **Português (PT-BR)** e **Inglês Original (1930)** |
| `Esc` | Volta ao Menu Inicial (ou fecha a confirmação de reset) |
| `A-` / `A+` | Ajusta o tamanho da fonte de leitura |

---

## Compilando a partir do Código-Fonte

### Pré-requisitos
- [Rust e Cargo](https://rustup.rs/) instalados (versão estável recente, Edition 2024).
- No **Linux**, bibliotecas de desenvolvimento OpenGL/X11/Wayland (normalmente já presentes em ambientes desktop).

### Comandos

```bash
# Clonar o repositório
git clone https://github.com/marcoantoniopereirajr-star/considere-as-consequencias.git
cd considere-as-consequencias

# Rodar os testes automatizados (integridade do grafo, 86 cenas, 43 finais e artes)
cargo test

# Executar em modo otimizado (release)
cargo run --release

# Gerar apenas o executável final em target/release/
cargo build --release
```

---

## Arquitetura do Projeto

```text
considere-as-consequencias/
├── .github/workflows/
│   ├── ci.yml                  # Testes automáticos em Linux, Windows e macOS
│   └── release.yml             # Build e publicação automática de binários multiplataforma
├── assets/
│   ├── story.json              # Estrutura completa das 86 cenas e 43 finais
│   ├── icon.svg                # Ícone vetorial Art Déco
│   ├── art/                    # 6 ilustrações a bico de pena (embutidas no binário)
│   └── fonts/                  # Fontes Noto Serif e Noto Sans (embutidas no binário)
├── src/
│   ├── main.rs                 # Ponto de entrada eframe, carregamento de fontes e janela
│   ├── app.rs                  # Telas (Menu, Leitura, Finais), estado e transições
│   ├── story.rs                # Modelo de dados, SaveState e testes de integridade
│   ├── tree.rs                 # Diagrama em árvore progressivo com curvas Bézier
│   ├── art.rs                  # Galeria de ilustrações a bico de pena e enquadramento
│   ├── icons.rs                # Ícones vetoriais Art Déco desenhados via egui::Painter
│   ├── fx.rs                   # Partículas, ondas de choque e ornamentos geométricos
│   ├── theme.rs                # Paletas de cores e interpolação suave entre temas
│   └── widgets.rs              # Cartões de escolha animados, botões e cards de campanha
├── Cargo.toml
├── LICENSE
└── README.md
```

---

## Créditos e Licença

- **Obra original (1930):** *Consider the Consequences!*, de Doris Webster e Mary Alden Hopkins.
- **Tecnologias:** [Rust](https://www.rust-lang.org/), [egui / eframe](https://github.com/emilk/egui), [serde](https://serde.rs/), [image](https://github.com/image-rs/image).
- **Licença do código:** Distribuído sob a licença **MIT** — veja [LICENSE](LICENSE) para detalhes.
# considere-as-consequencias
# considere-as-consequencias
# considere-as-consequencias
# considere-as-consequencias
# considere-as-consequencias
