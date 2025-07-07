use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{Relation, CreateRelationPayload, GetRelationsParams, RelationStatus, User, UserAuthPayload};
use crate::traits::{RelationRepositoryTrait, UserRepositoryTrait};
use sqlx::Error;

// Mock Repository for Testing
// 展示如何使用泛型約束進行測試
pub struct MockRelationRepository {
    relations: std::collections::HashMap<Uuid, Relation>,
}

impl MockRelationRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            relations: std::collections::HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_relations(mut self, relations: Vec<Relation>) -> Self {
        for relation in relations {
            self.relations.insert(relation.id, relation);
        }
        self
    }
}

#[async_trait]
impl RelationRepositoryTrait for MockRelationRepository {
    async fn find_by_video_id(&self, params: &GetRelationsParams) -> Result<Vec<Relation>, Error> {
        let relations: Vec<Relation> = self.relations
            .values()
            .filter(|r| r.source_video_id == params.video_id)
            .cloned()
            .collect();
        
        Ok(relations)
    }

    async fn create(&self, payload: &CreateRelationPayload) -> Result<Relation, Error> {
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
        
        Ok(relation)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Relation>, Error> {
        Ok(self.relations.get(&id).cloned())
    }

    async fn update_status(&self, id: Uuid, status: &RelationStatus) -> Result<Relation, Error> {
        if let Some(mut relation) = self.relations.get(&id).cloned() {
            relation.status = status.clone();
            relation.updated_at = chrono::Utc::now();
            Ok(relation)
        } else {
            Err(Error::RowNotFound)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        Ok(self.relations.contains_key(&id))
    }
}

// Mock User Repository
pub struct MockUserRepository {
    users: std::collections::HashMap<Uuid, User>,
}

impl MockUserRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            users: std::collections::HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_users(mut self, users: Vec<User>) -> Self {
        for user in users {
            self.users.insert(user.id, user);
        }
        self
    }
}

#[async_trait]
impl UserRepositoryTrait for MockUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        Ok(self.users.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = self.users.values().find(|u| u.email == email).cloned();
        Ok(user)
    }

    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, Error> {
        let user = self.users.values().find(|u| u.google_id.as_deref() == Some(google_id)).cloned();
        Ok(user)
    }

    async fn create(&self, payload: &UserAuthPayload) -> Result<User, Error> {
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
        
        Ok(user)
    }

    async fn update(&self, id: Uuid, payload: &UserAuthPayload) -> Result<User, Error> {
        if let Some(mut user) = self.users.get(&id).cloned() {
            user.google_id = payload.google_id.clone();
            user.email = payload.email.clone();
            user.name = payload.name.clone();
            user.avatar_url = payload.avatar_url.clone();
            user.updated_at = chrono::Utc::now();
            Ok(user)
        } else {
            Err(Error::RowNotFound)
        }
    }

    async fn update_reputation(&self, id: Uuid, reputation: i32) -> Result<User, Error> {
        if let Some(mut user) = self.users.get(&id).cloned() {
            user.reputation = reputation;
            user.updated_at = chrono::Utc::now();
            Ok(user)
        } else {
            Err(Error::RowNotFound)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        Ok(self.users.contains_key(&id))
    }
}

// 為 Mock 實作 Clone trait
impl Clone for MockRelationRepository {
    fn clone(&self) -> Self {
        Self {
            relations: self.relations.clone(),
        }
    }
}

impl Clone for MockUserRepository {
    fn clone(&self) -> Self {
        Self {
            users: self.users.clone(),
        }
    }
} 