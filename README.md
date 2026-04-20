🎓 Mahasiswa Management dApp (Soroban)

A simple Web3 application built on Stellar Soroban for managing student (mahasiswa) data on-chain.

🚀 Features
    ➕ Add mahasiswa
    🔍 Get mahasiswa by NIM
    📄 List all mahasiswa
    ✏️ Update mahasiswa
    ❌ Remove mahasiswa
    🔒 Admin-only write operations

⚙️ Setup & Usage
1️⃣ Generate Wallet
    stellar keys generate <alias>

2️⃣ Fund Wallet (Testnet)
    stellar keys fund <alias>
3️⃣ Build Contract
    stellar contract build
4️⃣ Deploy Contract
stellar contract deploy \
    --source-account <alias> \
    --network testnet

Save the Contract ID.

5️⃣ Initialize Contract (IMPORTANT)
    stellar contract invoke
    --id <CONTRACT_ID>
    --source-account <alias>
    --network testnet
    -- init
    --admin <PUBLIC_ADDRESS>

🧪 Contract Functions
➕ Add Mahasiswa
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
🔍 Get Mahasiswa
    stellar contract invoke
    --id <CONTRACT_ID>
    --network testnet
    -- get_student
    --nim 220001
📄 List Mahasiswa
    stellar contract invoke
    --id <CONTRACT_ID>
    --network testnet
    -- list_students
❌ Remove Mahasiswa
    stellar contract invoke
    --id <CONTRACT_ID>
    --source-account <alias>
    --network testnet
    -- remove_student
    --nim 220001

🔒 Authorization
Only the admin can:
add_student
update_student
remove_student
Admin is set during:
init(admin)

