use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AgentSkills::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AgentSkills::AgentId).uuid().not_null())
                    .col(ColumnDef::new(AgentSkills::SkillId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-agent_skills")
                            .col(AgentSkills::AgentId)
                            .col(AgentSkills::SkillId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-agent_skills-agent_id")
                            .from(AgentSkills::Table, AgentSkills::AgentId)
                            .to(Agents::Table, Agents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-agent_skills-skill_id")
                            .from(AgentSkills::Table, AgentSkills::SkillId)
                            .to(Skills::Table, Skills::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AgentSkills::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum AgentSkills {
    Table,
    AgentId,
    SkillId,
}

#[derive(DeriveIden)]
enum Agents {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Skills {
    Table,
    Id,
}
