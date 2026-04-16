#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::{Address, Env};

    #[test]
    fn test_create_and_get_quests() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, NebulaQuestContract);
        let client = NebulaQuestContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);

        let title = String::from_str(&env, "First Quest");
        let description = String::from_str(&env, "Complete the setup");
        let priority = Priority::High;

        client.create_quest(&owner, &title, &description, &priority);
        
        let quests = client.get_all_quests(&owner);
        assert_eq!(quests.len(), 1);
        
        let quest = quests.get(0).unwrap();
        assert_eq!(quest.title, title);
        assert_eq!(quest.description, description);
        assert_eq!(quest.status, QuestStatus::Todo);
        assert_eq!(quest.priority, priority);
    }

    #[test]
    fn test_update_status() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, NebulaQuestContract);
        let client = NebulaQuestContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);

        let quest_id = client.create_quest(
            &owner, 
            &String::from_str(&env, "Test Quest"), 
            &String::from_str(&env, "Desc"), 
            &Priority::Medium
        );

        let updated = client.update_quest_status(&owner, &quest_id, &QuestStatus::InProgress);
        assert!(updated);

        let quests = client.get_all_quests(&owner);
        assert_eq!(quests.get(0).unwrap().status, QuestStatus::InProgress);
    }

    #[test]
    fn test_delete_quest() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, NebulaQuestContract);
        let client = NebulaQuestContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);

        let quest_id = client.create_quest(
            &owner, 
            &String::from_str(&env, "To Delete"), 
            &String::from_str(&env, "Desc"), 
            &Priority::Low
        );

        assert_eq!(client.get_all_quests(&owner).len(), 1);

        let deleted = client.delete_quest(&owner, &quest_id);
        assert!(deleted);

        assert_eq!(client.get_all_quests(&owner).len(), 0);
    }
}
