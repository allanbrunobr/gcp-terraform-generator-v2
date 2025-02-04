# GCP Terraform Generator V2

Uma ferramenta moderna para gerar automaticamente configurações Terraform para recursos do Google Cloud Platform (GCP) através de uma interface web intuitiva.

## 🚀 Funcionalidades

- Interface web moderna e responsiva construída com Next.js 13+ e Tailwind CSS
- Backend robusto em Rust para alta performance
- Geração automática de código Terraform para recursos GCP:
  - Cloud Functions
  - Cloud Run
  - BigQuery Datasets
  - Pub/Sub Topics e Subscriptions
  - Storage Buckets
  - VPC Networks
- Preview do código Terraform gerado
- Suporte a credenciais GCP via Service Account
- Validação de configurações em tempo real

## 🛠️ Tecnologias Utilizadas

### Frontend
- Next.js 13+
- TypeScript
- Tailwind CSS
- Headless UI
- React Hooks

### Backend
- Rust
- Axum (Framework Web)
- Serde (Serialização/Deserialização)
- Tokio (Runtime assíncrono)

## 📋 Pré-requisitos

- Node.js 18+ 
- Rust 1.70+
- Cargo (Rust package manager)
- Credenciais GCP (Service Account com permissões adequadas)

## 🔧 Instalação

### Frontend

```bash
cd frontend
npm install
npm run dev
```

O frontend estará disponível em `http://localhost:3000`

### Backend

```bash
cd backend
cargo build
cargo run
```

O backend estará disponível em `http://localhost:8000`

## 🔑 Configuração das Credenciais GCP

1. Acesse o Console GCP
2. Navegue até IAM & Admin > Service Accounts
3. Crie uma nova Service Account ou selecione uma existente
4. Gere uma nova chave no formato JSON
5. Use o arquivo de credenciais ao carregar a aplicação

## 📦 Estrutura do Projeto

```
.
├── frontend/                # Aplicação Next.js
│   ├── src/
│   │   ├── app/            # Rotas e layouts
│   │   ├── components/     # Componentes React
│   │   ├── types/         # Definições de tipos TypeScript
│   │   └── terraform/     # Arquivos relacionados ao Terraform
│   └── public/            # Arquivos estáticos
│
└── backend/               # Servidor Rust
    ├── src/
    │   ├── handlers/     # Handlers HTTP
    │   ├── models/       # Modelos de dados
    │   ├── services/     # Lógica de negócio
    │   └── utils/        # Utilitários
    └── terraform_files/  # Templates Terraform
```

## 🤝 Contribuindo

1. Faça um Fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📝 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## ✨ Agradecimentos

- Google Cloud Platform
- Comunidade Rust
- Comunidade Next.js
