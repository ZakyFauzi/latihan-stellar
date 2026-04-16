#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuestStatus {
    Todo = 0,
    InProgress = 1,
    Completed = 2,
    Failed = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Quest {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: QuestStatus,
    pub priority: Priority,
    pub created_at: u64,
}

#[contract]
pub struct NebulaQuestContract;

#[contractimpl]
impl NebulaQuestContract {
    /// Create a new quest for the given address.
    /// Requires authorization from the owner address.
    pub fn create_quest(
        env: Env,
        owner: Address,
        title: String,
        description: String,
        priority: Priority
    ) -> u64 {
        owner.require_auth();

        let mut quests = Self::get_all_quests(env.clone(), owner.clone());
        
        // Generate a pseudo-random ID for the quest
        let id = env.prng().gen::<u64>();
        
        let quest = Quest {
            id,
            title,
            description,
            status: QuestStatus::Todo,
            priority,
            created_at: env.ledger().timestamp(),
        };

        quests.push_back(quest);

        // Store quests in persistent storage indexed by the owner
        let key = (symbol_short!("QUESTS"), owner);
        env.storage().persistent().set(&key, &quests);

        id
    }

    /// Retrieve all quests for a specific owner.
    pub fn get_all_quests(env: Env, owner: Address) -> Vec<Quest> {
        let key = (symbol_short!("QUESTS"), owner);
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }

    /// Update the status of a quest.
    /// Requires authorization from the owner address.
    pub fn update_quest_status(
        env: Env,
        owner: Address,
        quest_id: u64,
        status: QuestStatus
    ) -> bool {
        owner.require_auth();

        let mut quests = Self::get_all_quests(env.clone(), owner.clone());
        let mut found = false;

        for i in 0..quests.len() {
            let mut quest = quests.get(i).unwrap();
            if quest.id == quest_id {
                quest.status = status;
                quests.set(i, quest);
                found = true;
                break;
            }
        }

        if found {
            let key = (symbol_short!("QUESTS"), owner);
            env.storage().persistent().set(&key, &quests);
        }

        found
    }

    /// Delete a quest.
    /// Requires authorization from the owner address.
    pub fn delete_quest(env: Env, owner: Address, quest_id: u64) -> bool {
        owner.require_auth();

        let mut quests = Self::get_all_quests(env.clone(), owner.clone());
        let mut found_index: Option<u32> = None;

        for i in 0..quests.len() {
            if quests.get(i).unwrap().id == quest_id {
                found_index = Some(i);
                break;
            }
        }

        if let Some(index) = found_index {
            quests.remove(index);
            let key = (symbol_short!("QUESTS"), owner);
            env.storage().persistent().set(&key, &quests);
            return true;
        }

        false
    }
}

mod test;
