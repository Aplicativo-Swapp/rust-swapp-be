
# API de Gerenciamento de Dados e Match

Esta API foi desenvolvida em **Rust** utilizando o framework **Actix-web** para manipulação de dados e gerenciamento de "matches" entre usuários. Abaixo está a documentação completa dos endpoints, incluindo detalhes sobre o que cada um recebe e retorna.

---

## **POST** `/inserir`
**Descrição:** Insere dados na tabela `usuario_sub_habilidade`.

### **Requisição:**
- **JSON Corpo:**
  ```json
  {
      "id_users": 1,
      "id_sub_habilidade": 101,
      "descricao": "Descrição da habilidade",
      "valor": 12.34
  }
  ```

### **Resposta:**
- **200 OK:** `Dados inseridos com sucesso`
- **500 Internal Server Error:** `Erro ao inserir dados`

---

## **GET** `/obter/{id_users}`
**Descrição:** Retorna todos os dados da tabela `usuario_sub_habilidade` de um usuário específico.

### **Requisição:**
- **Parâmetro na URL:**
  - `id_users` (integer): ID do usuário.

### **Resposta:**
- **200 OK:** Lista de objetos com os dados do usuário.
  ```json
  [
      {
          "id_users": 1,
          "id_sub_habilidade": 101,
          "descricao": "Descrição da habilidade",
          "valor": 12.34
      }
  ]
  ```
- **500 Internal Server Error:** `Erro ao buscar dados`

---

## **GET** `/obter_tudo`
**Descrição:** Retorna todos os dados da tabela `usuario_sub_habilidade`.

### **Requisição:**
- Sem parâmetros.

### **Resposta:**
- **200 OK:** Lista com todos os objetos.
  ```json
  [
      {
          "id_users": 1,
          "id_sub_habilidade": 101,
          "descricao": "Descrição da habilidade",
          "valor": 12.34
      }
  ]
  ```
- **500 Internal Server Error:** `Erro ao buscar todos os dados`

---

## **DELETE** `/deletar/{id_users}`
**Descrição:** Deleta todos os dados da tabela `usuario_sub_habilidade` relacionados a um usuário específico.

### **Requisição:**
- **Parâmetro na URL:**
  - `id_users` (integer): ID do usuário.

### **Resposta:**
- **200 OK:** `Dados deletados com sucesso`
- **500 Internal Server Error:** `Erro ao deletar dados`

---

## **PUT** `/atualizar`
**Descrição:** Atualiza os dados de uma sub-habilidade de um usuário específico.

### **Requisição:**
- **JSON Corpo:**
  ```json
  {
      "id_users": 1,
      "id_sub_habilidade": 101,
      "descricao": "Nova descrição",
      "valor": 56.78
  }
  ```

### **Resposta:**
- **200 OK:** `Dados atualizados com sucesso`
- **500 Internal Server Error:** `Erro ao atualizar dados`

---

## **POST** `/add_like`
**Descrição:** Adiciona um novo "like" na tabela `teste_match`.

### **Requisição:**
- **JSON Corpo:**
  ```json
  {
      "id_deu_like": 1,
      "id_liked": 2
  }
  ```

### **Resposta:**
- **200 OK:** `Like adicionado com sucesso`
- **500 Internal Server Error:** `Erro ao adicionar like`

---

## **GET** `/buscar_likes/{id}`
**Descrição:** Retorna os IDs de quem deu "like" em um usuário específico.

### **Requisição:**
- **Parâmetro na URL:**
  - `id` (integer): ID do usuário que recebeu o like.

### **Resposta:**
- **200 OK:** Lista de IDs que deram like no usuário.
  ```json
  [1, 2, 3]
  ```
- **500 Internal Server Error:** `Erro ao buscar likes`

---

## **PUT** `/match`
**Descrição:** Atualiza a coluna `match` para `TRUE` em uma linha específica.

### **Requisição:**
- **JSON Corpo:**
  ```json
  {
      "id_deu_like": 1,
      "id_liked": 2
  }
  ```

### **Resposta:**
- **200 OK:** `Match atualizado com sucesso`
- **500 Internal Server Error:** `Erro ao atualizar match`

---

## **GET** `/matches/{id_liked}`
**Descrição:** Retorna todos os "matches" de um usuário específico.

### **Requisição:**
- **Parâmetro na URL:**
  - `id_liked` (integer): ID do usuário.

### **Resposta:**
- **200 OK:** Lista de IDs com quem o usuário possui um match.
  ```json
  [1, 2, 3]
  ```
- **500 Internal Server Error:** `Erro ao buscar matches`