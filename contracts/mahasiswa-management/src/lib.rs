#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct Mahasiswa {
    pub nim: String,
    pub nama: String,
    pub jurusan: String,
    pub angkatan: u32,
    pub aktif: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Student(String), // key by NIM
    StudentList,
}

#[contract]
pub struct MahasiswaManagementContract;

fn require_admin(env: &Env) -> Address {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap();
    admin.require_auth();
    admin
}

fn get_student_list(env: &Env) -> Vec<String> {
    env.storage()
        .instance()
        .get(&DataKey::StudentList)
        .unwrap_or(Vec::new(env))
}

fn save_student_list(env: &Env, list: &Vec<String>) {
    env.storage().instance().set(&DataKey::StudentList, list);
}

#[contractimpl]
impl MahasiswaManagementContract {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::StudentList, &Vec::<String>::new(&env));
    }

    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap()
    }

    pub fn add_student(
        env: Env,
        nim: String,
        nama: String,
        jurusan: String,
        angkatan: u32,
        aktif: bool,
    ) {
        require_admin(&env);

        let key = DataKey::Student(nim.clone());
        if env.storage().instance().has(&key) {
            panic!("student already exists");
        }

        let student = Mahasiswa {
            nim: nim.clone(),
            nama,
            jurusan,
            angkatan,
            aktif,
        };

        env.storage().instance().set(&key, &student);

        let mut list = get_student_list(&env);
        list.push_back(nim);
        save_student_list(&env, &list);
    }

    pub fn update_student(
        env: Env,
        nim: String,
        nama: String,
        jurusan: String,
        angkatan: u32,
        aktif: bool,
    ) {
        require_admin(&env);

        let key = DataKey::Student(nim.clone());
        if !env.storage().instance().has(&key) {
            panic!("student not found");
        }

        let student = Mahasiswa {
            nim,
            nama,
            jurusan,
            angkatan,
            aktif,
        };

        env.storage().instance().set(&key, &student);
    }

    pub fn get_student(env: Env, nim: String) -> Mahasiswa {
        let key = DataKey::Student(nim);
        env.storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("student not found"))
    }

    pub fn remove_student(env: Env, nim: String) {
        require_admin(&env);

        let key = DataKey::Student(nim.clone());
        if !env.storage().instance().has(&key) {
            panic!("student not found");
        }

        env.storage().instance().remove(&key);

        let old_list = get_student_list(&env);
        let mut new_list = Vec::new(&env);

        for item in old_list.iter() {
            if item != nim {
                new_list.push_back(item);
            }
        }

        save_student_list(&env, &new_list);
    }

    pub fn list_students(env: Env) -> Vec<String> {
        get_student_list(&env)
    }

    pub fn ping(_env: Env) -> Symbol {
        symbol_short!("pong")
    }
}