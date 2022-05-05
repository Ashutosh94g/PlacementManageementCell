use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[derive(Iden)]
        pub enum Personal {
            Table,
            Id,
            Firstname,
            Lastname,
            Email,
            Phone,
            Dob,
            GenderId,
            CategoryId,
            UGQualificationId,
            UGSpecializationId,
            UGStartyear,
            UGEndyear,
            UGCgpa,
        }

        manager
            .create_table(
                Table::create()
                    .table(Personal::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Personal::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Personal::Firstname).string().not_null())
                    .col(ColumnDef::new(Personal::Lastname).string().not_null())
                    .col(
                        ColumnDef::new(Personal::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Personal::Phone)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Personal::Dob).date().not_null())
                    .col(ColumnDef::new(Personal::GenderId).integer().not_null())
                    .col(ColumnDef::new(Personal::CategoryId).integer().not_null())
                    .col(
                        ColumnDef::new(Personal::UGQualificationId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Personal::UGSpecializationId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Personal::UGStartyear).integer().not_null())
                    .col(ColumnDef::new(Personal::UGEndyear).integer().not_null())
                    .col(ColumnDef::new(Personal::UGCgpa).float().not_null())
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Family {
            Table,
            Id,
            FatherName,
            MotherName,
            FatherOccupationId,
            MotherOccupationId,
            Address,
            City,
            State,
            Zip,
            TenthYear,
            TenthPercentage,
            TenthBoardId,
            TwelfthYear,
            TwelfthPercentage,
            TwelfthBoardId,
        }

        manager
            .create_table(
                Table::create()
                    .table(Family::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Family::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Family::FatherName).string().not_null())
                    .col(ColumnDef::new(Family::MotherName).string().not_null())
                    .col(
                        ColumnDef::new(Family::FatherOccupationId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Family::MotherOccupationId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Family::Address).string().not_null())
                    .col(ColumnDef::new(Family::City).string().not_null())
                    .col(ColumnDef::new(Family::State).string().not_null())
                    .col(ColumnDef::new(Family::Zip).string().not_null())
                    .col(ColumnDef::new(Family::TenthYear).integer().not_null())
                    .col(ColumnDef::new(Family::TenthPercentage).float().not_null())
                    .col(ColumnDef::new(Family::TenthBoardId).integer().not_null())
                    .col(ColumnDef::new(Family::TwelfthYear).integer().not_null())
                    .col(ColumnDef::new(Family::TwelfthPercentage).float().not_null())
                    .col(ColumnDef::new(Family::TwelfthBoardId).integer().not_null())
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Pg {
            Table,
            Id,
            QualificationId,
            SpecializationId,
            Startyear,
            Endyear,
            Cgpa,
        }

        manager
            .create_table(
                Table::create()
                    .table(Pg::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pg::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Pg::QualificationId).integer().not_null())
                    .col(ColumnDef::new(Pg::SpecializationId).integer().not_null())
                    .col(ColumnDef::new(Pg::Startyear).integer().not_null())
                    .col(ColumnDef::new(Pg::Endyear).integer().not_null())
                    .col(ColumnDef::new(Pg::Cgpa).float().not_null())
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Student {
            Table,
            Id,
            PersonalId,
            FamilyId,
            PgId,
        }

        manager
            .create_table(
                Table::create()
                    .table(Student::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Student::Id)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Student::PersonalId).integer().not_null())
                    .col(ColumnDef::new(Student::FamilyId).integer().not_null())
                    .col(ColumnDef::new(Student::PgId).integer())
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Gender {
            Table,
            Id,
            Value,
        }

        manager
            .create_table(
                Table::create()
                    .table(Gender::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Gender::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Gender::Value)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Category {
            Table,
            Id,
            Value,
        }

        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Category::Value)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Qualification {
            Table,
            Id,
            Value,
        }

        manager
            .create_table(
                Table::create()
                    .table(Qualification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Qualification::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Qualification::Value)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Specialization {
            Table,
            Id,
            Value,
        }

        manager
            .create_table(
                Table::create()
                    .table(Specialization::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Specialization::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Specialization::Value)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum FatherOccupation {
            Table,
            Id,
            Value,
        }

        manager
            .create_table(
                Table::create()
                    .table(FatherOccupation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FatherOccupation::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(FatherOccupation::Value)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum MotherOccupation {
            Table,
            Id,
            Value,
        }

        manager
            .create_table(
                Table::create()
                    .table(MotherOccupation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MotherOccupation::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(MotherOccupation::Value)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Board {
            Table,
            Id,
            Value,
        }

        manager
            .create_table(
                Table::create()
                    .table(Board::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Board::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Board::Value)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Teacher {
            Table,
            Id,
            Email,
            Password,
        }

        manager
            .create_table(
                Table::create()
                    .table(Teacher::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teacher::Id)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Teacher::Email)
                            .unique_key()
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Teacher::Password).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_personal_gender_id")
                    .from(Personal::Table, Personal::GenderId)
                    .to(Gender::Table, Gender::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_personal_category")
                    .from(Personal::Table, Personal::CategoryId)
                    .to(Category::Table, Category::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_personal_qualification")
                    .from(Personal::Table, Personal::UGQualificationId)
                    .to(Qualification::Table, Qualification::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_personal_specialization")
                    .from(Personal::Table, Personal::UGSpecializationId)
                    .to(Specialization::Table, Specialization::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_family_father_occupation")
                    .from(Family::Table, Family::FatherOccupationId)
                    .to(FatherOccupation::Table, FatherOccupation::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_family_mother_occupation")
                    .from(Family::Table, Family::MotherOccupationId)
                    .to(MotherOccupation::Table, MotherOccupation::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_family_tenth_board")
                    .from(Family::Table, Family::TenthBoardId)
                    .to(Board::Table, Board::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_family_twelfth_board")
                    .from(Family::Table, Family::TwelfthBoardId)
                    .to(Board::Table, Board::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_pg_specialization")
                    .from(Pg::Table, Pg::SpecializationId)
                    .to(Specialization::Table, Specialization::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_pg_qualification")
                    .from(Pg::Table, Pg::QualificationId)
                    .to(Qualification::Table, Qualification::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_student_personal")
                    .from(Student::Table, Student::PersonalId)
                    .to(Personal::Table, Personal::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_student_family")
                    .from(Student::Table, Student::FamilyId)
                    .to(Family::Table, Family::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_student_pg")
                    .from(Student::Table, Student::PgId)
                    .to(Pg::Table, Pg::Id)
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[derive(Iden)]
        pub enum Personal {
            Table,
        }

        // drop_table(sea_query::Table::drop().table(Students::Table).to_owned())

        manager
            .drop_table(sea_query::Table::drop().table(Personal::Table).to_owned())
            .await?;

        #[derive(Iden)]
        pub enum Family {
            Table,
        }

        manager
            .drop_table(sea_query::Table::drop().table(Family::Table).to_owned())
            .await?;

        #[derive(Iden)]
        pub enum Pg {
            Table,
        }

        manager
            .drop_table(sea_query::Table::drop().table(Pg::Table).to_owned())
            .await?;

        #[derive(Iden)]
        pub enum Student {
            Table,
        }

        manager
            .drop_table(sea_query::Table::drop().table(Student::Table).to_owned())
            .await?;

        #[derive(Iden)]
        pub enum Gender {
            Table,
        }

        manager
            .drop_table(sea_query::Table::drop().table(Gender::Table).to_owned())
            .await?;

        #[derive(Iden)]
        pub enum Category {
            Table,
        }

        manager
            .drop_table(sea_query::Table::drop().table(Category::Table).to_owned())
            .await?;

        #[derive(Iden)]
        pub enum Qualification {
            Table,
        }

        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(Qualification::Table)
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Specialization {
            Table,
        }

        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(Specialization::Table)
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum FatherOccupation {
            Table,
        }

        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(FatherOccupation::Table)
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum MotherOccupation {
            Table,
        }

        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(MotherOccupation::Table)
                    .to_owned(),
            )
            .await?;

        #[derive(Iden)]
        pub enum Board {
            Table,
        }

        manager
            .drop_table(sea_query::Table::drop().table(Board::Table).to_owned())
            .await?;

        Ok(())
    }
}
