use async_trait::async_trait;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use crate::domain::entities::{Relation, RelationStatus, User};
use crate::domain::requests::{CreateRelationPayload, GetRelationsParams, UserAuthPayload};
use crate::domain::errors::ErrorCode;
use crate::traits::{RelationRepositoryTrait, UserRepositoryTrait};

// Mock Repository for Testing
// 展示如何使用泛型約束進行測試
#[derive(Clone)]
pub struct MockRelationRepository {
    relations: Arc<Mutex<std::collections::HashMap<Uuid, Relation>>>,
}

impl MockRelationRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            relations: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    #[allow(dead_code)]
    pub fn with_relations(self, relations: Vec<Relation>) -> Self {
        let mut map = self.relations.lock().unwrap();
        for relation in relations {
            map.insert(relation.id, relation);
        }
        drop(map);
        self
    }
}

#[async_trait]
impl RelationRepositoryTrait for MockRelationRepository {
    async fn find_by_video_id(&self, params: &GetRelationsParams) -> Result<Vec<Relation>, u16> {
        let map = self.relations.lock().unwrap();
        let relations: Vec<Relation> = map
            .values()
            .filter(|r| r.source_video_id == params.video_id)
            .cloned()
            .collect();

        Ok(relations)
    }

    async fn create(&self, payload: &CreateRelationPayload) -> Result<Relation, u16> {
        let relation = Relation {
            id: Uuid::new_v4(),
            source_video_id: payload.source_video_id.clone(),
            source_start_time: payload.source_start_time,
            source_end_time: payload.source_end_time,
            target_video_id: payload.target_video_id.clone(),
            target_start_time: payload.target_start_time,
            target_end_time: payload.target_end_time,
            relation_type: payload.relation_type.clone(),
            user_id: payload.user_id,
            status: RelationStatus::Pending,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let mut map = self.relations.lock().unwrap();
        map.insert(relation.id, relation.clone());

        Ok(relation)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Relation>, u16> {
        let map = self.relations.lock().unwrap();
        Ok(map.get(&id).cloned())
    }

    async fn update_status(&self, id: Uuid, status: &RelationStatus) -> Result<Relation, u16> {
        let mut map = self.relations.lock().unwrap();

        if let Some(relation) = map.get_mut(&id) {
            relation.status = status.clone();
            relation.updated_at = chrono::Utc::now();
            Ok(relation.clone())
        } else {
            Err(ErrorCode::RelationNotFound.as_u16())
        }
    }
}

// Mock User Repository
#[derive(Clone)]
pub struct MockUserRepository {
    users: Arc<Mutex<std::collections::HashMap<Uuid, User>>>,
}

impl MockUserRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    #[allow(dead_code)]
    pub fn with_users(self, users: Vec<User>) -> Self {
        let mut map = self.users.lock().unwrap();
        for user in users {
            map.insert(user.id, user);
        }
        drop(map);
        self
    }
}

#[async_trait]
impl UserRepositoryTrait for MockUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, u16> {
        let map = self.users.lock().unwrap();
        Ok(map.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, u16> {
        let map = self.users.lock().unwrap();
        let user = map.values().find(|u| u.email == email).cloned();
        Ok(user)
    }

    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, u16> {
        let map = self.users.lock().unwrap();
        let user = map.values().find(|u| u.google_id.as_deref() == Some(google_id)).cloned();
        Ok(user)
    }

    async fn create(&self, payload: &UserAuthPayload) -> Result<User, u16> {
        let user = User {
            id: Uuid::new_v4(),
            google_id: payload.google_id.clone(),
            email: payload.email.clone(),
            name: payload.name.clone(),
            avatar_url: payload.avatar_url.clone(),
            reputation: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let mut map = self.users.lock().unwrap();
        map.insert(user.id, user.clone());

        Ok(user)
    }

    async fn update(&self, id: Uuid, payload: &UserAuthPayload) -> Result<User, u16> {
        let mut map = self.users.lock().unwrap();

        if let Some(user) = map.get_mut(&id) {
            user.google_id = payload.google_id.clone();
            user.email = payload.email.clone();
            user.name = payload.name.clone();
            user.avatar_url = payload.avatar_url.clone();
            user.updated_at = chrono::Utc::now();
            Ok(user.clone())
        } else {
            Err(ErrorCode::UserNotFound.as_u16())
        }
    }

    async fn update_reputation(&self, id: Uuid, reputation: i32) -> Result<User, u16> {
        let mut map = self.users.lock().unwrap();

        if let Some(user) = map.get_mut(&id) {
            user.reputation = reputation;
            user.updated_at = chrono::Utc::now();
            Ok(user.clone())
        } else {
            Err(ErrorCode::UserNotFound.as_u16())
        }
    }
}
