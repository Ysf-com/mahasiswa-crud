# 🎓 Mahasiswa Management dApp (Soroban)

A simple Web3 application built on Stellar Soroban for managing student (mahasiswa) data on-chain.

---

## 🚀 Features

- ➕ Add mahasiswa  
- 🔍 Get mahasiswa by NIM  
- 📄 List all mahasiswa  
- ✏️ Update mahasiswa  
- ❌ Remove mahasiswa  
- 🔒 Admin-only write operations  

---

## 🧠 Tech Stack

- Smart Contract: Rust + Soroban SDK  
- Blockchain: Stellar Testnet  
- CLI: Stellar CLI (Soroban Studio Terminal)  
- (Optional) Frontend: React + Freighter  

---

## 📂 Project Structure

```text
.
├── contracts
│   └── mahasiswa-management
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

---

## ⚙️ Setup & Usage

### 1️⃣ Generate Wallet
```bash
stellar keys generate <alias>
```

Example:
```bash
stellar keys generate yusufhamzah
```

---

### 2️⃣ Fund Wallet (Testnet)
```bash
stellar keys fund <alias>
```

---

### 3️⃣ Build Contract
```bash
stellar contract build
```

---

### 4️⃣ Deploy Contract
```bash
stellar contract deploy
  --source-account <alias>
  --network testnet
```

Save the **Contract ID**.

---

### 5️⃣ Initialize Contract (IMPORTANT)
```bash
stellar contract invoke
  --id <CONTRACT_ID>
  --source-account <alias>
  --network testnet
  -- init
  --admin <PUBLIC_ADDRESS>
```

---

## 🧪 Contract Functions

### ➕ Add Mahasiswa
```bash
stellar contract invoke
  --id <CONTRACT_ID>
  --source-account <alias>
  --network testnet
  -- add_student
  --nim 220001
  --nama Budi
  --jurusan Informatika
  --angkatan 2022
  --aktif true
```

---

### 🔍 Get Mahasiswa
```bash
stellar contract invoke
  --id <CONTRACT_ID>
  --network testnet
  -- get_student
  --nim 220001
```

---

### 📄 List Mahasiswa
```bash
stellar contract invoke
  --id <CONTRACT_ID>
  --network testnet
  -- list_students
```

---

### ❌ Remove Mahasiswa
```bash
stellar contract invoke
  --id <CONTRACT_ID>
  --source-account <alias>
  --network testnet
  -- remove_student
  --nim 220001
```

---

## 🔒 Authorization

Only the admin can:
- add_student  
- update_student  
- remove_student  

Admin is set during:
```bash
init(admin)
```

---

## 🌐 Future Improvements

- Web frontend (React + Freighter)
- Role-based access control
- GPA & course tracking
- Dashboard UI
- Pagination for large datasets

---

## 🧩 Notes

- Runs on Stellar Testnet (no real money)
- CLI wallet is used for deployment & admin actions
- Freighter is optional for frontend integration

---

## 🎯 Goal

This project demonstrates how to build a real Web3 CRUD application using Soroban smart contracts.
